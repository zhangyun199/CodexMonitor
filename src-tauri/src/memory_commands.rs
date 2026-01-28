use serde_json::json;
use tauri::{AppHandle, State};

use crate::auto_flush::{
    build_snapshot, parse_memory_flush_result, run_memory_flush_summarizer, write_memory_flush,
};
use crate::memory::service::MemoryStatus;
use crate::memory::supabase::{MemoryEntry, MemorySearchResult};
use crate::remote_backend;
use crate::state::AppState;
use crate::types::AutoMemorySettings;

#[tauri::command]
pub(crate) async fn memory_status(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<MemoryStatus, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(&*state, app, "memory_status", json!({})).await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }

    let memory = state.memory.read().await;
    match memory.as_ref() {
        Some(mem) => mem.status().await,
        None => Ok(MemoryStatus {
            enabled: false,
            embeddings_enabled: false,
            total: 0,
            pending: 0,
            ready: 0,
            error: 0,
        }),
    }
}

#[tauri::command]
pub(crate) async fn memory_search(
    query: String,
    limit: Option<usize>,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Vec<MemorySearchResult>, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "memory_search",
            json!({ "query": query, "limit": limit.unwrap_or(10) }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }

    let memory = state.memory.read().await;
    match memory.as_ref() {
        Some(mem) => mem.search(&query, limit.unwrap_or(10)).await,
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
pub(crate) async fn memory_append(
    memory_type: String,
    content: String,
    tags: Vec<String>,
    workspace_id: Option<String>,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<MemoryEntry, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "memory_append",
            json!({
                "type": memory_type,
                "content": content,
                "tags": tags,
                "workspace_id": workspace_id
            }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }

    let memory = state
        .memory
        .read()
        .await
        .clone()
        .ok_or("Memory not enabled")?;
    memory
        .append(&memory_type, &content, tags, workspace_id)
        .await
}

#[tauri::command]
pub(crate) async fn memory_bootstrap(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Vec<MemorySearchResult>, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response =
            remote_backend::call_remote(&*state, app, "memory_bootstrap", json!({})).await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }

    let memory = state.memory.read().await;
    match memory.as_ref() {
        Some(mem) => mem.bootstrap().await,
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
pub(crate) async fn memory_flush_now(
    workspace_id: String,
    thread_id: String,
    force: Option<bool>,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<serde_json::Value, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "memory_flush_now",
            json!({
                "workspaceId": workspace_id,
                "threadId": thread_id,
                "force": force.unwrap_or(false)
            }),
        )
        .await?;
        return Ok(response);
    }

    let settings = state.app_settings.lock().await.clone();
    if !settings.auto_memory.enabled && !force.unwrap_or(false) {
        return Err("Auto memory disabled".to_string());
    }
    let memory = state
        .memory
        .read()
        .await
        .clone()
        .ok_or("Memory not enabled")?;
    let session = {
        let sessions = state.sessions.lock().await;
        sessions
            .get(&workspace_id)
            .cloned()
            .ok_or("workspace not connected".to_string())?
    };

    perform_memory_flush(
        session,
        memory,
        settings.auto_memory,
        workspace_id,
        thread_id,
        0,
        0,
    )
    .await
}

async fn perform_memory_flush(
    session: std::sync::Arc<crate::backend::app_server::WorkspaceSession>,
    memory: crate::memory::MemoryService,
    settings: AutoMemorySettings,
    workspace_id: String,
    thread_id: String,
    context_tokens: u32,
    model_context_window: u32,
) -> Result<serde_json::Value, String> {
    let snapshot = build_snapshot(
        &session,
        &workspace_id,
        &thread_id,
        context_tokens,
        model_context_window,
        &settings,
    )
    .await?;

    let raw = run_memory_flush_summarizer(&session, &snapshot).await?;
    let result = parse_memory_flush_result(&raw);
    write_memory_flush(&memory, &snapshot, &result, &settings).await?;

    Ok(json!({
        "ok": true,
        "noReply": result.no_reply,
        "tags": result.tags,
    }))
}
