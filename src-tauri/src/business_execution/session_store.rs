//! Durable session admission over the existing identity/task writer. The runner
//! must consume an Admission once; replayed requests never return a new permit.
use super::{common::*, receipts, records, scope, types::*, validation};
use crate::business_identity::{self as identity, Principal};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, TransactionTrait,
};

/// Host discovery only. No deserialization or transport command accepts this.
pub(crate) struct DiscoveredProfile {
    pub config_key: String,
    pub config_hash: String,
    pub summary: ProfileSummary,
}
pub(crate) async fn sync_profiles(
    db: &DatabaseConnection,
    principal: &Principal,
    discovered: Vec<DiscoveredProfile>,
) -> Result<ProfilesResult> {
    let tx = identity::begin_write(db, principal.organization_id()).await?;
    scope::operator(&tx, principal).await?;
    let mut profiles = vec![];
    let mut retained = std::collections::HashSet::new();
    for mut found in discovered {
        if found.config_hash.len() != 64
            || !found.config_hash.bytes().all(|b| b.is_ascii_hexdigit())
            || !retained.insert((found.summary.client_id.clone(), found.config_key.clone()))
        {
            return Err(OperationReason::Invalid.into());
        }
        let old = records::Profile::find_by_statement(statement(
            "SELECT * FROM business_execution_profile WHERE organization_id=? AND member_id=? AND client_id=? AND config_key=?",
            vec![principal.organization_id().into(), principal.member_id().into(), found.summary.client_id.clone().into(), found.config_key.clone().into()]))
            .one(&tx).await?;
        let (profile_id, revision) = match &old {
            Some(row) => (
                row.id.clone(),
                row.revision
                    .checked_add(i64::from(
                        row.config_hash != found.config_hash || row.retired_at.is_some(),
                    ))
                    .ok_or(OperationReason::Conflict)?,
            ),
            None => (id(), 1),
        };
        found.summary.id = profile_id.clone();
        found.summary.revision = revision;
        found.summary.custody = Custody::OriginalOperator;
        let json = encode(&found.summary)?;
        if old.is_some() {
            tx.execute(statement("UPDATE business_execution_profile SET config_hash=?,revision=?,summary_json=?,retired_at=NULL WHERE organization_id=? AND member_id=? AND id=?",
                vec![found.config_hash.into(), revision.into(), json.into(), principal.organization_id().into(), principal.member_id().into(), profile_id.into()])).await?;
        } else {
            tx.execute(statement("INSERT INTO business_execution_profile(id,organization_id,member_id,client_id,config_key,config_hash,revision,summary_json) VALUES(?,?,?,?,?,?,?,?)",
                vec![profile_id.into(), principal.organization_id().into(), principal.member_id().into(), found.summary.client_id.clone().into(), found.config_key.into(), found.config_hash.into(), revision.into(), json.into()])).await?;
        }
        profiles.push(found.summary);
    }
    let old = records::Profile::find_by_statement(statement("SELECT * FROM business_execution_profile WHERE organization_id=? AND member_id=? AND retired_at IS NULL",
        vec![principal.organization_id().into(), principal.member_id().into()])).all(&tx).await?;
    for row in old {
        if !retained.contains(&(row.client_id, row.config_key)) {
            tx.execute(statement(
                "UPDATE business_execution_profile SET retired_at=?,revision=revision+1 WHERE id=?",
                vec![now().into(), row.id.into()],
            ))
            .await?;
        }
    }
    tx.commit().await?;
    Ok(ProfilesResult {
        unavailable_reason: profiles.is_empty().then_some(SetupReason::MissingClient),
        profiles,
    })
}

