#[path = "../memory/mod.rs"]
mod memory;

use memory::supabase::{MemoryEntry, MemorySearchResult};
use memory::MemoryService;
use serde_json::{json, Value};
use std::env;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

const SERVER_NAME: &str = "codex-monitor-memory";
const SERVER_VERSION: &str = "0.1.0";

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    runtime.block_on(async {
        let supabase_url = env::var("SUPABASE_URL").unwrap_or_default();
        let supabase_anon_key = env::var("SUPABASE_ANON_KEY").unwrap_or_default();
        let minimax_api_key = env::var("MINIMAX_API_KEY").unwrap_or_default();

        let enabled = !supabase_url.is_empty() && !supabase_anon_key.is_empty();
        let memory = MemoryService::new(
            &supabase_url,
            &supabase_anon_key,
            if minimax_api_key.is_empty() {
                None
            } else {
                Some(minimax_api_key.as_str())
            },
            enabled,
        );

        eprintln!(
            "codex-monitor-memory-mcp running (enabled={}, embeddings={})",
            enabled,
            !minimax_api_key.is_empty()
        );

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
                "tools/list" => {
                    id.map(|id| build_result(&id, json!({ "tools": tool_definitions() })))
                }
                "tools/call" => {
                    if let Some(id) = id {
                        let result = handle_tool_call(&memory, params).await;
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
    });
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
            "name": "memory_bootstrap",
            "description": "Get recent curated + daily memory for context.",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "memory_search",
            "description": "Search memory (semantic if embeddings enabled; text fallback).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" },
                    "limit": { "type": "number", "description": "Max results (default 10)" }
                },
                "required": ["query"]
            }
        }),
        json!({
            "name": "memory_append",
            "description": "Append a memory entry.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "type": { "type": "string", "description": "daily | curated" },
                    "content": { "type": "string", "description": "Memory content" },
                    "tags": { "type": "array", "items": { "type": "string" } },
                    "workspace_id": { "type": "string" }
                },
                "required": ["content"]
            }
        }),
    ]
}

async fn handle_tool_call(memory: &MemoryService, params: Value) -> Result<Value, String> {
    let params_obj = params.as_object().ok_or("Missing params")?;
    let tool_name = params_obj
        .get("name")
        .and_then(|value| value.as_str())
        .ok_or("Missing tool name")?;
    let args = params_obj.get("arguments").cloned().unwrap_or(Value::Null);

    match tool_name {
        "memory_bootstrap" => {
            let results = memory.bootstrap().await?;
            let text = format_bootstrap(&results);
            Ok(tool_text_response(text))
        }
        "memory_search" => {
            let query = get_string_arg(&args, "query").ok_or("Missing query")?;
            let limit = get_number_arg(&args, "limit").unwrap_or(10).clamp(1, 50) as usize;
            let results = memory.search(&query, limit).await?;
            let text = format_search(&query, &results);
            Ok(tool_text_response(text))
        }
        "memory_append" => {
            let content = get_string_arg(&args, "content").ok_or("Missing content")?;
            let memory_type = get_string_arg(&args, "type").unwrap_or_else(|| "daily".to_string());
            let tags = get_string_array_arg(&args, "tags");
            let workspace_id = get_string_arg(&args, "workspace_id");
            let entry = memory
                .append(&memory_type, &content, tags, workspace_id)
                .await?;
            let text = format_append(&entry);
            Ok(tool_text_response(text))
        }
        _ => Err(format!("Unknown tool: {tool_name}")),
    }
}

fn tool_text_response(text: String) -> Value {
    json!({
        "content": [
            {
                "type": "text",
                "text": text
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

fn build_error(id: &Value, code: i32, message: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message
        }
    })
}

fn get_string_arg(args: &Value, key: &str) -> Option<String> {
    args.as_object()
        .and_then(|map| map.get(key))
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn get_number_arg(args: &Value, key: &str) -> Option<i64> {
    args.as_object()
        .and_then(|map| map.get(key))
        .and_then(|value| value.as_i64().or_else(|| value.as_f64().map(|v| v as i64)))
}

fn get_string_array_arg(args: &Value, key: &str) -> Vec<String> {
    args.as_object()
        .and_then(|map| map.get(key))
        .and_then(|value| value.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(|value| value.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn format_search(query: &str, results: &[MemorySearchResult]) -> String {
    if results.is_empty() {
        return format!("No memory found for \"{}\"", query);
    }

    let mut out = format!("🔎 {} result(s) for \"{}\":\n\n", results.len(), query);
    for entry in results {
        let tag_str = if entry.tags.is_empty() {
            String::new()
        } else {
            format!("[{}] ", entry.tags.join(", "))
        };
        let line1 = format!("• {}{}", tag_str, preview(&entry.content, 140));
        let score = format_score(entry.score, entry.rank, entry.distance);
        let line2 = format!("  type={} {} id={}", entry.memory_type, score, entry.id);
        out.push_str(&line1);
        out.push('\n');
        out.push_str(&line2);
        out.push_str("\n\n");
    }
    out.trim().to_string()
}

fn format_bootstrap(results: &[MemorySearchResult]) -> String {
    if results.is_empty() {
        return "No memory entries available.".to_string();
    }

    let mut out = format!("🧠 Bootstrap memory ({} entries):\n\n", results.len());
    for entry in results {
        let tag_str = if entry.tags.is_empty() {
            String::new()
        } else {
            format!("[{}] ", entry.tags.join(", "))
        };
        out.push_str(&format!(
            "• {}{} ({})\n",
            tag_str,
            preview(&entry.content, 120),
            entry.memory_type
        ));
    }
    out.trim().to_string()
}

fn format_append(entry: &MemoryEntry) -> String {
    let id = entry.id.clone().unwrap_or_else(|| "(pending)".to_string());
    let status = entry
        .embedding_status
        .clone()
        .unwrap_or_else(|| "pending".to_string());
    format!("✅ Memory saved (id={}, status={})", id, status)
}

fn format_score(score: Option<f64>, rank: Option<f32>, distance: Option<f64>) -> String {
    if let Some(score) = score {
        return format!("score={:.3}", score);
    }
    if let Some(rank) = rank {
        return format!("rank={:.3}", rank);
    }
    if let Some(distance) = distance {
        return format!("distance={:.3}", distance);
    }
    "".to_string()
}

fn preview(text: &str, max_len: usize) -> String {
    let trimmed = text.trim();
    if trimmed.len() <= max_len {
        return trimmed.to_string();
    }
    let mut out = trimmed.chars().take(max_len).collect::<String>();
    out.push('…');
    out
}
