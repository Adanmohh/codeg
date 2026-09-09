//! Closed application-command permissions derived from the actual registry.
//! Also read by build.rs; no runtime/dependency imports here.
pub const TENANT_COMMANDS: &[&str] = &[
    "business_window_context",
    "business_window_control",
    "business_session_open",
    "business_session_close",
    "business_context",
    "business_members_list",
    "business_members_create",
    "business_members_update",
    "business_members_revoke",
    "business_credentials_issue",
    "business_credentials_list",
    "business_credentials_revoke",
    "business_settings_get",
    "business_settings_update",
    "business_tasks_list",
    "business_tasks_get",
    "business_tasks_create",
    "business_tasks_update",
    "business_tasks_assign",
    "business_tasks_progress",
    "business_tasks_note",
    "business_tasks_submit",
    "business_tasks_review",
    "business_tasks_cancel",
    "business_tasks_archive",
    "business_tasks_link_execution",
];

pub fn command_names(source: &str) -> Vec<&str> {
    let body = source
        .split_once(".invoke_handler(tauri::generate_handler![")
        .expect("single native registry")
        .1
        .split_once("])")
        .expect("registry end")
        .0;
    let commands: Vec<_> = body
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.starts_with("//"))
        .map(|s| {
            let path = s
                .strip_suffix(',')
                .expect("registry entry must end with comma");
            assert!(
                path.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':'),
                "unexpected registry syntax; review ACL generator"
            );
            path.rsplit("::").next().expect("command name")
        })
        .collect();
    let unique: std::collections::BTreeSet<_> = commands.iter().collect();
    assert_eq!(commands.len(), unique.len(), "duplicate native command");
    for tenant in TENANT_COMMANDS {
        assert!(
            commands.contains(tenant),
            "unregistered tenant command: {tenant}"
        );
    }
    commands
}
pub fn permissions(source: &str) -> String {
    let commands = command_names(source);
    let quoted = |names: &[&str]| {
        names
            .iter()
            .map(|name| format!("{name:?}"))
            .collect::<Vec<_>>()
            .join(",")
    };
    format!("[[permission]]\nidentifier=\"allow-host-commands\"\ndescription=\"Existing trusted host application commands\"\ncommands.allow=[{}]\n\n[[permission]]\nidentifier=\"allow-tenant-commands\"\ndescription=\"Closed authenticated tenant commands only\"\ncommands.allow=[{}]\n",quoted(&commands),quoted(TENANT_COMMANDS))
}
pub fn is_tenant(label: &str) -> bool {
    label
        .strip_prefix("tenant-")
        .is_some_and(|id| !id.is_empty())
}
pub fn is_platform(label: &str) -> bool {
    matches!(
        label,
        "main" | "settings" | "project-boot" | "import-sessions" | "pet" | "pet-panel"
    ) || [
        "commit-",
        "merge-",
        "stash-",
        "push-",
        "remote-project-boot-",
        "remote-import-sessions-",
        "remote-workspace-",
        "remote-settings-",
        "remote-commit-",
        "remote-merge-",
        "remote-stash-",
        "remote-push-",
    ]
    .iter()
    .any(|prefix| {
        label
            .strip_prefix(prefix)
            .is_some_and(|suffix| !suffix.is_empty())
    })
}
