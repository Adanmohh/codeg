//! Trusted account/resource resolution and current public cached projections.
use super::{
    common::*,
    error::{self, Reason},
    fireflies::{Normalized, TextPassage},
    services::Services,
    types::*,
};
use crate::{
    ops::{self, Operator},
    ops_intake_host,
};
use sea_orm::{ConnectionTrait, DatabaseTransaction};

async fn generation(tx: &DatabaseTransaction, kind: &str, resource: &str) -> Result<String> {
    let row=tx.query_one(sql("SELECT generation FROM business_intake_legacy_generation WHERE kind=? AND resource_id=? AND changing=0",vec![kind.into(),resource.into()])).await?.ok_or(Reason::BindingUnavailable)?;
    let generation: i64 = row.try_get("", "generation")?;
    digest(&(kind, resource, generation))
}
pub(super) async fn resolve(
    tx: &DatabaseTransaction,
    op: &Operator,
    source: &SourceSetup,
) -> Result<ResourceIdentity> {
    match source {
        SourceSetup::Email { inbox_id } if *inbox_id > 0 => {
            let account_id = ops::business::resolve(tx, op, *inbox_id)
                .await
                .map_err(|_| Reason::BindingUnavailable)?;
            Ok(ResourceIdentity::Email {
                account_id,
                inbox_id: *inbox_id,
                configuration_identity: generation(tx, "email", &inbox_id.to_string()).await?,
            })
        }
        SourceSetup::HafidhTestflight { product_id } => {
            let account_id = ops_intake_host::business::resolve(tx, op, product_id)
                .await
                .map_err(|_| Reason::BindingUnavailable)?;
            Ok(ResourceIdentity::HafidhTestflight {
                account_id,
                product_id: product_id.clone(),
                configuration_identity: generation(tx, "hafidh_testflight", product_id).await?,
            })
        }
        _ => Err(error::invalid()),
    }
}
/// Returns only the private store reference to the caller inside this module.
async fn reference(tx: &DatabaseTransaction, resource: &ResourceIdentity) -> Result<String> {
    match resource {
        ResourceIdentity::Email {
            account_id,
            inbox_id,
            configuration_identity,
        } => {
            if generation(tx, "email", &inbox_id.to_string()).await? != *configuration_identity {
                return Err(Reason::BindingUnavailable.into());
            }
            ops::business::credential(tx, *account_id, *inbox_id)
                .await
                .map_err(|_| Reason::BindingUnavailable.into())
        }
        ResourceIdentity::HafidhTestflight {
            account_id,
            product_id,
            configuration_identity,
        } => {
            if generation(tx, "hafidh_testflight", product_id).await? != *configuration_identity {
                return Err(Reason::BindingUnavailable.into());
            }
            ops_intake_host::business::credential(tx, *account_id, product_id)
                .await
                .map_err(|_| Reason::BindingUnavailable.into())
        }
        _ => Err(Reason::BindingUnavailable.into()),
    }
}
pub(super) async fn check(
    tx: &DatabaseTransaction,
    resource: &ResourceIdentity,
    services: &Services,
) -> Result<()> {
    if services
        .get(&reference(tx, resource).await?)
        .await?
        .is_none()
    {
        return Err(Reason::CredentialUnavailable.into());
    }
    Ok(())
}
pub(super) async fn project(
    tx: &DatabaseTransaction,
    resource: &ResourceIdentity,
    reference: &LegacyRef,
    services: &Services,
) -> Result<Normalized> {
    check(tx, resource, services).await?;
    let (title, text, kind, expires_at, provider_revision) = match (resource, reference) {
        (
            ResourceIdentity::Email {
                account_id,
                inbox_id,
                ..
            },
            LegacyRef::Email {
                conversation_id,
                message_id,
            },
        ) if *conversation_id > 0 && *message_id > 0 => {
            let content =
                ops::business::message(tx, *account_id, *inbox_id, *conversation_id, *message_id)
                    .await
                    .map_err(|_| Reason::SourceDenied)?;
            (
                "Email message".into(),
                content,
                PassageKind::EmailMessage,
                None,
                None,
            )
        }
        (
            ResourceIdentity::HafidhTestflight {
                account_id,
                product_id,
                ..
            },
            LegacyRef::HafidhTestflight { ulid },
        ) => {
            let row = ops_intake_host::business::snapshot(tx, *account_id, product_id, ulid)
                .await
                .map_err(|_| Reason::SourceExpired)?;
            let expires = chrono::DateTime::from_timestamp(row.valid_until, 0)
                .ok_or(Reason::SourceExpired)?
                .to_rfc3339();
            (
                row.title,
                row.description,
                PassageKind::Feedback,
                Some(expires),
                Some(row.revision),
            )
        }
        _ => return Err(error::invalid()),
    };
    if !super::common::text(&title, 2000, false) || !super::common::text(&text, 20_000, false) {
        return Err(Reason::UnsupportedSchema.into());
    }
    let passages = if text.trim().is_empty() {
        vec![]
    } else {
        vec![TextPassage {
            kind,
            text,
            index: None,
            start: None,
            end: None,
        }]
    };
    Ok(Normalized {
        title,
        content: if passages.is_empty() {
            ContentState::Missing
        } else {
            ContentState::Available
        },
        passages,
        summary: SummaryState::Missing,
        provider_status: None,
        expires_at,
        provider_revision,
    })
}
/// Fence before a legacy in-place key write/delete, even if that store I/O fails.
/// SQLite serializes this write with intake capture/decision transactions.
pub(crate) async fn fence_email<C: ConnectionTrait>(
    db: &C,
    inbox_id: i32,
    changing: bool,
) -> std::result::Result<(), sea_orm::DbErr> {
    db.execute(sql("INSERT INTO business_intake_legacy_generation(kind,resource_id,generation,changing) VALUES('email',?,1,?) ON CONFLICT(kind,resource_id) DO UPDATE SET generation=generation+1,changing=excluded.changing",vec![inbox_id.to_string().into(),changing.into()])).await?;
    Ok(())
}
