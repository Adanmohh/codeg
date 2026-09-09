//! Closed wire contract670af9ca + protected access18be55ed. No caller identity.
use crate::business_identity::Domain;
pub use crate::business_tasks::types::PreparedTask;
use serde::{Deserialize, Serialize};

macro_rules! input {
    ($name:ident { $($fields:tt)* }) => {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct $name { $($fields)* }
    };
}
macro_rules! dto {
    ($name:ident { $($fields:tt)* }) => {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name { $($fields)* }
    };
}
macro_rules! vocabulary {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($variant),+ }
    };
}
vocabulary!(SourceKind {
    Fireflies,
    Email,
    HafidhTestflight
});
vocabulary!(CandidateState {
    Pending,
    Accepted,
    Linked,
    Discarded
});
vocabulary!(CandidateOrigin {
    SourceReview,
    HumanSelection
});
vocabulary!(Disclosure {
    Fresh,
    MetadataOnly
});
vocabulary!(AccessState {
    Unverified,
    Fresh,
    Expired,
    Denied
});
vocabulary!(ContentState {
    Missing,
    Available,
    Unsupported
});
vocabulary!(SummaryState {
    Missing,
    Empty,
    Available,
    Unsupported
});
vocabulary!(PassageKind {
    Sentence,
    Summary,
    EmailMessage,
    Feedback
});
vocabulary!(ImportState {
    Queued,
    Running,
    Waiting,
    Complete,
    Failed,
    Cancelled
});
vocabulary!(Coverage {
    NotStarted,
    Partial,
    BoundedEnd,
    Capped
});
vocabulary!(GrantScope {
    BindingCurrentAndFutureSources
});
vocabulary!(GrantState { Active, Revoked });
vocabulary!(CredentialState { Missing, Present });
vocabulary!(ImportView { Unfinished, All });
impl Default for ImportView {
    fn default() -> Self {
        Self::Unfinished
    }
}
impl Default for CandidateState {
    fn default() -> Self {
        Self::Pending
    }
}

