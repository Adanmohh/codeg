use super::{super::*, support::*};
use crate::{
    db::{service::ticket_service as tickets, test_helpers::seed_folder},
    ops::email::SecretStore,
};
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde_json::json;

#[tokio::test]
async fn intake_email_projection_excludes_private_activity_and_fences_config_away_and_back() {
    let f = Fixture::new().await;
    let (op, key) = crate::ops::tests::seed(&f.db, 17).await;
    f.db.conn.execute(common::sql("INSERT INTO ops_email_config(inbox_id,account_id,credential_ref) VALUES(?,17,'synthetic-email-ref')",vec![key.inbox_id.into()])).await.unwrap();
    f.secrets
        .set("synthetic-email-ref", "synthetic-existing")
        .unwrap();
    let scope = tickets::Scope {
        account_id: 17,
        inbox_id: key.inbox_id,
    };
    let messages = tickets::list_messages(
        &f.db.conn,
        scope,
        key.conversation_id,
        tickets::MessageView::Public,
    )
    .await
    .unwrap();
    let reference = LegacyRef::Email {
        conversation_id: key.conversation_id,
        message_id: messages[0].id,
    };
    let note = tickets::add_private_note(
        &f.db.conn,
        scope,
        key.conversation_id,
        "operator:fixture",
        "Private note must stay private",
    )
    .await
    .unwrap();
    let tx = f.db.conn.begin().await.unwrap();
    let resource = legacy::resolve(
        &tx,
        &op,
        &SourceSetup::Email {
            inbox_id: key.inbox_id,
        },
    )
    .await
    .unwrap();
    let projection = legacy::project(&tx, &resource, &reference, &f.services)
        .await
        .unwrap();
    assert_eq!(projection.passages[0].text, "Please help");
    assert!(legacy::project(
        &tx,
        &resource,
        &LegacyRef::Email {
            conversation_id: key.conversation_id,
            message_id: note.id
        },
        &f.services
    )
    .await
    .is_err());
    let wrong_op = crate::ops::tests::operator(18);
    assert!(legacy::resolve(
        &tx,
        &wrong_op,
        &SourceSetup::Email {
            inbox_id: key.inbox_id
        }
    )
    .await
    .is_err());
    tx.rollback().await.unwrap();
    f.db.conn
        .execute(common::sql(
            "UPDATE ops_ticket_message SET message_type=2 WHERE id=?",
            vec![messages[0].id.into()],
        ))
        .await
        .unwrap();
    let tx = f.db.conn.begin().await.unwrap();
    assert!(legacy::project(&tx, &resource, &reference, &f.services)
        .await
        .is_err());
    tx.rollback().await.unwrap();
    for address in ["changed@example.test", "support@example.com"] {
        f.db.conn
            .execute(common::sql(
                "UPDATE ops_ticket_inbox SET email_address=? WHERE id=?",
                vec![address.into(), key.inbox_id.into()],
            ))
            .await
            .unwrap();
    }
    let tx = f.db.conn.begin().await.unwrap();
    assert!(legacy::check(&tx, &resource, &f.services).await.is_err());
    let current = legacy::resolve(
        &tx,
        &op,
        &SourceSetup::Email {
            inbox_id: key.inbox_id,
        },
    )
    .await
    .unwrap();
    assert_ne!(current, resource);
    tx.rollback().await.unwrap();
    legacy::fence_email(&f.db.conn, key.inbox_id, true)
        .await
        .unwrap();
    let tx = f.db.conn.begin().await.unwrap();
    assert!(legacy::resolve(
        &tx,
        &op,
        &SourceSetup::Email {
            inbox_id: key.inbox_id
        }
    )
    .await
    .is_err());
    tx.rollback().await.unwrap();
    legacy::fence_email(&f.db.conn, key.inbox_id, false)
        .await
        .unwrap();
    let tx = f.db.conn.begin().await.unwrap();
    assert!(legacy::check(&tx, &current, &f.services).await.is_err());
    assert_eq!(f.mock.count(), 0);
}

