#[cfg(feature = "tauri-runtime")]
const SERVICE_NAME: &str = "codeg";

fn token_key(account_id: &str) -> String {
    format!("github-token:{}", account_id)
}

fn channel_token_key(channel_id: i32) -> String {
    format!("chat-channel:{}", channel_id)
}

// ── Tauri mode: OS keyring ──

#[cfg(feature = "tauri-runtime")]
pub fn set_token(account_id: &str, token: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE_NAME, &token_key(account_id))
        .map_err(|e| format!("keyring init error: {e}"))?;
    entry
        .set_password(token)
        .map_err(|e| format!("keyring set error: {e}"))
}

#[cfg(feature = "tauri-runtime")]
pub fn get_token(account_id: &str) -> Option<String> {
    let entry = keyring::Entry::new(SERVICE_NAME, &token_key(account_id)).ok()?;
    entry.get_password().ok()
}

#[cfg(feature = "tauri-runtime")]
pub fn delete_token(account_id: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE_NAME, &token_key(account_id))
        .map_err(|e| format!("keyring init error: {e}"))?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("keyring delete error: {e}")),
    }
}

// ── Server mode: file-based token store ──

// Every file-store mutation is read/modify/write. Serialize the whole sequence
// so configuring two inboxes (or a GitHub account alongside one) preserves both
// entries, rather than atomically renaming a stale copy over the newer map.
#[cfg(not(feature = "tauri-runtime"))]
static TOKEN_WRITE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(not(feature = "tauri-runtime"))]
fn tokens_file_path() -> std::path::PathBuf {
    tokens_file_path_for(std::env::var("CODEG_DATA_DIR").ok().as_deref())
}

/// Resolve the on-disk `tokens.json` path given an explicit
/// `CODEG_DATA_DIR` value (or `None` to fall back to the platform
/// default). Always returns an absolute path so subprocess credential
/// helpers — which inherit our env but run in git's CWD, not ours —
/// don't end up looking for `tokens.json` in the user's repo. Factored
/// out so tests can exercise path resolution without poking at process
/// env state.
#[cfg(not(feature = "tauri-runtime"))]
fn tokens_file_path_for(env_value: Option<&str>) -> std::path::PathBuf {
    let dir = env_value.map(std::path::PathBuf::from).unwrap_or_else(|| {
        dirs::data_dir()
            .map(|d| d.join("codeg"))
            .unwrap_or_else(|| std::path::PathBuf::from(".codeg-data"))
    });
    crate::git_credential::absolutize(&dir).join("tokens.json")
}

#[cfg(not(feature = "tauri-runtime"))]
fn read_tokens() -> std::collections::HashMap<String, String> {
    read_tokens_at(&tokens_file_path())
}

/// Read the token map, first tightening a pre-existing file to `0600`.
/// Stores written before the permission hardening landed sit at the umask
/// default (usually 0644) inside a bind-mounted `/data` volume; tightening on
/// every read is idempotent and cheap, and doing it BEFORE the read means no
/// code path ever handles token bytes from a world-readable file it could have
/// fixed. Best-effort: if chmod fails the read will usually fail too, and a
/// read-only mount is not made worse by proceeding.
#[cfg(not(feature = "tauri-runtime"))]
fn read_tokens_at(path: &std::path::Path) -> std::collections::HashMap<String, String> {
    #[cfg(unix)]
    if path.exists() {
        use std::os::unix::fs::PermissionsExt;
        if let Err(err) = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)) {
            // Keep reading (a read-only mount is not made worse), but make the
            // failed hardening observable instead of silently world-readable.
            tracing::warn!(
                "[tokens] could not tighten {} to 0600: {err}",
                path.display()
            );
        }
    }
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

