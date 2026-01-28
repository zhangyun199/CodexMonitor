use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{mpsc, oneshot, Mutex};

#[derive(Clone)]
pub struct BrowserService {
    worker: Arc<Mutex<Option<BrowserWorkerClient>>>,
}

impl BrowserService {
    pub fn new() -> Self {
        Self {
            worker: Arc::new(Mutex::new(None)),
        }
    }

    async fn ensure_worker(&self) -> Result<BrowserWorkerClient, String> {
        let mut guard = self.worker.lock().await;
        if let Some(worker) = guard.clone() {
            return Ok(worker);
        }
        let worker = BrowserWorkerClient::spawn().await?;
        *guard = Some(worker.clone());
        Ok(worker)
    }

    pub async fn request(&self, method: &str, params: Value) -> Result<Value, String> {
        let worker = self.ensure_worker().await?;
        worker.send_request(method, params).await
    }
}

#[derive(Clone)]
struct BrowserWorkerClient {
    child: Arc<Mutex<Child>>,
    stdin: Arc<Mutex<ChildStdin>>,
    pending: Arc<Mutex<HashMap<String, oneshot::Sender<Value>>>>,
    next_id: Arc<AtomicU64>,
}

impl BrowserWorkerClient {
    async fn spawn() -> Result<Self, String> {
        let worker_path = env::var("CODEX_MONITOR_BROWSER_WORKER")
            .unwrap_or_else(|_| "browser-worker/dist/index.js".to_string());
        let mut cmd = Command::new("node");
        cmd.arg(worker_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit());

        let mut child = cmd.spawn().map_err(|e| e.to_string())?;
        let stdin = child.stdin.take().ok_or("missing worker stdin")?;
        let stdout = child.stdout.take().ok_or("missing worker stdout")?;

        let client = BrowserWorkerClient {
            child: Arc::new(Mutex::new(child)),
            stdin: Arc::new(Mutex::new(stdin)),
            pending: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        };

        let pending = Arc::clone(&client.pending);
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if line.trim().is_empty() {
                    continue;
                }
                let value: Value = match serde_json::from_str(&line) {
                    Ok(value) => value,
                    Err(_) => continue,
                };
                if let Some(id) = value.get("id").and_then(|v| v.as_str()) {
                    if let Some(tx) = pending.lock().await.remove(id) {
                        let _ = tx.send(value);
                    }
                }
            }
        });

        Ok(client)
    }

    async fn send_request(&self, method: &str, params: Value) -> Result<Value, String> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst).to_string();
        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id.clone(), tx);

        let payload = json!({
            "id": id,
            "method": method,
            "params": params
        });
        let mut stdin = self.stdin.lock().await;
        let mut line = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        line.push('\n');
        stdin
            .write_all(line.as_bytes())
            .await
            .map_err(|e| e.to_string())?;

        let response = rx.await.map_err(|_| "worker request canceled")?;
        if let Some(error) = response.get("error") {
            return Err(error
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("worker error")
                .to_string());
        }
        Ok(response.get("result").cloned().unwrap_or(Value::Null))
    }
}
