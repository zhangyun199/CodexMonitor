use tauri::{AppHandle, Emitter, Manager};

use crate::backend::events::{AppServerEvent, EventSink, TerminalOutput};
use crate::auto_flush::{
    build_snapshot, parse_memory_flush_result, run_memory_flush_summarizer, write_memory_flush,
};
use crate::state::AppState;

#[derive(Clone)]
pub(crate) struct TauriEventSink {
    app: AppHandle,
}

impl TauriEventSink {
    pub(crate) fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl EventSink for TauriEventSink {
    fn emit_app_server_event(&self, event: AppServerEvent) {
        let _ = self.app.emit("app-server-event", event.clone());
        let app = self.app.clone();
        tauri::async_runtime::spawn(async move {
            maybe_trigger_auto_memory(app, event).await;
        });
    }

    fn emit_terminal_output(&self, event: TerminalOutput) {
        let _ = self.app.emit("terminal-output", event);
    }
}

async fn maybe_trigger_auto_memory(app: AppHandle, event: AppServerEvent) {
    let method = event
        .message
        .get("method")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    if method != "thread/tokenUsage/updated" {
        return;
    }

    let params = event
        .message
        .get("params")
        .and_then(|value| value.as_object())
        .cloned()
        .unwrap_or_default();
    let thread_id = params
        .get("threadId")
        .or_else(|| params.get("thread_id"))
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .to_string();
    if thread_id.is_empty() {
        return;
    }
    let token_usage = params
        .get("tokenUsage")
        .or_else(|| params.get("token_usage"))
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let total_tokens = token_usage
        .pointer("/total/totalTokens")
        .or_else(|| token_usage.pointer("/total/total_tokens"))
        .or_else(|| token_usage.get("totalTokens"))
        .or_else(|| token_usage.get("total_tokens"))
        .and_then(|value| value.as_u64())
        .unwrap_or(0) as u32;
    let model_context_window = token_usage
        .get("modelContextWindow")
        .or_else(|| token_usage.get("model_context_window"))
        .or_else(|| params.get("modelContextWindow"))
        .or_else(|| params.get("model_context_window"))
        .and_then(|value| value.as_u64())
        .unwrap_or(0) as u32;
    if total_tokens == 0 || model_context_window == 0 {
        return;
    }

    let state = app.state::<AppState>();
    let settings = state.app_settings.lock().await.clone();
    if !settings.auto_memory.enabled {
        return;
    }

    let should_flush = {
        let mut runtime = state.auto_memory_runtime.lock().await;
        runtime.update_and_check(
            &format!("{}:{}", event.workspace_id, thread_id),
            total_tokens,
            model_context_window,
            &settings.auto_memory,
        )
    };
    if !should_flush {
        return;
    }

    let memory = match state.memory.read().await.clone() {
        Some(mem) => mem,
        None => return,
    };
    let session = {
        let sessions = state.sessions.lock().await;
        sessions.get(&event.workspace_id).cloned()
    };
    let Some(session) = session else {
        return;
    };

    let auto_settings = settings.auto_memory.clone();
    let workspace_id = event.workspace_id.clone();
    let thread_id_clone = thread_id.clone();
    tauri::async_runtime::spawn(async move {
        let snapshot = match build_snapshot(
            &session,
            &workspace_id,
            &thread_id_clone,
            total_tokens,
            model_context_window,
            &auto_settings,
        )
        .await
        {
            Ok(snapshot) => snapshot,
            Err(err) => {
                eprintln!("Auto memory snapshot failed: {err}");
                return;
            }
        };

        let raw = match run_memory_flush_summarizer(&session, &snapshot).await {
            Ok(raw) => raw,
            Err(err) => {
                eprintln!("Auto memory summarizer failed: {err}");
                return;
            }
        };

        let result = parse_memory_flush_result(&raw);
        if let Err(err) = write_memory_flush(&memory, &snapshot, &result, &auto_settings).await {
            eprintln!("Auto memory write failed: {err}");
        }
    });
}