#[cfg(not(feature = "tauri-runtime"))]
fn read_tokens_for_write(
    path: &std::path::Path,
) -> Result<std::collections::HashMap<String, String>, String> {
    // Only a genuinely absent store starts empty. In particular, malformed or
    // unreadable existing bytes must never become a replacement empty map.
    // The caller holds TOKEN_WRITE_LOCK across this read and the atomic write.
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(std::collections::HashMap::new());
        }
        Err(_) => return Err("credential store unavailable".to_owned()),
    };
    serde_json::from_str(&contents).map_err(|_| "credential store unavailable".to_owned())
}

#[cfg(not(feature = "tauri-runtime"))]
fn change_token_at(path: &std::path::Path, key: String, value: Option<&str>) -> Result<(), String> {
    let _guard = TOKEN_WRITE_LOCK
        .lock()
        .map_err(|_| "credential store unavailable".to_string())?;
    let mut tokens = read_tokens_for_write(path)?;
    match value {
        Some(value) => {
            tokens.insert(key, value.to_string());
        }
        None => {
            tokens.remove(&key);
        }
    }
    write_tokens_at(path, &tokens)
}

/// Persist the token map without ever exposing a wide-permission file, even
/// transiently. A plain `fs::write` + chmod leaves a window (and a permanent
/// 0644 file if the process dies between the two), so on Unix the content goes
/// into a same-directory temp file created with mode `0600`, is fsynced, and
/// then atomically renamed over the store. The explicit `set_permissions`
/// after creation pins the bits exactly even under an exotic umask (umask can
/// only clear bits at open time; chmod is not masked).
#[cfg(not(feature = "tauri-runtime"))]
fn write_tokens_at(
    path: &std::path::Path,
    tokens: &std::collections::HashMap<String, String>,
) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "token store path has no parent directory".to_string())?;
    std::fs::create_dir_all(parent)
        .map_err(|e| format!("failed to create token store directory: {e}"))?;
    let json = serde_json::to_string_pretty(tokens)
        .map_err(|e| format!("failed to serialize tokens: {e}"))?;

    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
        use std::sync::atomic::{AtomicU64, Ordering};
        static TMP_SEQ: AtomicU64 = AtomicU64::new(0);
        let tmp = parent.join(format!(
            ".tokens.json.tmp-{}-{}",
            std::process::id(),
            TMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&tmp)
            .map_err(|e| format!("failed to create token store temp file: {e}"))?;
        let write_result = file
            .write_all(json.as_bytes())
            .and_then(|()| file.set_permissions(std::fs::Permissions::from_mode(0o600)))
            .and_then(|()| file.sync_all())
            .map_err(|e| format!("failed to write token store: {e}"))
            .and_then(|()| {
                std::fs::rename(&tmp, path)
                    .map_err(|e| format!("failed to persist token store: {e}"))
            });
        if write_result.is_err() {
            let _ = std::fs::remove_file(&tmp);
        }
        write_result
    }

    #[cfg(not(unix))]
    {
        // Non-Unix server builds are outside the supported surface; keep the
        // plain write rather than pretending NTFS ACL hardening exists here.
        std::fs::write(path, json).map_err(|e| format!("failed to write token store: {e}"))
    }
}

#[cfg(not(feature = "tauri-runtime"))]
pub fn set_token(account_id: &str, token: &str) -> Result<(), String> {
    change_token_at(&tokens_file_path(), token_key(account_id), Some(token))
}

#[cfg(not(feature = "tauri-runtime"))]
pub fn get_token(account_id: &str) -> Option<String> {
    read_tokens().get(&token_key(account_id)).cloned()
}

#[cfg(not(feature = "tauri-runtime"))]
pub fn delete_token(account_id: &str) -> Result<(), String> {
    change_token_at(&tokens_file_path(), token_key(account_id), None)
}

// ── Chat channel token helpers ──
// Reuse the same storage mechanism (keyring or file) with a different key prefix.