pub(super) fn summary(row: &records::Session) -> Result<SessionSummary> {
    let status: SessionStatus = decode(&encode(&row.status)?)?;
    let live = matches!(
        status,
        SessionStatus::Idle | SessionStatus::Running | SessionStatus::AwaitingInput
    );
    let idle = status == SessionStatus::Idle;
    let terminal = row.mode == "terminal";
    Ok(SessionSummary {
        id: row.id.clone(),
        task_id: row.task_id.clone(),
        profile_id: row.profile_id.clone(),
        profile_revision: row.profile_revision,
        revision: row.revision,
        generation: row.generation,
        status,
        mode: decode(&encode(&row.mode)?)?,
        title: row.title.clone(),
        created_at: row.created_at.clone(),
        updated_at: row.updated_at.clone(),
        last_activity_at: row.last_activity_at.clone(),
        capabilities: SessionCapabilities {
            read: true,
            prompt: idle && !terminal,
            r#continue: matches!(
                status,
                SessionStatus::Stopped | SessionStatus::Failed | SessionStatus::Interrupted
            ),
            stop: live || status == SessionStatus::Starting,
            terminal_write: live && terminal,
            import_output: live || status == SessionStatus::Stopped,
        },
        reason: match status {
            SessionStatus::Interrupted => Some(SessionReason::LaunchUncertain),
            SessionStatus::Revoked | SessionStatus::Closed => Some(SessionReason::AuthorityChanged),
            _ => None,
        },
    })
}
pub(crate) async fn get(
    db: &DatabaseConnection,
    principal: &Principal,
    input: SessionInput,
) -> Result<SessionGetResult> {
    validation::uuid(&input.session_id)?;
    let tx = db.begin().await?;
    let row = scope::session(&tx, principal, &input.session_id).await?;
    let result = SessionGetResult {
        session: summary(&row)?,
    };
    tx.commit().await?;
    Ok(result)
}
pub(crate) async fn list(
    db: &DatabaseConnection,
    principal: &Principal,
    input: SessionListInput,
) -> Result<Page<SessionSummary>> {
    validation::uuid(&input.task_id)?;
    validation::page(input.limit, 50, input.cursor.as_deref())?;
    let tx = db.begin().await?;
    scope::task(&tx, principal, &input.task_id, false).await?;
    // A cursor is an actual row UUID under this task, never an offset or caller
    // authority. Session IDs are immutable and give deterministic pagination.
    if let Some(cursor) = &input.cursor {
        let row = scope::session(&tx, principal, cursor).await?;
        if row.task_id != input.task_id {
            return Err(OperationReason::Missing.into());
        }
    }
    let mut rows = records::Session::find_by_statement(statement(
        "SELECT * FROM business_execution_session WHERE organization_id=? AND member_id=? AND task_id=? AND (? IS NULL OR id<?) ORDER BY id DESC LIMIT ?",
        vec![principal.organization_id().into(), principal.member_id().into(), input.task_id.into(), input.cursor.clone().into(), input.cursor.into(), (i64::from(input.limit)+1).into()])).all(&tx).await?;
    let next_cursor =
        (rows.len() > input.limit as usize).then(|| rows[input.limit as usize - 1].id.clone());
    rows.truncate(input.limit as usize);
    let mut items = vec![];
    for row in rows {
        match scope::validate_session(&tx, principal, &row).await {
            Ok(()) => items.push(summary(&row)?),
            Err(Error(OperationReason::AuthorityChanged)) => {}
            Err(error) => return Err(error),
        }
    }
    tx.commit().await?;
    Ok(Page { items, next_cursor })
}