#[tokio::test]
async fn intake_hafidh_projection_is_cached_public_only_and_fenced_by_host_and_folder() {
    let f = Fixture::new().await;
    let folder = seed_folder(&f.db, "/synthetic/intake-projection").await;
    let product = "synthetic-hafidh";
    let ulid = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
    let binding = crate::ops_intake::RepositoryBinding {
        product_id: product.into(),
        folder_id: folder,
        app_id: "fixture-app".into(),
        installation_id: 7,
        repository_id: 11,
        full_name: "owner/repo".into(),
        enabled: true,
    };
    crate::ops_intake::configure_repository(&f.db.conn, &binding)
        .await
        .unwrap();
    let config = json!({"binding":binding,"origin":"https://synthetic.invalid","bearer_ref":"synthetic-hafidh-ref","key_ref":null});
    f.db.conn
        .execute(common::sql(
            "INSERT INTO ops_intake_host_product(product_id,account_id,config_json) VALUES(?,17,?)",
            vec![product.into(), config.to_string().into()],
        ))
        .await
        .unwrap();
    f.secrets
        .set("synthetic-hafidh-ref", "synthetic-existing")
        .unwrap();
    let record = json!({"schema_version":1,"source_ref":{"product_id":product,"source":"testflight","ulid":ulid,"external_id":"PRIVATE-REPORTER"},"source_revision":"a".repeat(64),"fetched_at":common::now(),"title":"Public feedback title","title_is_draft":true,"description":"Public feedback body","feedback_type":null,"source_status":"pending","submitted_at":null,"source_updated_at":common::now(),"device":"PRIVATE-DEVICE","os_version":null,"app_version":null,"build_number":null,"platform":null,"locale":null,"screenshots":[],"triage":{"seeded_tags":[],"seeded_severity":"medium","source_tags":[],"source_severity":"medium","confirmed_tags":null,"confirmed_severity":null},"evidence_candidates":[],"missing_required":["build","screen","reciter","log"]});
    let observed = chrono::Utc::now().timestamp();
    f.db.conn.execute(common::sql("INSERT INTO ops_intake_host_snapshot(product_id,ulid,record_json,verified_at,error) VALUES(?,?,?,?,NULL)",vec![product.into(),ulid.into(),record.to_string().into(),observed.into()])).await.unwrap();
    let op = crate::ops::tests::operator(17);
    let tx = f.db.conn.begin().await.unwrap();
    let resource = legacy::resolve(
        &tx,
        &op,
        &SourceSetup::HafidhTestflight {
            product_id: product.into(),
        },
    )
    .await
    .unwrap();
    let reference = LegacyRef::HafidhTestflight { ulid: ulid.into() };
    let public = legacy::project(&tx, &resource, &reference, &f.services)
        .await
        .unwrap();
    assert_eq!(public.title, "Public feedback title");
    assert_eq!(public.passages[0].text, "Public feedback body");
    assert!(!common::json(&public.passages).unwrap().contains("PRIVATE"));
    tx.rollback().await.unwrap();
    let actual: i64 =
        f.db.conn
            .query_one(common::sql(
                "SELECT verified_at FROM ops_intake_host_snapshot WHERE product_id=? AND ulid=?",
                vec![product.into(), ulid.into()],
            ))
            .await
            .unwrap()
            .unwrap()
            .try_get("", "verified_at")
            .unwrap();
    assert_eq!(actual, observed);
    f.db.conn
        .execute(common::sql(
            "UPDATE ops_intake_host_snapshot SET verified_at=NULL WHERE product_id=?",
            vec![product.into()],
        ))
        .await
        .unwrap();
    let tx = f.db.conn.begin().await.unwrap();
    assert!(legacy::project(&tx, &resource, &reference, &f.services)
        .await
        .is_err());
    tx.rollback().await.unwrap();
    for deleted in [Some(common::now()), None] {
        f.db.conn
            .execute(common::sql(
                "UPDATE folder SET deleted_at=? WHERE id=?",
                vec![deleted.into(), folder.into()],
            ))
            .await
            .unwrap();
    }
    let tx = f.db.conn.begin().await.unwrap();
    assert!(legacy::check(&tx, &resource, &f.services).await.is_err());
    let rebound = legacy::resolve(
        &tx,
        &op,
        &SourceSetup::HafidhTestflight {
            product_id: product.into(),
        },
    )
    .await
    .unwrap();
    tx.rollback().await.unwrap();
    let mut changed = binding.clone();
    changed.enabled = false;
    crate::ops_intake::configure_repository(&f.db.conn, &changed)
        .await
        .unwrap();
    crate::ops_intake::configure_repository(&f.db.conn, &binding)
        .await
        .unwrap();
    let tx = f.db.conn.begin().await.unwrap();
    assert!(legacy::check(&tx, &rebound, &f.services).await.is_err());
    tx.rollback().await.unwrap();
    assert_eq!(f.mock.count(), 0);
    assert_eq!(count(&f.db.conn, "ops_intake_evidence").await, 0);
}
