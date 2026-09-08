//! Synthetic adapters reuse the accepted local Resend HTTP fixture and SecretStore.
use super::super::{common::*, fireflies::Reader, services::Services, types::*};
use crate::{
    business_identity::{self as identity, store, types::*, Principal},
    db::{test_helpers::fresh_in_memory_db, AppDatabase},
    ops::email::SecretStore,
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::{json, Value};
use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

#[derive(Default)]
pub(super) struct Secrets {
    pub values: Mutex<HashMap<String, String>>,
    pub fail_set: AtomicBool,
    pub fail_delete: AtomicBool,
}
impl SecretStore for Secrets {
    fn get(&self, key: &str) -> Option<String> {
        self.values.lock().unwrap().get(key).cloned()
    }
    fn set(&self, key: &str, value: &str) -> std::result::Result<(), ()> {
        if self.fail_set.load(Ordering::SeqCst) {
            return Err(());
        }
        self.values.lock().unwrap().insert(key.into(), value.into());
        Ok(())
    }
    fn delete(&self, key: &str) -> std::result::Result<(), ()> {
        if self.fail_delete.load(Ordering::SeqCst) {
            return Err(());
        }
        self.values.lock().unwrap().remove(key);
        Ok(())
    }
}
pub(super) struct Reply {
    pub status: u16,
    pub body: String,
    pub headers: HeaderMap,
    pub gate: Option<Arc<tokio::sync::Barrier>>,
}
impl Reply {
    pub fn json(value: Value) -> Self {
        Self {
            status: 200,
            body: value.to_string(),
            headers: HeaderMap::new(),
            gate: None,
        }
    }
}
#[derive(Clone, Default)]
struct MockState {
    replies: Arc<Mutex<VecDeque<Reply>>>,
    seen: Arc<Mutex<Vec<Value>>>,
}
pub(super) struct Mock {
    pub address: std::net::SocketAddr,
    state: MockState,
    task: tokio::task::JoinHandle<()>,
}
impl Mock {
    pub async fn start() -> Self {
        async fn handle(
            State(state): State<MockState>,
            headers: HeaderMap,
            Json(body): Json<Value>,
        ) -> (StatusCode, HeaderMap, String) {
            assert!(headers
                .get("authorization")
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("Bearer synthetic-"));
            state.seen.lock().unwrap().push(body);
            let reply = state
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected synthetic upstream read");
            if let Some(gate) = reply.gate {
                gate.wait().await;
            }
            (
                StatusCode::from_u16(reply.status).unwrap(),
                reply.headers,
                reply.body,
            )
        }
        let state = MockState::default();
        let router = Router::new()
            .route("/graphql", post(handle))
            .with_state(state.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        Self {
            address,
            state,
            task,
        }
    }
    pub fn reply(&self, reply: Reply) {
        self.state.replies.lock().unwrap().push_back(reply);
    }
    pub fn json(&self, value: Value) {
        self.reply(Reply::json(value));
    }
    pub fn count(&self) -> usize {
        self.state.seen.lock().unwrap().len()
    }
    pub fn requests(&self) -> Vec<Value> {
        self.state.seen.lock().unwrap().clone()
    }
    pub async fn seen(&self, n: usize) {
        tokio::time::timeout(Duration::from_secs(3), async {
            while self.count() < n {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
    }
}
impl Drop for Mock {
    fn drop(&mut self) {
        self.task.abort();
    }
}
pub(super) struct Fixture {
    pub db: AppDatabase,
    pub op: Principal,
    pub services: Arc<Services>,
    pub secrets: Arc<Secrets>,
    pub mock: Mock,
}
impl Fixture {
    pub async fn new() -> Self {
        let db = fresh_in_memory_db().await;
        store::bootstrap(
            &db.conn,
            BootstrapInput {
                organization_name: "Synthetic source team".into(),
                owner_name: "Source operator".into(),
            },
        )
        .await
        .unwrap();
        let op = identity::operator_principal(&db.conn).await.unwrap();
        let mock = Mock::start().await;
        let secrets = Arc::new(Secrets::default());
        let services = Services::fixture(
            Reader::fixture(mock.address, Duration::from_secs(1)).unwrap(),
            secrets.clone(),
        );
        Self {
            db,
            op,
            services,
            secrets,
            mock,
        }
    }
    pub async fn binding(&self) -> BindingAdmin {
        self.mock
            .json(json!({"data":{"user":{"user_id":"source-user"}}}));
        super::super::bindings_create(
            &self.db.conn,
            &self.op,
            &self.services,
            None,
            self.create_input(&id()),
        )
        .await
        .unwrap()
    }
    pub fn create_input(&self, operation_id: &str) -> CreateBindingInput {
        serde_json::from_value(self.create_json(operation_id)).unwrap()
    }
    pub fn create_json(&self, operation_id: &str) -> Value {
        json!({"operationId":operation_id,"label":"Reviewed meetings","domain":"feedback","sourceOwnerId":self.op.member_id(),"source":{"kind":"fireflies","apiKey":"synthetic-fireflies"},"publicationDomains":["feedback"],"retainedTaskText":true})
    }
    pub async fn allowed(&self) -> BindingAdmin {
        let b = self.binding().await;
        let result = super::super::grants_upsert(
            &self.db.conn,
            &self.op,
            &self.services,
            grant(&b, self.op.member_id(), None),
        )
        .await
        .unwrap();
        super::super::bindings_update(
            &self.db.conn,
            &self.op,
            &self.services,
            enable(&result.binding),
        )
        .await
        .unwrap()
    }
}
pub(super) fn grant(b: &BindingAdmin, member: &str, revision: Option<i64>) -> UpsertGrantInput {
    UpsertGrantInput {
        operation_id: id(),
        binding_id: b.id.clone(),
        expected_binding_revision: b.revision,
        member_id: member.into(),
        expected_grant_revision: revision,
        scope: GrantScope::BindingCurrentAndFutureSources,
        read: true,
        import: true,
        triage: true,
        publication_domains: vec![Domain::Feedback],
        expires_at: None,
    }
}
pub(super) fn enable(b: &BindingAdmin) -> UpdateBindingInput {
    UpdateBindingInput {
        operation_id: id(),
        binding_id: b.id.clone(),
        expected_revision: b.revision,
        label: b.label.clone(),
        enabled: true,
        publication_domains: b.publication_domains.clone(),
        retained_task_text: b.retained_task_text,
        credential: None,
    }
}
pub(super) async fn human(
    conn: &DatabaseConnection,
    op: &Principal,
    role: Role,
    domains: Vec<Domain>,
) -> (Member, IssuedCredential, Principal) {
    let member = store::create_member(
        conn,
        op,
        CreateMemberInput {
            organization_id: op.organization_id().into(),
            display_name: "Synthetic colleague".into(),
            kind: MemberKind::Human,
            role,
            domains,
        },
    )
    .await
    .unwrap();
    let issued = store::issue_credential(
        conn,
        op,
        IssueCredentialInput {
            organization_id: op.organization_id().into(),
            member_id: member.id.clone(),
            label: "Synthetic session".into(),
        },
    )
    .await
    .unwrap();
    let p = store::resolve_credential(conn, &issued.token)
        .await
        .unwrap();
    (member, issued, p)
}
pub(super) async fn count(conn: &DatabaseConnection, table: &str) -> i64 {
    conn.query_one(sql(
        &format!("SELECT count(*) AS count FROM {table}"),
        vec![],
    ))
    .await
    .unwrap()
    .unwrap()
    .try_get("", "count")
    .unwrap()
}
pub(super) fn transcript(id: &str, text: &str) -> Value {
    json!({"data":{"transcript":{"id":id,"title":"Feedback planning","user":{"user_id":"source-user"},"privacy":"private","shared_with":[],"sentences":[{"index":7,"text":text,"start_time":2.5,"end_time":4}],"summary":{"action_items":["Review proposed follow-up."]},"meeting_info":{"summary_status":"provider-specific"}}}})
}
