//! Returned cursors survive revocation without exposing revoked private rows.
use super::{
    common::*,
    session_store as store,
    session_tests::{setup, start},
    types::*,
};
use crate::{business_identity as identity, business_tasks as tasks};
use sea_orm::{ConnectionTrait, DatabaseConnection};

fn page(task_id: &str, cursor: Option<String>, limit: u32) -> SessionListInput {
    SessionListInput {
        task_id: task_id.into(),
        cursor,
        limit,
    }
}
async fn admitted(
    db: &DatabaseConnection,
    op: &identity::Principal,
    task: &tasks::types::Detail,
    profile: &ProfileSummary,
    count: usize,
) -> Vec<String> {
    let mut ids = vec![];
    for _ in 0..count {
        ids.push(
            store::reserve_start(db, op, &start(task, profile))
                .await
                .unwrap()
                .result
                .session
                .id,
        );
    }
    ids.sort_by(|a, b| b.cmp(a));
    ids
}
async fn revoke(db: &DatabaseConnection, session_id: &str, status: &str) {
    db.execute(statement("UPDATE business_execution_session SET status=?,generation=generation+1,revision=revision+1 WHERE id=?",
        vec![status.into(), session_id.into()])).await.unwrap();
}
async fn task(db: &DatabaseConnection, principal: &identity::Principal) -> tasks::types::Detail {
    tasks::store::create(
        db,
        &tasks::ActorContext::authenticated(principal.clone()),
        tasks::types::CreateInput {
            title: "Separate synthetic task".into(),
            notes: String::new(),
            domain: identity::Domain::Feedback,
            priority: tasks::vocabulary::TaskPriority::Normal,
            due_date: None,
            owner_id: None,
            assignee_id: None,
            reviewer_id: None,
        },
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn execution_pagination_skips_leading_and_interleaved_stale_rows() {
    let (db, op, task, profile) = setup().await;
    let ids = admitted(&db, &op, &task, &profile, 5).await;
    let all = store::list(&db, &op, page(&task.task.id, None, 50))
        .await
        .unwrap();
    assert_eq!(
        all.items.iter().map(|s| s.id.clone()).collect::<Vec<_>>(),
        ids
    );
    assert!(all.next_cursor.is_none());
    revoke(&db, &ids[0], "revoked").await;
    revoke(&db, &ids[2], "closed").await;
    let expected = vec![ids[1].clone(), ids[3].clone(), ids[4].clone()];
    let mut cursor = None;
    let mut found = vec![];
    for expected_id in &expected {
        let next = store::list(&db, &op, page(&task.task.id, cursor, 1))
            .await
            .unwrap();
        assert_eq!(next.items.len(), 1);
        assert_eq!(&next.items[0].id, expected_id);
        if let Some(c) = &next.next_cursor {
            assert_eq!(c, expected_id);
            assert_ne!(c, &ids[0]);
            assert_ne!(c, &ids[2]);
        }
        found.push(next.items[0].id.clone());
        cursor = next.next_cursor;
    }
    assert_eq!(found, expected);
    assert!(cursor.is_none());
}

#[tokio::test]
async fn execution_pagination_returned_cursor_remains_usable_after_revocation() {
    let (db, op, task, profile) = setup().await;
    let ids = admitted(&db, &op, &task, &profile, 3).await;
    let first = store::list(&db, &op, page(&task.task.id, None, 1))
        .await
        .unwrap();
    assert_eq!(first.next_cursor.as_deref(), Some(ids[0].as_str()));
    revoke(&db, &ids[0], "revoked").await;
    assert!(matches!(
        store::get(
            &db,
            &op,
            SessionInput {
                session_id: ids[0].clone()
            }
        )
        .await,
        Err(Error(OperationReason::AuthorityChanged))
    ));
    let second = store::list(&db, &op, page(&task.task.id, first.next_cursor, 1))
        .await
        .unwrap();
    assert_eq!(second.items[0].id, ids[1]);
    let last = store::list(&db, &op, page(&task.task.id, second.next_cursor, 1))
        .await
        .unwrap();
    assert_eq!(last.items[0].id, ids[2]);
    assert!(last.next_cursor.is_none());
}

// Retained foreign-row fixture only. This does not enable E1 admission for the
// real member credentials below; the public gate must still reject them.
async fn foreign_cursor(
    db: &DatabaseConnection,
    principal: &identity::Principal,
    task_id: &str,
) -> String {
    let session_id = id();
    let profile_id = id();
    let grant = identity::delegation_grant(principal)
        .unwrap()
        .to_storage()
        .unwrap();
    db.execute(statement("INSERT INTO business_execution_profile(id,organization_id,member_id,client_id,config_key,config_hash,revision,summary_json) VALUES(?,?,?,'pi','retained-synthetic',?,1,'{}')",
        vec![profile_id.clone().into(), principal.organization_id().into(), principal.member_id().into(), "a".repeat(64).into()])).await.unwrap();
    db.execute(statement("INSERT INTO business_execution_session(id,organization_id,task_id,member_id,authority_json,authorization_epoch,task_scope_epoch,profile_id,profile_revision,revision,generation,mode,status,title,created_at,updated_at,last_activity_at) VALUES(?,?,?,?,?,?,1,?,1,1,1,'chat','stopped','Private foreign session','then','then','then')",
        vec![session_id.clone().into(), principal.organization_id().into(), task_id.into(), principal.member_id().into(), grant.into(), principal.authorization_epoch().into(), profile_id.into()])).await.unwrap();
    session_id
}

#[tokio::test]
async fn execution_pagination_rejects_foreign_task_member_and_tenant_cursors() {
    let (db, op, own_task, profile) = setup().await;
    let own = admitted(&db, &op, &own_task, &profile, 2).await;
    let other_task = task(&db, &op).await;
    let wrong_task = admitted(&db, &op, &other_task, &profile, 1).await.remove(0);
    let member = identity::store::create_member(
        &db,
        &op,
        identity::types::CreateMemberInput {
            organization_id: op.organization_id().into(),
            display_name: "Separate human".into(),
            kind: identity::MemberKind::Human,
            role: identity::Role::Member,
            domains: vec![identity::Domain::Feedback],
        },
    )
    .await
    .unwrap();
    let credential = identity::store::issue_credential(
        &db,
        &op,
        identity::types::IssueCredentialInput {
            organization_id: op.organization_id().into(),
            member_id: member.id,
            label: "Synthetic cursor isolation".into(),
        },
    )
    .await
    .unwrap();
    let other_member = identity::store::resolve_credential(&db, &credential.token)
        .await
        .unwrap();
    let wrong_member = foreign_cursor(&db, &other_member, &own_task.task.id).await;
    let tenant = identity::platform::create(
        &db,
        &identity::platform::PlatformContext::from_operator(
            &crate::web::auth::AuthenticatedOperator,
        ),
        identity::platform::CreateTenantInput {
            operation_id: id(),
            organization_name: "Cursor tenant".into(),
            owner_name: "Other owner".into(),
        },
    )
    .await
    .unwrap();
    let tenant_principal = identity::store::resolve_credential(&db, tenant.token.as_ref().unwrap())
        .await
        .unwrap();
    let tenant_task = task(&db, &tenant_principal).await;
    let wrong_tenant = foreign_cursor(&db, &tenant_principal, &tenant_task.task.id).await;
    for cursor in [wrong_task, wrong_member, wrong_tenant, id()] {
        assert!(matches!(
            store::list(&db, &op, page(&own_task.task.id, Some(cursor), 1)).await,
            Err(Error(OperationReason::Missing))
        ));
    }
    for principal in [&other_member, &tenant_principal] {
        assert!(matches!(
            store::list(
                &db,
                principal,
                page(&own_task.task.id, Some(own[0].clone()), 1)
            )
            .await,
            Err(Error(OperationReason::Forbidden))
        ));
    }
    let visible = store::list(&db, &op, page(&own_task.task.id, None, 50))
        .await
        .unwrap();
    assert_eq!(
        visible
            .items
            .iter()
            .map(|s| s.id.clone())
            .collect::<Vec<_>>(),
        own
    );
    assert!(visible.next_cursor.is_none());
}
