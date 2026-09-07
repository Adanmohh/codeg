//! Per-launch Pi assets and credentials. No project/global config is written.
//! Codeg companion launch/cache pattern and pi-acp's executable override;
//! immutable upstream mappings and licenses are in NOTICE.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::delegation::listener::{TokenEntry, TokenRegistry};
use crate::acp::error::AcpError;

const ASSETS: &[(&str, &str)] = &[
    (
        "index.ts",
        include_str!("../../../integrations/pi-desk/index.ts"),
    ),
    (
        "broker.ts",
        include_str!("../../../integrations/pi-desk/broker.ts"),
    ),
    (
        "protocol.ts",
        include_str!("../../../integrations/pi-desk/protocol.ts"),
    ),
    (
        "transport.ts",
        include_str!("../../../integrations/pi-desk/transport.ts"),
    ),
    (
        "launch.mjs",
        include_str!("../../../integrations/pi-desk/launch.mjs"),
    ),
    (
        "LICENSE",
        include_str!("../../../integrations/pi-desk/LICENSE"),
    ),
];

pub(crate) struct PreparedLaunch {
    pub directory: PathBuf,
    pub env: BTreeMap<String, String>,
    token: Option<(Arc<TokenRegistry>, String, tokio::runtime::Handle)>,
}

impl PreparedLaunch {
    pub async fn bind_token(
        &mut self,
        registry: Arc<TokenRegistry>,
        parent: &str,
        socket: &Path,
        cwd: &Path,
    ) -> String {
        let token = uuid::Uuid::new_v4().to_string();
        self.token = Some((
            registry.clone(),
            token.clone(),
            tokio::runtime::Handle::current(),
        ));
        registry
            .register(
                token.clone(),
                TokenEntry {
                    parent_connection_id: parent.to_owned(),
                    working_dir: cwd.to_owned(),
                },
            )
            .await;
        self.env.insert("CODEG_DESK_TOKEN".into(), token.clone());
        self.env.insert(
            "CODEG_DESK_SOCKET".into(),
            socket.to_string_lossy().into_owned(),
        );
        token
    }
}

impl Drop for PreparedLaunch {
    fn drop(&mut self) {
        // Also runs for a cancelled spawn future, driver panic or thread-spawn
        // failure. The manager's live-connection check closes the teardown gap.
        if let Some((registry, token, runtime)) = self.token.take() {
            runtime.spawn(async move {
                registry.revoke(&token).await;
            });
        }
        // Only the unique directory we created; never a caller path/config.
        let _ = std::fs::remove_dir_all(&self.directory);
    }
}

fn setup_error() -> AcpError {
    AcpError::SdkNotInstalled(
        "Pi Desk is not installed or configured: pi 0.85.1, pi-mcp-adapter 2.32.1, \
         and a configured gpt-6-astra catalogue entry with max reasoning are required. \
         Use the existing Pi client configuration; no model fallback or login was attempted."
            .into(),
    )
}

fn write_assets(root: &Path) -> Result<PathBuf, AcpError> {
    let directory = root.join(uuid::Uuid::new_v4().to_string());
    std::fs::create_dir_all(&directory).map_err(|_| setup_error())?;
    for (name, text) in ASSETS {
        if std::fs::write(directory.join(name), text).is_err() {
            let _ = std::fs::remove_dir_all(&directory);
            return Err(setup_error());
        }
    }
    Ok(directory)
}

/// Resolve existing installed clients. Probe prints only package metadata and
/// model availability, never keys. No dependency install or model request.
pub(crate) async fn prepare(
    runtime_env: &BTreeMap<String, String>,
    companion: &Path,
    cwd: &Path,
) -> Result<PreparedLaunch, AcpError> {
    let root = super::binary_cache::cache_dir()?.join("pi-desk-launches");
    prepare_at(runtime_env, companion, cwd, &root).await
}

