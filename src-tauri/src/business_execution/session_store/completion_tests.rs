//! Tampering is possible here only because this test is a child of the private
//! admission module. Production callers cannot construct or mutate Admission.
use super::*;
use crate::business_execution::session_tests::{link, profile, setup, start};

async fn snapshot(db: &DatabaseConnection) -> Vec<Vec<String>> {
    let mut result = vec![];
    for table in [
        "business_execution_session",
        "business_execution_generation",
        "business_execution_operation",
    ] {
        let columns = db
            .query_all(statement(&format!("PRAGMA table_info({table})"), vec![]))
            .await
            .unwrap();
        let names = columns
            .into_iter()
            .map(|row| format!("\"{}\"", row.try_get::<String>("", "name").unwrap()))
            .collect::<Vec<_>>()
            .join(",");
        result.push(
            db.query_all(statement(
                &format!("SELECT json_array({names}) AS value FROM {table} ORDER BY rowid"),
                vec![],
            ))
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.try_get("", "value").unwrap())
            .collect(),
        );
    }
    result
}

#[tokio::test]
async fn execution_admission_rejects_cross_operation_completion_without_mutation() {
    let (db, principal, task, profile) = setup().await;
    let a_input = start(&task, &profile);
    let b_input = start(&task, &profile);
    let mut a = reserve_start(&db, &principal, &a_input)
        .await
        .unwrap()
        .admission
        .unwrap();
    let b = reserve_start(&db, &principal, &b_input)
        .await
        .unwrap()
        .admission
        .unwrap();
    let original = a.operation_id.clone();
    let before = snapshot(&db).await;
    a.operation_id = b.operation_id.clone();
    assert!(matches!(
        complete_launch(&db, &a, &link()).await,
        Err(Error(OperationReason::Conflict))
    ));
    assert_eq!(snapshot(&db).await, before);

    a.operation_id = original;
    let confirmed_a = complete_launch(&db, &a, &link()).await.unwrap();
    assert_eq!(confirmed_a.session.id, a.session_id);
    assert_eq!(confirmed_a.operation.id, a_input.operation_id);
    let pending_b = operation(
        &db,
        &principal,
        OperationInput {
            operation_id: b_input.operation_id.clone(),
            kind: OperationKind::Start,
        },
    )
    .await
    .unwrap();
    assert_eq!(pending_b.operation.status, OperationStatus::Pending);
    assert_eq!(
        pending_b.resource_id.as_deref(),
        Some(b.session_id.as_str())
    );
    let confirmed_b = complete_launch(&db, &b, &link()).await.unwrap();
    assert_eq!(confirmed_b.session.id, b.session_id);
    assert_eq!(confirmed_b.operation.id, b_input.operation_id);
    assert_ne!(confirmed_a.session.id, confirmed_b.session.id);
}

#[tokio::test]
async fn execution_admission_rejects_stale_generation_and_authority_completion() {
    let (db, principal, task, configured) = setup().await;
    let mut admission = reserve_start(&db, &principal, &start(&task, &configured))
        .await
        .unwrap()
        .admission
        .unwrap();
    let before = snapshot(&db).await;
    admission.generation += 1;
    assert!(complete_launch(&db, &admission, &link()).await.is_err());
    assert_eq!(snapshot(&db).await, before);
    admission.generation -= 1;
    assert_eq!(
        complete_launch(&db, &admission, &link())
            .await
            .unwrap()
            .operation
            .status,
        OperationStatus::Confirmed
    );

    let late = reserve_start(&db, &principal, &start(&task, &configured))
        .await
        .unwrap()
        .admission
        .unwrap();
    // Use the actual discovery update and permanent profile fence, not a new
    // Principal or fabricated refreshed epoch, before the late completion.
    sync_profiles(&db, &principal, vec![profile("b")])
        .await
        .unwrap();
    let revoked = snapshot(&db).await;
    assert!(matches!(
        complete_launch(&db, &late, &link()).await,
        Err(Error(OperationReason::AuthorityChanged))
    ));
    assert_eq!(snapshot(&db).await, revoked);
}
