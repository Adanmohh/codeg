use super::*;

fn update(org: &str, member: &Member, role: Role, domains: Vec<Domain>) -> UpdateMemberInput {
    UpdateMemberInput {
        organization_id: org.into(),
        member_id: member.id.clone(),
        expected_revision: member.revision,
        display_name: member.display_name.clone(),
        role,
        domains,
    }
}

#[tokio::test]
async fn administrative_grants_never_exceed_current_authority_or_create_elevated_agents() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let org = op.organization_id();
    let (admin, _, admin_p) = human(&db.conn, &op, Role::Admin, vec![Domain::Feedback]).await;
    let (ordinary, _, _) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    for role in [Role::Owner, Role::Admin] {
        assert!(matches!(
            store::create_member(
                &db.conn,
                &admin_p,
                CreateMemberInput {
                    organization_id: org.into(),
                    display_name: "Escalated".into(),
                    kind: MemberKind::Human,
                    role,
                    domains: vec![Domain::Feedback],
                }
            )
            .await,
            Err(IdentityError::Forbidden)
        ));
        assert!(matches!(
            store::update_member(
                &db.conn,
                &admin_p,
                update(org, &ordinary, role, vec![Domain::Feedback])
            )
            .await,
            Err(IdentityError::Forbidden)
        ));
    }
    assert!(matches!(
        store::update_member(
            &db.conn,
            &admin_p,
            update(org, &admin, Role::Admin, Domain::ALL.to_vec())
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    assert!(matches!(
        store::update_member(
            &db.conn,
            &admin_p,
            update(org, &ordinary, Role::Member, Domain::ALL.to_vec())
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    assert!(matches!(
        store::create_member(
            &db.conn,
            &admin_p,
            CreateMemberInput {
                organization_id: org.into(),
                display_name: "Outside domain".into(),
                kind: MemberKind::Human,
                role: Role::Member,
                domains: vec![Domain::Engineering],
            }
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    for role in [Role::Owner, Role::Admin, Role::Manager, Role::Viewer] {
        assert!(matches!(
            store::create_member(
                &db.conn,
                &op,
                CreateMemberInput {
                    organization_id: org.into(),
                    display_name: "Elevated agent".into(),
                    kind: MemberKind::Agent,
                    role,
                    domains: vec![Domain::Feedback],
                }
            )
            .await,
            Err(IdentityError::Invalid(_))
        ));
    }
    for member_id in [op.member_id(), admin.id.as_str()] {
        assert!(matches!(
            store::issue_credential(
                &db.conn,
                &admin_p,
                IssueCredentialInput {
                    organization_id: org.into(),
                    member_id: member_id.into(),
                    label: "Escalated credential".into(),
                }
            )
            .await,
            Err(IdentityError::Forbidden | IdentityError::NotFound)
        ));
    }
    let owner = store::member(&db.conn, org, op.member_id())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        store::update_member(
            &db.conn,
            &op,
            update(org, &owner, Role::Member, vec![Domain::Feedback])
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    assert!(matches!(
        store::revoke_member(
            &db.conn,
            &op,
            RevokeMemberInput {
                organization_id: org.into(),
                member_id: owner.id,
                expected_revision: owner.revision,
            }
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    let created = store::create_member(
        &db.conn,
        &admin_p,
        CreateMemberInput {
            organization_id: org.into(),
            display_name: "Permitted manager".into(),
            kind: MemberKind::Human,
            role: Role::Manager,
            domains: vec![Domain::Feedback],
        },
    )
    .await
    .unwrap();
    assert!(created.allows(Permission::Review, Some(Domain::Feedback)));
}

#[tokio::test]
async fn directory_and_active_references_respect_current_domain_org_and_membership() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let org = op.organization_id();
    let (_, _, reader) = human(&db.conn, &op, Role::Viewer, vec![Domain::Feedback]).await;
    let (engineering, _, _) = human(&db.conn, &op, Role::Member, vec![Domain::Engineering]).await;
    let (revoked, _, _) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    store::revoke_member(
        &db.conn,
        &op,
        RevokeMemberInput {
            organization_id: org.into(),
            member_id: revoked.id.clone(),
            expected_revision: 1,
        },
    )
    .await
    .unwrap();
    let members = store::list_members(
        &db.conn,
        &reader,
        MembersInput {
            organization_id: org.into(),
            domain: None,
        },
    )
    .await
    .unwrap();
    assert!(!members
        .iter()
        .any(|m| m.id == engineering.id || m.id == revoked.id));
    for (organization, member_id, domain) in [
        (org, engineering.id.as_str(), Domain::Feedback),
        (org, revoked.id.as_str(), Domain::Feedback),
        ("foreign-organization", reader.member_id(), Domain::Feedback),
    ] {
        assert!(matches!(
            active_reference(&db.conn, organization, member_id, domain).await,
            Err(IdentityError::NotFound)
        ));
    }
    assert!(
        active_reference(&db.conn, org, &engineering.id, Domain::Engineering)
            .await
            .is_ok()
    );
    assert!(matches!(
        authorize(
            &db.conn,
            &reader,
            "foreign-organization",
            Permission::Read,
            None
        )
        .await,
        Err(IdentityError::NotFound)
    ));
    for permission in [
        Permission::Create,
        Permission::Contribute,
        Permission::Assign,
        Permission::Review,
        Permission::ManageMembers,
    ] {
        assert!(matches!(
            authorize(&db.conn, &reader, org, permission, Some(Domain::Feedback)).await,
            Err(IdentityError::Forbidden)
        ));
    }
}

#[tokio::test]
async fn original_credential_and_current_human_agent_grants_fence_restored_delegations() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let org = op.organization_id();
    let (manager, issued, principal) =
        human(&db.conn, &op, Role::Manager, vec![Domain::Feedback]).await;
    let agent = store::create_member(
        &db.conn,
        &op,
        CreateMemberInput {
            organization_id: org.into(),
            display_name: "Named synthetic agent".into(),
            kind: MemberKind::Agent,
            role: Role::Member,
            domains: vec![Domain::Feedback, Domain::Engineering],
        },
    )
    .await
    .unwrap();
    let lineage = delegation_grant(&principal).unwrap().to_storage().unwrap();
    assert!(!lineage.contains(&issued.token));
    assert!(lineage.contains(&issued.credential.id));
    let restored = agent_principal_from_binding(&db.conn, org, &agent.id, &lineage)
        .await
        .unwrap();
    assert_eq!(restored.member_id(), agent.id);
    assert!(!restored.is_operator());
    assert!(authorize(
        &db.conn,
        &restored,
        org,
        Permission::Contribute,
        Some(Domain::Feedback)
    )
    .await
    .is_ok());
    for (permission, domain) in [
        (Permission::Review, Some(Domain::Feedback)),
        (Permission::Assign, Some(Domain::Feedback)),
        (Permission::Create, Some(Domain::Feedback)),
        (Permission::ManageMembers, None),
        (Permission::Read, None),
        (Permission::Read, Some(Domain::Engineering)),
    ] {
        assert!(matches!(
            authorize(&db.conn, &restored, org, permission, domain).await,
            Err(IdentityError::Forbidden)
        ));
    }
    assert!(matches!(
        delegation_grant(&restored),
        Err(IdentityError::Forbidden)
    ));
    assert!(
        agent_principal_from_binding(&db.conn, "wrong-organization", &agent.id, &lineage)
            .await
            .is_err()
    );
    assert!(agent_principal(&db.conn, &restored, &agent.id)
        .await
        .is_err());
    assert!(matches!(
        store::issue_credential(
            &db.conn,
            &op,
            IssueCredentialInput {
                organization_id: org.into(),
                member_id: agent.id.clone(),
                label: "No agent human login".into()
            }
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    // Current role/domain, not link-time grants, governs an already restored principal.
    let changed = store::update_member(
        &db.conn,
        &op,
        update(org, &manager, Role::Viewer, vec![Domain::Feedback]),
    )
    .await
    .unwrap();
    assert!(matches!(
        authorize(
            &db.conn,
            &restored,
            org,
            Permission::Contribute,
            Some(Domain::Feedback)
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    store::update_member(
        &db.conn,
        &op,
        update(org, &changed, Role::Manager, vec![Domain::Feedback]),
    )
    .await
    .unwrap();
    // A second still-valid credential must not revive an older revoked lineage.
    let replacement = store::issue_credential(
        &db.conn,
        &op,
        IssueCredentialInput {
            organization_id: org.into(),
            member_id: manager.id,
            label: "Replacement session".into(),
        },
    )
    .await
    .unwrap();
    store::revoke_credential(
        &db.conn,
        &op,
        RevokeCredentialInput {
            organization_id: org.into(),
            credential_id: issued.credential.id,
        },
    )
    .await
    .unwrap();
    assert!(store::resolve_credential(&db.conn, &replacement.token)
        .await
        .is_ok());
    assert!(matches!(
        agent_principal_from_binding(&db.conn, org, &agent.id, &lineage).await,
        Err(IdentityError::Unauthorized)
    ));
    let tx = begin_write(&db.conn, org).await.unwrap();
    assert!(matches!(
        authorize(
            &tx,
            &restored,
            org,
            Permission::Contribute,
            Some(Domain::Feedback)
        )
        .await,
        Err(IdentityError::Unauthorized)
    ));
}

#[tokio::test]
async fn revoking_members_revokes_every_token_and_does_not_retarget_old_identity() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let org = op.organization_id();
    let (member, first, cached) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let second = store::issue_credential(
        &db.conn,
        &op,
        IssueCredentialInput {
            organization_id: org.into(),
            member_id: member.id.clone(),
            label: "Second browser".into(),
        },
    )
    .await
    .unwrap();
    assert!(first.token != second.token);
    assert!(first.credential.id != second.credential.id);
    store::revoke_member(
        &db.conn,
        &op,
        RevokeMemberInput {
            organization_id: org.into(),
            member_id: member.id.clone(),
            expected_revision: member.revision,
        },
    )
    .await
    .unwrap();
    for token in [&first.token, &second.token] {
        assert!(matches!(
            store::resolve_credential(&db.conn, token).await,
            Err(IdentityError::Unauthorized)
        ));
    }
    assert!(matches!(
        authorize(&db.conn, &cached, org, Permission::Read, None).await,
        Err(IdentityError::Unauthorized)
    ));
    let replacement = store::create_member(
        &db.conn,
        &op,
        CreateMemberInput {
            organization_id: org.into(),
            display_name: member.display_name,
            kind: MemberKind::Human,
            role: Role::Member,
            domains: vec![Domain::Feedback],
        },
    )
    .await
    .unwrap();
    assert_ne!(replacement.id, member.id);
    let credentials = store::list_credentials(
        &db.conn,
        &op,
        CredentialsInput {
            organization_id: org.into(),
            member_id: member.id,
        },
    )
    .await
    .unwrap();
    assert_eq!(credentials.len(), 2);
    assert!(credentials.iter().all(|c| c.revoked_at.is_some()));
}

#[tokio::test]
async fn self_revoke_does_not_reveal_or_revoke_another_members_credential() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (_, issued, viewer) = human(&db.conn, &op, Role::Viewer, vec![Domain::Feedback]).await;
    let (_, other, _) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    for credential_id in [other.credential.id, uuid::Uuid::new_v4().to_string()] {
        assert!(matches!(
            store::revoke_credential(
                &db.conn,
                &viewer,
                RevokeCredentialInput {
                    organization_id: op.organization_id().into(),
                    credential_id
                }
            )
            .await,
            Err(IdentityError::NotFound)
        ));
    }
    store::revoke_credential(
        &db.conn,
        &viewer,
        RevokeCredentialInput {
            organization_id: op.organization_id().into(),
            credential_id: issued.credential.id,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        store::resolve_credential(&db.conn, &issued.token).await,
        Err(IdentityError::Unauthorized)
    ));
    assert!(store::resolve_credential(&db.conn, &other.token)
        .await
        .is_ok());
}
