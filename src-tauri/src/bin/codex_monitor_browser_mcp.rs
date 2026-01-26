use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot, Mutex};

const SERVER_NAME: &str = "codex-monitor-browser";
const SERVER_VERSION: &str = "0.1.0";

type PendingMap = HashMap<u64, oneshot::Sender<Result<Value, String>>>;

#[derive(Clone)]
struct DaemonClient {
    out_tx: mpsc::UnboundedSender<String>,
    pending: Arc<Mutex<PendingMap>>,
    next_id: Arc<AtomicU64>,
}

impl DaemonClient {
    async fn connect(addr: &str, token: Option<String>) -> Result<Self, String> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|e| format!("Failed to connect to daemon {addr}: {e}"))?;
        let (reader, writer) = stream.into_split();
        let (out_tx, mut out_rx) = mpsc::unbounded_channel::<String>();
        let pending = Arc::new(Mutex::new(PendingMap::new()));
        let pending_reader = Arc::clone(&pending);
        let pending_writer = Arc::clone(&pending);

        tokio::spawn(async move {
            let mut writer = BufWriter::new(writer);
            while let Some(message) = out_rx.recv().await {
                if writer.write_all(message.as_bytes()).await.is_err()
                    || writer.write_all(b"\n").await.is_err()
                {
                    let mut pending = pending_writer.lock().await;
                    for (_, sender) in pending.drain() {
                        let _ = sender.send(Err("daemon disconnected".to_string()));
                    }
                    break;
                }
                let _ = writer.flush().await;
            }
        });

        tokio::spawn(async move {
            let mut lines = BufReader::new(reader).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if line.trim().is_empty() {
                    continue;
                }
                let value: Value = match serde_json::from_str(&line) {
                    Ok(value) => value,
                    Err(_) => continue,
                };
                let id = value.get("id").and_then(|v| v.as_u64());
                if let Some(id) = id {
                    if let Some(tx) = pending_reader.lock().await.remove(&id) {
                        if let Some(error) = value.get("error") {
                            let msg = error
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("daemon error");
                            let _ = tx.send(Err(msg.to_string()));
                        } else {
                            let _ = tx.send(Ok(value.get("result").cloned().unwrap_or(Value::Null)));
                        }
                    }
                }
            }
        });

        let client = DaemonClient {
            out_tx,
            pending,
            next_id: Arc::new(AtomicU64::new(1)),
        };

        if let Some(token) = token {
            client.call("auth", json!({ "token": token })).await?;
        }

        Ok(client)
    }

    async fn call(&self, method: &str, params: Value) -> Result<Value, String> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);

        let payload = json!({ "id": id, "method": method, "params": params });
        let message = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
        if self.out_tx.send(message).is_err() {
            self.pending.lock().await.remove(&id);
            return Err("daemon disconnected".to_string());
        }

        rx.await.map_err(|_| "daemon disconnected".to_string())?
    }
}

fn initialize_result() -> Value {
    json!({
        "protocolVersion": "2025-11-25",
        "serverInfo": { "name": SERVER_NAME, "version": SERVER_VERSION },
        "capabilities": {
            "tools": { "listChanged": false },
            "resources": { "listChanged": false },
            "prompts": { "listChanged": false }
        }
    })
}