/// Backend-owned admission and exact captured principal. Never serialized to UI.
pub(super) struct Admission {
    principal: Principal,
    operation_id: String,
    kind: OperationKind,
    session_id: String,
    generation: i64,
    admission_id: String,
    profile_id: String,
    profile_revision: i64,
    mode: Mode,
}
impl Admission {
    pub(super) fn session_id(&self) -> &str {
        &self.session_id
    }
}
pub(super) struct Reservation {
    pub result: SessionResult,
    pub admission: Option<Admission>,
}
async fn add_generation(
    tx: &DatabaseTransaction,
    principal: &Principal,
    session_id: &str,
    generation: i64,
    admission_id: &str,
) -> Result<()> {
    tx.execute(statement("INSERT INTO business_execution_generation(organization_id,session_id,generation,admission_id,initial_cursor,created_at) VALUES(?,?,?,?,?,?)",
        vec![principal.organization_id().into(), session_id.into(), generation.into(), admission_id.into(), id().into(), now().into()])).await?;
    Ok(())
}
pub(super) async fn reserve_start(
    db: &DatabaseConnection,
    principal: &Principal,
    input: &StartInput,
) -> Result<Reservation> {
    validation::start(input)?;
    let tx = identity::begin_write(db, principal.organization_id()).await?;
    scope::operator(&tx, principal).await?;
    if let Some(old) = receipts::replay(
        &tx,
        principal,
        OperationKind::Start,
        &input.operation_id,
        input,
    )
    .await?
    {
        let row = scope::session(
            &tx,
            principal,
            old.session_id
                .as_deref()
                .ok_or(OperationReason::Unavailable)?,
        )
        .await?;
        let result = SessionResult {
            session: summary(&row)?,
            operation: old.summary()?,
        };
        tx.commit().await?;
        return Ok(Reservation {
            result,
            admission: None,
        });
    }
    let task = scope::task(&tx, principal, &input.task_id, true).await?;
    let profile = scope::profile(&tx, principal, &input.profile_id).await?;
    let configured: ProfileSummary = decode(&profile.summary_json)?;
    if task.task.revision != input.expected_task_revision
        || profile.revision != input.expected_profile_revision
    {
        return Err(OperationReason::Conflict.into());
    }
    if profile.retired_at.is_some()
        || configured.readiness != Readiness::Ready
        || !configured.capabilities.start
        || !configured.modes.contains(&input.mode)
    {
        return Err(OperationReason::SetupRequired.into());
    }
    let session_id = id();
    let admission_id = id();
    let stamp = now();
    tx.execute(statement("INSERT INTO business_execution_session(id,organization_id,task_id,member_id,authority_json,authorization_epoch,task_scope_epoch,profile_id,profile_revision,revision,generation,mode,status,title,created_at,updated_at,last_activity_at) VALUES(?,?,?,?,?,?,?,?,?,1,1,?,'starting',?,?,?,?)",
        vec![session_id.clone().into(), principal.organization_id().into(), input.task_id.clone().into(), principal.member_id().into(), scope::authority(principal)?.into(), principal.authorization_epoch().into(), task.epoch.into(), profile.id.clone().into(), profile.revision.into(), key(input.mode)?.into(), task.task.title.into(), stamp.clone().into(), stamp.clone().into(), stamp.into()])).await?;
    add_generation(&tx, principal, &session_id, 1, &admission_id).await?;
    receipts::reserve(
        &tx,
        principal,
        OperationKind::Start,
        &input.operation_id,
        input,
        receipts::Target {
            task_id: &input.task_id,
            session_id: Some(&session_id),
            generation: Some(1),
        },
        &session_id,
    )
    .await?;
    let row = scope::session(&tx, principal, &session_id).await?;
    let result = SessionResult {
        session: summary(&row)?,
        operation: OperationSummary {
            id: input.operation_id.clone(),
            status: OperationStatus::Pending,
            reason: None,
        },
    };
    let admission = Admission {
        principal: principal.clone(),
        operation_id: input.operation_id.clone(),
        kind: OperationKind::Start,
        session_id,
        generation: 1,
        admission_id,
        profile_id: profile.id,
        profile_revision: profile.revision,
        mode: input.mode,
    };
    tx.commit().await?;
    Ok(Reservation {
        result,
        admission: Some(admission),
    })
}

