use serde_json::{json, Value};
use tauri::{AppHandle, State};

use crate::remote_backend;
use crate::state::AppState;

#[path = "life_core.rs"]
mod core;

pub(crate) use core::{build_life_workspace_prompt, is_life_workspace, life_debug_enabled};

#[tauri::command]
pub(crate) async fn get_life_workspace_prompt(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<String, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response =
            remote_backend::call_remote(&*state, app, "get_life_workspace_prompt", json!({}))
                .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    build_life_workspace_prompt()
}

#[tauri::command]
pub(crate) async fn get_delivery_dashboard(
    workspace_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Value, String> {
    if remote_backend::is_remote_mode(&*state).await {
        return remote_backend::call_remote(
            &*state,
            app,
            "get_delivery_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await;
    }
    Ok(json!({}))
}

#[tauri::command]
pub(crate) async fn get_nutrition_dashboard(
    workspace_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Value, String> {
    if remote_backend::is_remote_mode(&*state).await {
        return remote_backend::call_remote(
            &*state,
            app,
            "get_nutrition_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await;
    }
    Ok(json!({}))
}

#[tauri::command]
pub(crate) async fn get_exercise_dashboard(
    workspace_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Value, String> {
    if remote_backend::is_remote_mode(&*state).await {
        return remote_backend::call_remote(
            &*state,
            app,
            "get_exercise_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await;
    }
    Ok(json!({}))
}

#[tauri::command]
pub(crate) async fn get_media_dashboard(
    workspace_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Value, String> {
    if remote_backend::is_remote_mode(&*state).await {
        return remote_backend::call_remote(
            &*state,
            app,
            "get_media_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await;
    }
    Ok(json!({}))
}

#[tauri::command]
pub(crate) async fn get_youtube_dashboard(
    workspace_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Value, String> {
    if remote_backend::is_remote_mode(&*state).await {
        return remote_backend::call_remote(
            &*state,
            app,
            "get_youtube_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await;
    }
    Ok(json!({}))
}

#[tauri::command]
pub(crate) async fn get_finance_dashboard(
    workspace_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Value, String> {
    if remote_backend::is_remote_mode(&*state).await {
        return remote_backend::call_remote(
            &*state,
            app,
            "get_finance_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await;
    }
    Ok(json!({}))
}