fn tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "browser_create_session",
            "description": "Create a browser session.",
            "inputSchema": { "type": "object", "properties": { "headless": { "type": "boolean" }, "viewport": { "type": "object" }, "userDataDir": { "type": "string" }, "startUrl": { "type": "string" } } }
        }),
        json!({
            "name": "browser_list_sessions",
            "description": "List browser sessions.",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "browser_close_session",
            "description": "Close a browser session.",
            "inputSchema": { "type": "object", "properties": { "sessionId": { "type": "string" } }, "required": ["sessionId"] }
        }),
        json!({
            "name": "browser_navigate",
            "description": "Navigate to a URL.",
            "inputSchema": { "type": "object", "properties": { "sessionId": { "type": "string" }, "url": { "type": "string" }, "waitUntil": { "type": "string" }, "timeoutMs": { "type": "number" } }, "required": ["sessionId", "url"] }
        }),
        json!({
            "name": "browser_screenshot",
            "description": "Capture a screenshot.",
            "inputSchema": { "type": "object", "properties": { "sessionId": { "type": "string" }, "fullPage": { "type": "boolean" } }, "required": ["sessionId"] }
        }),
        json!({
            "name": "browser_click",
            "description": "Click by selector or coordinates.",
            "inputSchema": { "type": "object", "properties": { "sessionId": { "type": "string" }, "selector": { "type": "string" }, "x": { "type": "number" }, "y": { "type": "number" } }, "required": ["sessionId"] }
        }),
        json!({
            "name": "browser_type",
            "description": "Type into an element.",
            "inputSchema": { "type": "object", "properties": { "sessionId": { "type": "string" }, "selector": { "type": "string" }, "text": { "type": "string" }, "clearFirst": { "type": "boolean" } }, "required": ["sessionId", "selector"] }
        }),
        json!({
            "name": "browser_press",
            "description": "Press a keyboard key.",
            "inputSchema": { "type": "object", "properties": { "sessionId": { "type": "string" }, "key": { "type": "string" } }, "required": ["sessionId"] }
        }),
        json!({
            "name": "browser_evaluate",
            "description": "Evaluate JavaScript in the page.",
            "inputSchema": { "type": "object", "properties": { "sessionId": { "type": "string" }, "js": { "type": "string" } }, "required": ["sessionId", "js"] }
        }),
        json!({
            "name": "browser_snapshot",
            "description": "Get screenshot + simplified DOM list.",
            "inputSchema": { "type": "object", "properties": { "sessionId": { "type": "string" }, "fullPage": { "type": "boolean" } }, "required": ["sessionId"] }
        })
    ]
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = env::var("CODEX_MONITOR_DAEMON_ADDR").unwrap_or_else(|_| "127.0.0.1:4732".to_string());
    let token = env::var("CODEX_MONITOR_DAEMON_TOKEN").ok();
    let client = match DaemonClient::connect(&addr, token).await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to connect to daemon: {err}");
            return;
        }
    };

    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let stdout = tokio::io::stdout();
    let mut writer = BufWriter::new(stdout);

    while let Ok(Some(line)) = lines.next_line().await {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let message: Value = match serde_json::from_str(trimmed) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Failed to parse MCP message: {err}");
                continue;
            }
        };
        let id = message.get("id").cloned();
        let method = message
            .get("method")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        let params = message.get("params").cloned().unwrap_or(Value::Null);

        let response = match method {
            "initialize" => id.map(|id| build_result(&id, initialize_result())),
            "tools/list" => id.map(|id| build_result(&id, json!({ "tools": tool_definitions() }))),
            "tools/call" => {
                if let Some(id) = id {
                    let result = handle_tool_call(&client, params).await;
                    Some(match result {
                        Ok(value) => build_result(&id, value),
                        Err(err) => build_error(&id, -32602, &err),
                    })
                } else {
                    None
                }
            }
            "resources/list" => id.map(|id| build_result(&id, json!({ "resources": [] }))),
            "prompts/list" => id.map(|id| build_result(&id, json!({ "prompts": [] }))),
            "initialized" => None,
            "ping" => id.map(|id| build_result(&id, json!({ "ok": true }))),
            _ => id.map(|id| build_error(&id, -32601, &format!("Unknown method: {method}"))),
        };

        if let Some(payload) = response {
            if let Ok(serialized) = serde_json::to_string(&payload) {
                let _ = writer.write_all(serialized.as_bytes()).await;
                let _ = writer.write_all(b"\n").await;
                let _ = writer.flush().await;
            }
        }
    }
}

async fn handle_tool_call(client: &DaemonClient, params: Value) -> Result<Value, String> {
    let params_obj = params.as_object().ok_or("Missing params")?;
    let tool_name = params_obj
        .get("name")
        .and_then(|value| value.as_str())
        .ok_or("Missing tool name")?;
    let args = params_obj.get("arguments").cloned().unwrap_or(Value::Null);

    let method = match tool_name {
        "browser_create_session" => "browser_create_session",
        "browser_list_sessions" => "browser_list_sessions",
        "browser_close_session" => "browser_close_session",
        "browser_navigate" => "browser_navigate",
        "browser_screenshot" => "browser_screenshot",
        "browser_click" => "browser_click",
        "browser_type" => "browser_type",
        "browser_press" => "browser_press",
        "browser_evaluate" => "browser_evaluate",
        "browser_snapshot" => "browser_snapshot",
        _ => return Err(format!("Unknown tool: {tool_name}")),
    };

    let result = client.call(method, args).await?;
    Ok(tool_text_response(result))
}

fn tool_text_response(result: Value) -> Value {
    json!({
        "content": [
            {
                "type": "text",
                "text": result.to_string()
            }
        ]
    })
}

fn build_result(id: &Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    })
}

fn build_error(id: &Value, code: i64, message: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": { "code": code, "message": message }
    })
}