/// Private existing-engine linkage. The caller is the owned runtime, never wire.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct EngineLink {
    pub connection_id: String,
    pub conversation_id: Option<i32>,
    pub folder_id: Option<i32>,
}
pub(super) async fn generation(
    tx: &DatabaseTransaction,
    principal: &Principal,
    session_id: &str,
    generation: i64,
) -> Result<records::Generation> {
    records::Generation::find_by_statement(statement("SELECT * FROM business_execution_generation WHERE organization_id=? AND session_id=? AND generation=?",
        vec![principal.organization_id().into(), session_id.into(), generation.into()])).one(tx).await?.ok_or_else(|| OperationReason::Missing.into())
}
pub(super) async fn complete_launch(
    db: &DatabaseConnection,
    admission: &Admission,
    engine: &EngineLink,
) -> Result<SessionResult> {
    let principal = &admission.principal;
    let tx = identity::begin_write(db, principal.organization_id()).await?;
    let row = scope::session(&tx, principal, &admission.session_id).await?;
    let gen = generation(&tx, principal, &row.id, admission.generation).await?;
    let receipt = receipts::find(&tx, principal, admission.kind, &admission.operation_id)
        .await?
        .ok_or(OperationReason::Missing)?;
    receipts::require_target(
        &receipt,
        &receipts::Target {
            task_id: &row.task_id,
            session_id: Some(&row.id),
            generation: Some(admission.generation),
        },
        &row.id,
    )?;
    if !matches!(
        admission.kind,
        OperationKind::Start | OperationKind::Continue
    ) || !matches!(receipt.status.as_str(), "pending" | "uncertain")
        || row.generation != admission.generation
        || row.status != "starting"
        || gen.admission_id != admission.admission_id
        || gen.engine_json.is_some()
        || row.profile_id != admission.profile_id
        || row.profile_revision != admission.profile_revision
        || row.mode != key(admission.mode)?
    {
        return Err(OperationReason::AuthorityChanged.into());
    }
    let changed = tx.execute(statement("UPDATE business_execution_generation SET engine_json=? WHERE organization_id=? AND session_id=? AND generation=? AND admission_id=? AND engine_json IS NULL",
        vec![encode(engine)?.into(), principal.organization_id().into(), row.id.clone().into(), row.generation.into(), admission.admission_id.clone().into()])).await?.rows_affected();
    if changed != 1 {
        return Err(OperationReason::Conflict.into());
    }
    set_status(&tx, principal, &row, SessionStatus::Idle).await?;
    let row = scope::session(&tx, principal, &row.id).await?;
    let result = SessionResult {
        session: summary(&row)?,
        operation: OperationSummary {
            id: admission.operation_id.clone(),
            status: OperationStatus::Confirmed,
            reason: None,
        },
    };
    receipts::complete(
        &tx,
        principal,
        admission.kind,
        &admission.operation_id,
        receipts::Completion {
            target: receipts::Target {
                task_id: &row.task_id,
                session_id: Some(&row.id),
                generation: Some(row.generation),
            },
            resource_id: &row.id,
            status: OperationStatus::Confirmed,
            reason: None,
            result: Some(&result),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(result)
}
pub(super) async fn set_status(
    tx: &DatabaseTransaction,
    principal: &Principal,
    row: &records::Session,
    status: SessionStatus,
) -> Result<()> {
    let stamp = now();
    let changed = tx.execute(statement("UPDATE business_execution_session SET status=?,revision=revision+1,updated_at=?,last_activity_at=? WHERE organization_id=? AND id=? AND revision=? AND generation=?",
        vec![key(status)?.into(), stamp.clone().into(), stamp.into(), principal.organization_id().into(), row.id.clone().into(), row.revision.into(), row.generation.into()])).await?.rows_affected();
    if changed != 1 {
        return Err(OperationReason::Conflict.into());
    }
    Ok(())
}

pub(crate) async fn operation(
    db: &DatabaseConnection,
    principal: &Principal,
    input: OperationInput,
) -> Result<OperationResult> {
    validation::uuid(&input.operation_id)?;
    let tx = db.begin().await?;
    let receipt = receipts::find(&tx, principal, input.kind, &input.operation_id)
        .await?
        .ok_or(OperationReason::Missing)?;
    let result = OperationResult {
        operation: receipt.summary()?,
        resource_id: receipt.resource_id,
    };
    tx.commit().await?;
    Ok(result)
}

#[cfg(test)]
mod completion_tests;