#[cfg(feature = "tauri-runtime")]
pub fn set_channel_token(channel_id: i32, token: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE_NAME, &channel_token_key(channel_id))
        .map_err(|e| format!("keyring init error: {e}"))?;
    entry
        .set_password(token)
        .map_err(|e| format!("keyring set error: {e}"))
}

#[cfg(feature = "tauri-runtime")]
pub fn get_channel_token(channel_id: i32) -> Option<String> {
    let entry = keyring::Entry::new(SERVICE_NAME, &channel_token_key(channel_id)).ok()?;
    entry.get_password().ok()
}

#[cfg(feature = "tauri-runtime")]
pub fn delete_channel_token(channel_id: i32) -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE_NAME, &channel_token_key(channel_id))
        .map_err(|e| format!("keyring init error: {e}"))?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("keyring delete error: {e}")),
    }
}

#[cfg(not(feature = "tauri-runtime"))]
pub fn set_channel_token(channel_id: i32, token: &str) -> Result<(), String> {
    change_token_at(
        &tokens_file_path(),
        channel_token_key(channel_id),
        Some(token),
    )
}

#[cfg(not(feature = "tauri-runtime"))]
pub fn get_channel_token(channel_id: i32) -> Option<String> {
    read_tokens().get(&channel_token_key(channel_id)).cloned()
}

#[cfg(not(feature = "tauri-runtime"))]
pub fn delete_channel_token(channel_id: i32) -> Result<(), String> {
    change_token_at(&tokens_file_path(), channel_token_key(channel_id), None)
}

#[cfg(all(test, not(feature = "tauri-runtime")))]
mod tests {
    use super::*;