async fn prepare_at(
    runtime_env: &BTreeMap<String, String>,
    companion: &Path,
    cwd: &Path,
    root: &Path,
) -> Result<PreparedLaunch, AcpError> {
    let original_pi = runtime_env
        .get("PI_ACP_PI_COMMAND")
        .map(String::as_str)
        .unwrap_or("pi");
    let pi = crate::commands::acp::resolve_pi_command_path(original_pi).ok_or_else(setup_error)?;
    let mut prepared = PreparedLaunch {
        directory: write_assets(root)?,
        env: runtime_env.clone(),
        token: None,
    };
    let script = prepared.directory.join("launch.mjs");
    let node = crate::commands::acp::resolve_pi_command_path("node").ok_or_else(setup_error)?;
    let mut probe = tokio::process::Command::new(&node);
    probe
        .arg(&script)
        .arg("--probe")
        .envs(runtime_env)
        .env("CODEG_DESK_REAL_PI", &pi)
        .env_remove("CODEG_TOKEN")
        .current_dir(cwd)
        .kill_on_drop(true);
    let result = tokio::time::timeout(std::time::Duration::from_secs(15), probe.output())
        .await
        .map_err(|_| setup_error())?
        .map_err(|_| setup_error())?;
    let output: serde_json::Value =
        serde_json::from_slice(&result.stdout).map_err(|_| setup_error())?;
    if !result.status.success() || output["modelAvailable"] != true {
        return Err(setup_error());
    }
    // Override caller/inherited launch plumbing with backend-created values.
    prepared
        .env
        .retain(|key, _| !key.starts_with("CODEG_DESK_") && key != "CODEG_PI_DESK_LAUNCH");
    prepared.env.insert(
        "PI_ACP_PI_COMMAND".into(),
        companion.to_string_lossy().into_owned(),
    );
    prepared
        .env
        .insert("CODEG_PI_DESK_LAUNCH".into(), "1".into());
    prepared.env.insert(
        "CODEG_DESK_LAUNCH_SCRIPT".into(),
        script.to_string_lossy().into_owned(),
    );
    prepared.env.insert(
        "CODEG_DESK_NODE".into(),
        node.to_string_lossy().into_owned(),
    );
    prepared.env.insert(
        "CODEG_DESK_REAL_PI".into(),
        pi.to_string_lossy().into_owned(),
    );
    // Empty values are removals in pinned vendor/sacp-tokio. Never inherit the
    // operator bearer, even if an agent runtime override supplied one.
    prepared.env.insert("CODEG_TOKEN".into(), String::new());
    Ok(prepared)
}

