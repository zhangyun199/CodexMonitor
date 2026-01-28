use serde_json::{json, Value};
use tauri::{AppHandle, State};

pub(crate) use crate::life_core::{
    build_delivery_dashboard, build_life_workspace_prompt, build_media_dashboard,
    is_life_workspace, life_debug_enabled, DeliveryDashboard, MediaDashboard,
};
use crate::remote_backend;
use crate::state::AppState;

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
) -> Result<DeliveryDashboard, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "get_delivery_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let workspaces = state.workspaces.lock().await;
    let entry = workspaces.get(&workspace_id).ok_or("workspace not found")?;
    let supabase = {
        let settings = state.app_settings.lock().await;
        if settings.supabase_url.trim().is_empty() || settings.supabase_anon_key.trim().is_empty() {
            None
        } else {
            Some((
                settings.supabase_url.clone(),
                settings.supabase_anon_key.clone(),
            ))
        }
    };

    build_delivery_dashboard(
        &entry.path,
        entry.settings.obsidian_root.as_deref(),
        supabase.as_ref().map(|value| value.0.as_str()),
        supabase.as_ref().map(|value| value.1.as_str()),
        &range,
    )
    .await
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
) -> Result<MediaDashboard, String> {
    if remote_backend::is_remote_mode(&*state).await {
        return remote_backend::call_remote(
            &*state,
            app,
            "get_media_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await
        .and_then(|response| serde_json::from_value(response).map_err(|err| err.to_string()));
    }
    let workspaces = state.workspaces.lock().await;
    let entry = workspaces.get(&workspace_id).ok_or("workspace not found")?;
    let supabase = {
        let settings = state.app_settings.lock().await;
        if settings.supabase_url.trim().is_empty() || settings.supabase_anon_key.trim().is_empty() {
            None
        } else {
            Some((
                settings.supabase_url.clone(),
                settings.supabase_anon_key.clone(),
            ))
        }
    };

    build_media_dashboard(
        &entry.path,
        entry.settings.obsidian_root.as_deref(),
        supabase.as_ref().map(|value| value.0.as_str()),
        supabase.as_ref().map(|value| value.1.as_str()),
        &range,
    )
    .await
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