    #[test]
    fn intake_store_mutation_preserves_malformed_and_unreadable_stores() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tokens.json");
        for bytes in [
            b"{\"unrelated\":\"synthetic\",broken".as_slice(),
            b"{\"unrelated\":42}",
            b"\xff",
        ] {
            std::fs::write(&path, bytes).unwrap();
            for value in [Some("synthetic-staged"), None] {
                assert!(change_token_at(&path, token_key("intake-staged"), value).is_err());
                assert_eq!(std::fs::read(&path).unwrap(), bytes);
                assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
            }
        }
        std::fs::remove_file(&path).unwrap();
        std::fs::create_dir(&path).unwrap();
        let retained = path.join("retained");
        std::fs::write(&retained, b"synthetic-preserved").unwrap();
        for value in [Some("synthetic-staged"), None] {
            assert!(change_token_at(&path, token_key("intake-staged"), value).is_err());
            assert_eq!(std::fs::read(&retained).unwrap(), b"synthetic-preserved");
        }
    }

    #[test]
    fn intake_store_missing_file_and_staged_cleanup_preserve_other_entries() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tokens.json");
        change_token_at(&path, token_key("unrelated"), Some("synthetic-original")).unwrap();
        change_token_at(&path, token_key("intake-staged"), Some("synthetic-staged")).unwrap();
        change_token_at(&path, token_key("intake-staged"), None).unwrap();
        let tokens = read_tokens_for_write(&path).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[&token_key("unrelated")], "synthetic-original");
        #[cfg(unix)]
        assert_eq!(mode_bits(&path), 0o600);
    }

    #[test]
    fn concurrent_inbox_and_channel_updates_preserve_every_credential() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tokens.json");
        std::thread::scope(|scope| {
            for id in 0..16 {
                let path = &path;
                scope.spawn(move || {
                    change_token_at(
                        path,
                        token_key(&format!("ops-resend:{id}")),
                        Some("synthetic-inbox-key"),
                    )
                    .unwrap();
                    change_token_at(path, channel_token_key(id), Some("synthetic-channel-key"))
                        .unwrap();
                    change_token_at(path, channel_token_key(id), None).unwrap();
                });
            }
        });
        let result = read_tokens_at(&path);
        assert_eq!(result.len(), 16);
        for id in 0..16 {
            assert_eq!(
                result[&token_key(&format!("ops-resend:{id}"))],
                "synthetic-inbox-key"
            );
        }
        #[cfg(unix)]
        assert_eq!(mode_bits(&path), 0o600);
    }

    #[test]
    fn test_tokens_file_path_absolutizes_relative_env() {
        // Regression: a relative `CODEG_DATA_DIR=data` previously made
        // `tokens.json` resolve against the helper subprocess's CWD (i.e.
        // git's repo dir), even after we'd absolutized the path used for
        // the database. The token store must always land on an absolute
        // path so DB lookup and token lookup point at the same root.
        let cwd = std::env::current_dir().expect("cwd");
        let resolved = tokens_file_path_for(Some("data"));
        assert!(
            resolved.is_absolute(),
            "tokens path must be absolute, got: {}",
            resolved.display()
        );
        assert_eq!(resolved, cwd.join("data").join("tokens.json"));
    }

    #[test]
    fn test_tokens_file_path_absolute_env_unchanged() {
        let data_dir = std::env::current_dir().expect("cwd").join("codeg-data");
        let data_dir_str = data_dir.to_string_lossy().to_string();
        let resolved = tokens_file_path_for(Some(&data_dir_str));
        assert_eq!(resolved, data_dir.join("tokens.json"));
    }

    #[test]
    fn test_tokens_file_path_default_when_unset() {
        // No env var → derived from `dirs::data_dir()` (always absolute on
        // every platform we ship to). Just verify we end at `tokens.json`
        // and that the result is absolute, not the literal default.
        let resolved = tokens_file_path_for(None);
        assert!(resolved.is_absolute());
        assert!(resolved.ends_with("tokens.json"));
    }

    #[cfg(unix)]
    fn mode_bits(path: &std::path::Path) -> u32 {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path)
            .expect("metadata")
            .permissions()
            .mode()
            & 0o777
    }

    /// A fresh store must be 0600 from its very first byte on disk — there is
    /// no window where a parallel reader could see a wide-permission file,
    /// because the temp file is created with the final mode and only then
    /// renamed into place.
    #[test]
    #[cfg(unix)]
    fn test_write_tokens_creates_0600() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("tokens.json");
        let mut tokens = std::collections::HashMap::new();
        tokens.insert("github-token:a".to_string(), "secret".to_string());
        write_tokens_at(&path, &tokens).expect("write");
        assert_eq!(mode_bits(&path), 0o600);
        assert_eq!(
            read_tokens_at(&path).get("github-token:a").unwrap(),
            "secret"
        );
        // No temp residue left behind.
        let leftovers: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with(".tokens.json.tmp")
            })
            .collect();
        assert!(leftovers.is_empty(), "temp files must not survive a write");
    }

    /// Overwriting a legacy wide-permission store must end 0600: the rename
    /// replaces the inode, so the old 0644 bits die with the old file.
    #[test]
    #[cfg(unix)]
    fn test_write_tokens_replaces_legacy_wide_file_with_0600() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("tokens.json");
        std::fs::write(&path, "{}").expect("seed legacy file");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        let mut tokens = std::collections::HashMap::new();
        tokens.insert("github-token:b".to_string(), "s2".to_string());
        write_tokens_at(&path, &tokens).expect("write");
        assert_eq!(mode_bits(&path), 0o600);
    }

    /// Reading an existing legacy store tightens it to 0600 before the bytes
    /// are consumed, so a server that only ever reads (never re-saves) still
    /// heals the volume-mounted file.
    #[test]
    #[cfg(unix)]
    fn test_read_tokens_tightens_existing_file() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("tokens.json");
        std::fs::write(&path, r#"{"github-token:c":"s3"}"#).expect("seed");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        let tokens = read_tokens_at(&path);
        assert_eq!(tokens.get("github-token:c").unwrap(), "s3");
        assert_eq!(mode_bits(&path), 0o600);
    }
}