/// codeg-mcp doubles as pi-acp's executable trampoline, avoiding shell scripts
/// or another binary/registry. Its normal MCP mode is unchanged.
pub fn launch_from_companion() -> std::process::ExitCode {
    let (Some(node), Some(script)) = (
        std::env::var_os("CODEG_DESK_NODE"),
        std::env::var_os("CODEG_DESK_LAUNCH_SCRIPT"),
    ) else {
        return std::process::ExitCode::from(2);
    };
    match std::process::Command::new(node)
        .arg(script)
        .args(std::env::args_os().skip(1))
        .env_remove("CODEG_PI_DESK_LAUNCH")
        .env_remove("CODEG_TOKEN")
        .status()
    {
        Ok(status) => std::process::ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(_) => std::process::ExitCode::from(2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Requires existing pinned clients. No installation, global config or model
    /// call. The model below is synthetic local catalogue metadata, not a claim
    /// that the owner's actual Astra credentials/catalogue have been configured.
    #[tokio::test]
    #[ignore = "requires installed pi 0.85.1 and pi-mcp-adapter 2.32.1; run explicitly"]
    async fn pi_desk_extracted_assets_real_rpc_discovery() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/pi-desk-fixtures");
        std::fs::create_dir_all(&root).unwrap();
        let temp = tempfile::tempdir_in(root).unwrap();
        let agent = temp.path().join("agent");
        std::fs::create_dir_all(&agent).unwrap();
        let sink = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        std::fs::write(
            agent.join("models.json"),
            serde_json::to_vec(&serde_json::json!({
                "providers": { "desk-fixture": {
                    "baseUrl": format!("http://{}/v1", sink.local_addr().unwrap()),
                    "api": "openai-completions", "apiKey": "synthetic-fixture-only",
                    "models": [{ "id": "gpt-6-astra", "reasoning": true,
                        "thinkingLevelMap": { "max": "max" } }]
                }}
            }))
            .unwrap(),
        )
        .unwrap();
        let runtime = BTreeMap::from([
            (
                "PI_CODING_AGENT_DIR".into(),
                agent.to_string_lossy().into_owned(),
            ),
            ("PI_OFFLINE".into(), "1".into()),
        ]);
        let mut prepared = prepare_at(
            &runtime,
            &temp.path().join("fixture-companion"),
            temp.path(),
            &temp.path().join("assets"),
        )
        .await
        .expect("local installed packages and fixture catalogue");
        prepared
            .bind_token(
                Arc::new(TokenRegistry::default()),
                "fixture-parent",
                &temp.path().join("socket"),
                temp.path(),
            )
            .await;
        let mut child = tokio::process::Command::new(&prepared.env["CODEG_DESK_NODE"])
            .arg(&prepared.env["CODEG_DESK_LAUNCH_SCRIPT"])
            .args(["--mode", "rpc", "--no-themes"])
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .envs(&prepared.env)
            .current_dir(temp.path())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        let mut output = BufReader::new(child.stdout.take().unwrap()).lines();
        input
            .write_all(b"{\"id\":\"fixture-discovery\",\"type\":\"get_commands\"}\n")
            .await
            .unwrap();
        let result = tokio::time::timeout(std::time::Duration::from_secs(15), async {
            loop {
                let line = output
                    .next_line()
                    .await
                    .unwrap()
                    .expect("wrapper stays live");
                let response: serde_json::Value = serde_json::from_str(&line).unwrap();
                assert_ne!(
                    response["type"], "message_start",
                    "no model turn is allowed"
                );
                if response["id"] == "fixture-discovery" {
                    break response;
                }
            }
        })
        .await
        .unwrap();
        assert_eq!(result["success"], true);
        let commands = result["data"]["commands"].as_array().unwrap();
        for name in ["desk-status", "mcp"] {
            assert!(commands
                .iter()
                .any(|command| command["name"] == name && command["source"] == "extension"));
        }
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(30), sink.accept())
                .await
                .is_err(),
            "catalogue/discovery must never contact even the fixture provider"
        );
        drop(input);
        tokio::time::timeout(std::time::Duration::from_secs(10), child.wait())
            .await
            .expect("RPC EOF terminates wrapper and Pi")
            .unwrap();
        // Empty isolated catalogue is a setup failure, never an implicit fallback.
        std::fs::remove_file(agent.join("models.json")).unwrap();
        assert!(prepare_at(
            &runtime,
            Path::new("fixture-companion"),
            temp.path(),
            &temp.path().join("missing-model-assets")
        )
        .await
        .is_err());
    }

    #[test]
    fn pi_desk_assets_are_embedded_and_each_launch_owns_its_directory() {
        let temp = tempfile::tempdir().unwrap();
        let first = write_assets(temp.path()).unwrap();
        let second = write_assets(temp.path()).unwrap();
        assert_ne!(first, second);
        assert!(std::fs::read_to_string(first.join("index.ts"))
            .unwrap()
            .contains("installDesk"));
        let guard = PreparedLaunch {
            directory: first.clone(),
            env: BTreeMap::new(),
            token: None,
        };
        drop(guard);
        assert!(!first.exists());
        assert!(second.join("launch.mjs").exists());
    }

    #[tokio::test]
    async fn pi_desk_dropped_launch_revokes_its_token() {
        let temp = tempfile::tempdir().unwrap();
        let registry = Arc::new(TokenRegistry::default());
        let mut launch = PreparedLaunch {
            directory: write_assets(temp.path()).unwrap(),
            env: BTreeMap::new(),
            token: None,
        };
        let token = launch
            .bind_token(
                registry.clone(),
                "launch-parent",
                &temp.path().join("socket"),
                temp.path(),
            )
            .await;
        assert!(registry.lookup(&token).await.is_some());
        drop(launch);
        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            while registry.lookup(&token).await.is_some() {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("teardown revokes without a running model process");
    }
}
