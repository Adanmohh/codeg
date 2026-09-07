//! Closed child-process glue. Tokio 1.53.1 kill-on-drop/timeout/bounded IO.
use super::types::HostError;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{path::PathBuf, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout},
};

const MAX_RESPONSE: u64 = 2 * 1024 * 1024;

pub(super) struct Bridge {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Reply<T> {
    request_id: Option<u64>,
    result: AdapterResult<T>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdapterResult<T> {
    status: String,
    value: Option<T>,
    error: Option<AdapterError>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdapterError {
    code: String,
    message: String,
}

pub(super) fn python() -> PathBuf {
    // Host configuration only; never accept an executable or args in UI JSON.
    std::env::var_os("CODEG_INTAKE_PYTHON")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../integrations/hafidh-intake/.venv/bin/python")
        })
}
impl Bridge {
    pub fn start(origin: &str, product: &str, bearer: &str) -> Result<Self, HostError> {
        let executable = python();
        if !executable.is_absolute() || !executable.is_file() {
            return Err(HostError::AdapterMissing);
        }
        let mut child = crate::process::tokio_command(executable)
            .args(["-I", "-m", "hafidh_intake.host"])
            .env_clear()
            .env("HAFIDH_INTAKE_ORIGIN", origin)
            .env("HAFIDH_INTAKE_PRODUCT_ID", product)
            .env("HAFIDH_INTAKE_BEARER", bearer)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .map_err(|_| HostError::AdapterMissing)?;
        let stdin = child.stdin.take().ok_or(HostError::SourceUnavailable)?;
        let stdout = BufReader::new(child.stdout.take().ok_or(HostError::SourceUnavailable)?);
        Ok(Self {
            child,
            stdin,
            stdout,
            next: 0,
        })
    }
    pub async fn read<T: DeserializeOwned>(
        &mut self,
        operation: &str,
        input: Value,
    ) -> Result<T, HostError> {
        self.next += 1;
        let id = self.next;
        let mut bytes =
            serde_json::to_vec(&json!({"request_id": id, "operation": operation, "input": input}))
                .map_err(|_| HostError::InvalidInput)?;
        if bytes.len() >= 16 * 1024 {
            return Err(HostError::InvalidInput);
        }
        bytes.push(b'\n');
        let result = tokio::time::timeout(Duration::from_secs(45), async {
            self.stdin
                .write_all(&bytes)
                .await
                .map_err(|_| HostError::SourceUnavailable)?;
            self.stdin
                .flush()
                .await
                .map_err(|_| HostError::SourceUnavailable)?;
            let mut out = vec![];
            // Limit applies before allocating, including unterminated malicious output.
            (&mut self.stdout)
                .take(MAX_RESPONSE + 1)
                .read_until(b'\n', &mut out)
                .await
                .map_err(|_| HostError::SourceUnavailable)?;
            if out.len() as u64 > MAX_RESPONSE || !out.ends_with(b"\n") {
                return Err(HostError::SourceUnavailable);
            }
            let reply: Reply<T> =
                serde_json::from_slice(&out).map_err(|_| HostError::SourceUnavailable)?;
            if reply.request_id != Some(id) {
                return Err(HostError::SourceUnavailable);
            }
            if reply.result.status == "ok" && reply.result.error.is_none() {
                return reply.result.value.ok_or(HostError::SourceUnavailable);
            }
            // Only fixed codes cross into UI; do not propagate adapter messages.
            Err(match reply.result.error {
                Some(e) if e.code == "not_configured" => HostError::NotConfigured,
                Some(e) if e.code == "access_denied" => HostError::AccessDenied,
                _ => HostError::SourceUnavailable,
            })
        })
        .await
        .unwrap_or(Err(HostError::SourceUnavailable));
        if result.is_err() {
            let _ = self.child.kill().await;
        }
        result
    }
}