/// Write-only input. Intentionally no Debug/Serialize/Clone.
pub struct Secret(pub(super) String);
impl<'de> Deserialize<'de> for Secret {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        if value.is_empty() || value.len() > 4096 || !value.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(serde::de::Error::custom("Invalid credential"));
        }
        Ok(Self(value))
    }
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SourceSetup {
    Fireflies {
        #[serde(rename = "apiKey")]
        api_key: Secret,
    },
    Email {
        #[serde(rename = "inboxId")]
        inbox_id: i32,
    },
    HafidhTestflight {
        #[serde(rename = "productId")]
        product_id: String,
    },
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CredentialReplacement {
    Fireflies {
        #[serde(rename = "apiKey")]
        api_key: Secret,
    },
}
fn present_nonnull<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}
fn required_nullable<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ResourceIdentity {
    Fireflies {
        #[serde(rename = "providerUserId")]
        provider_user_id: String,
        mine: bool,
    },
    Email {
        #[serde(rename = "accountId")]
        account_id: i32,
        #[serde(rename = "inboxId")]
        inbox_id: i32,
        #[serde(rename = "configurationIdentity")]
        configuration_identity: String,
    },
    HafidhTestflight {
        #[serde(rename = "accountId")]
        account_id: i32,
        #[serde(rename = "productId")]
        product_id: String,
        #[serde(rename = "configurationIdentity")]
        configuration_identity: String,
    },
}
input!(PageInput { #[serde(default)] pub page: u32, });
input!(BindingInput { pub binding_id: String, });
input!(BindingPageInput { pub binding_id: String, #[serde(default)] pub page: u32, });
input!(CreateBindingInput {
    pub operation_id: String, pub label: String, pub domain: Domain, pub source_owner_id: String,
    pub source: SourceSetup, pub publication_domains: Vec<Domain>, pub retained_task_text: bool,
});
input!(UpdateBindingInput {
    pub operation_id: String, pub binding_id: String, pub expected_revision: i64,
    pub label: String, pub enabled: bool, pub publication_domains: Vec<Domain>, pub retained_task_text: bool,
    #[serde(default, deserialize_with = "present_nonnull")] pub credential: Option<CredentialReplacement>,
});
input!(DisableBindingInput { pub operation_id: String, pub binding_id: String, pub expected_revision: i64, });
input!(UpsertGrantInput {
    pub operation_id: String, pub binding_id: String, pub expected_binding_revision: i64, pub member_id: String,
    #[serde(deserialize_with = "required_nullable")] pub expected_grant_revision: Option<i64>,
    pub scope: GrantScope, pub read: bool, pub import: bool, pub triage: bool, pub publication_domains: Vec<Domain>,
    #[serde(deserialize_with = "required_nullable")] pub expires_at: Option<String>,
});
input!(RevokeGrantInput {
    pub operation_id: String, pub binding_id: String, pub expected_binding_revision: i64,
    pub grant_id: String, pub expected_grant_revision: i64,
});
dto!(BindingCapabilities { pub read: bool, pub import: bool, pub triage: bool, pub publication_domains: Vec<Domain>, });
dto!(BindingSummary {
    pub id: String, pub kind: SourceKind, pub label: String, pub domain: Domain, pub revision: i64,
    pub access_epoch: i64, pub enabled: bool, pub credential_state: CredentialState, pub capabilities: BindingCapabilities,
});
dto!(BindingAdmin {
    pub id: String, pub kind: SourceKind, pub label: String, pub domain: Domain, pub source_owner_id: String,
    pub revision: i64, pub access_epoch: i64, pub enabled: bool, pub publication_domains: Vec<Domain>,
    pub retained_task_text: bool, pub credential_state: CredentialState, pub resource: ResourceIdentity,
});
dto!(BindingView { pub binding: BindingSummary, pub admin: Option<BindingAdmin>, });
dto!(BindingList { pub can_manage_setup: bool, pub setup_kinds: Vec<SourceKind>, pub setup_domains: Vec<Domain>, pub items: Vec<BindingView>, pub page: u32, pub has_more: bool, });
dto!(Grant {
    pub id: String, pub binding_id: String, pub member_id: String, pub revision: i64,
    pub state: GrantState, pub scope: GrantScope, pub read: bool, pub import: bool, pub triage: bool,
    pub publication_domains: Vec<Domain>, pub expires_at: Option<String>,
});
dto!(GrantPage { pub items: Vec<Grant>, pub page: u32, pub has_more: bool, });
dto!(GrantResult { pub binding: BindingAdmin, pub grant: Grant, });

input!(SourceInput { pub source_id: String, });
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Selection {
    Window {
        #[serde(rename = "fromDate")]
        from_date: String,
        #[serde(rename = "toDate")]
        to_date: String,
    },
    Record {
        #[serde(rename = "sourceId")]
        source_id: String,
    },
}
#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LegacyRef {
    Email {
        #[serde(rename = "conversationId")]
        conversation_id: i32,
        #[serde(rename = "messageId")]
        message_id: i32,
    },
    HafidhTestflight {
        ulid: String,
    },
}
input!(StartImportInput { pub operation_id: String, pub binding_id: String, pub selection: Selection, });
input!(CaptureInput { pub operation_id: String, pub binding_id: String, #[serde(rename = "ref")] pub reference: LegacyRef, });
input!(ImportsInput { pub binding_id: String, #[serde(default)] pub view: ImportView, #[serde(default)] pub page: u32, });
input!(ImportInput { pub import_id: String, });
input!(ImportRevisionInput { pub operation_id: String, pub import_id: String, pub expected_revision: i64, });
input!(CandidatesInput { pub source_id: String, #[serde(default)] pub state: CandidateState, #[serde(default)] pub page: u32, });
input!(CandidateInput { pub candidate_id: String, });
input!(CreateCandidateInput { pub operation_id: String, pub source_id: String, pub expected_source_revision: i64, pub passage_ids: Vec<String>, });
input!(SelectCandidateInput { pub operation_id: String, pub candidate_id: String, pub expected_revision: i64, pub expected_source_revision: i64, pub passage_ids: Vec<String>, });
input!(EditCandidateInput {
    pub operation_id: String, pub candidate_id: String, pub expected_revision: i64, pub expected_source_revision: i64,
    pub passage_ids: Vec<String>, pub task: crate::business_tasks::types::CreateInput,
    pub owner_suggestion: Option<String>, pub due_suggestion: Option<String>,
});
input!(AcceptCandidateInput { pub operation_id: String, pub candidate_id: String, pub expected_revision: i64, pub expected_source_revision: i64, pub publish_to_domain: Domain, });
input!(LinkCandidateInput {
    pub operation_id: String, pub candidate_id: String, pub expected_revision: i64, pub expected_source_revision: i64,
    pub task_id: String, pub expected_task_revision: i64, pub publish_to_domain: Domain,
});
input!(DiscardInput { pub operation_id: String, pub candidate_id: String, pub expected_revision: i64, });
input!(TaskInput { pub task_id: String, });
dto!(SourceSummary {
    pub id: String, pub binding_id: String, pub kind: SourceKind, pub title: String, pub revision: Option<i64>,
    pub observed_at: Option<String>, pub access_valid_until: Option<String>, pub access: AccessState,
    pub content: ContentState, pub summary: SummaryState, pub provider_summary_status: Option<String>, pub requires_refresh: bool,
});
dto!(SourceDetail { pub source: SourceSummary, pub disclosure: Disclosure, pub passages: Vec<Passage>, pub candidate_count: u64, });
dto!(SourcePage { pub items: Vec<SourceSummary>, pub page: u32, pub has_more: bool, });
dto!(Passage { pub id: String, pub source_revision: i64, pub kind: PassageKind, pub text: String, pub index: Option<i64>, pub start: Option<f64>, pub end: Option<f64>, });
dto!(CandidateCapabilities { pub select: bool, pub edit: bool, pub accept: bool, pub link: bool, pub discard: bool, pub publication_domains: Vec<Domain>, });
dto!(Candidate {
    pub id: String, pub source_id: String, pub revision: i64, pub source_revision: i64, pub state: CandidateState,
    pub origin: CandidateOrigin, pub requires_rebase: bool, pub disclosure: Disclosure, pub has_prepared_draft: bool,
    pub draft: Option<PreparedTask>, pub owner_suggestion: Option<String>, pub due_suggestion: Option<String>, pub capabilities: CandidateCapabilities,
});
dto!(CandidateDetail { pub candidate: Candidate, pub passages: Vec<Passage>, pub source: SourceSummary, pub decision: Option<Decision>, });
dto!(CandidatePage { pub items: Vec<Candidate>, pub page: u32, pub has_more: bool, });
#[derive(Debug, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum DecisionTask {
    None,
    Restricted,
    Accessible {
        #[serde(rename = "taskId")]
        task_id: String,
        #[serde(rename = "taskRevision")]
        task_revision: i64,
    },
}
dto!(Decision {
    pub id: String, pub candidate_id: String, pub from_revision: i64, pub source_revision: i64,
    pub kind: CandidateState, pub actor_id: String, pub task: DecisionTask, pub created_at: String,
});
dto!(DecisionResult { pub decision: Decision, pub task: crate::business_tasks::types::Detail, pub replayed: bool, });
dto!(ImportCapabilities { pub advance: bool, pub cancel: bool, });
dto!(Import {
    pub id: String, pub binding_id: String, pub revision: i64, pub state: ImportState, pub coverage: Coverage,
    pub discovered: u64, pub completed: u64, pub failed: u64, pub next_attempt_at: Option<String>,
    pub error_code: Option<super::error::Reason>, pub capabilities: ImportCapabilities,
});
dto!(ImportPage { pub items: Vec<Import>, pub page: u32, pub has_more: bool, });
dto!(SourceLinkView { pub link_id: String, pub accessible: bool, pub source: Option<SourceSummary>, });
dto!(TaskSources { pub links: Vec<SourceLinkView>, });
