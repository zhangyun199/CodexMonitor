use serde_json::json;
use tauri::{AppHandle, State};

pub(crate) use crate::life_core::{
    build_delivery_dashboard, build_exercise_dashboard, build_finance_dashboard,
    build_life_workspace_prompt, build_media_library, build_nutrition_dashboard,
    build_youtube_library, enrich_media_covers as enrich_media_covers_inner, is_life_workspace,
    life_debug_enabled, DeliveryDashboard, ExerciseDashboard, FinanceDashboard, MediaCoverSummary,
    MediaLibrary, NutritionDashboard, YouTubeLibrary,
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
) -> Result<NutritionDashboard, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "get_nutrition_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let workspaces = state.workspaces.lock().await;
    let entry = workspaces.get(&workspace_id).ok_or("workspace not found")?;

    build_nutrition_dashboard(&entry.path, entry.settings.obsidian_root.as_deref(), &range).await
}

#[tauri::command]
pub(crate) async fn get_exercise_dashboard(
    workspace_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<ExerciseDashboard, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "get_exercise_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let workspaces = state.workspaces.lock().await;
    let entry = workspaces.get(&workspace_id).ok_or("workspace not found")?;

    build_exercise_dashboard(&entry.path, entry.settings.obsidian_root.as_deref(), &range).await
}

#[tauri::command]
pub(crate) async fn get_media_dashboard(
    workspace_id: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<MediaLibrary, String> {
    if remote_backend::is_remote_mode(&*state).await {
        return remote_backend::call_remote(
            &*state,
            app,
            "get_media_dashboard",
            json!({ "workspaceId": workspace_id }),
        )
        .await
        .and_then(|response| serde_json::from_value(response).map_err(|err| err.to_string()));
    }
    let workspaces = state.workspaces.lock().await;
    let entry = workspaces.get(&workspace_id).ok_or("workspace not found")?;

    build_media_library(&entry.path, entry.settings.obsidian_root.as_deref()).await
}

#[tauri::command]
pub(crate) async fn get_youtube_dashboard(
    workspace_id: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<YouTubeLibrary, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "get_youtube_dashboard",
            json!({ "workspaceId": workspace_id }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let workspaces = state.workspaces.lock().await;
    let entry = workspaces.get(&workspace_id).ok_or("workspace not found")?;

    build_youtube_library(&entry.path, entry.settings.obsidian_root.as_deref()).await
}

#[tauri::command]
pub(crate) async fn enrich_media_covers(
    workspace_id: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<MediaCoverSummary, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "enrich_media_covers",
            json!({ "workspaceId": workspace_id }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let workspaces = state.workspaces.lock().await;
    let entry = workspaces.get(&workspace_id).ok_or("workspace not found")?;
    let settings = state.app_settings.lock().await;
    let tmdb_key = resolve_api_key(settings.tmdb_api_key.as_str(), "TMDB_API_KEY");
    let igdb_client_id = resolve_api_key(settings.igdb_client_id.as_str(), "IGDB_CLIENT_ID");
    let igdb_client_secret =
        resolve_api_key(settings.igdb_client_secret.as_str(), "IGDB_CLIENT_SECRET");

    enrich_media_covers_inner(
        &entry.path,
        entry.settings.obsidian_root.as_deref(),
        tmdb_key.as_deref(),
        igdb_client_id.as_deref(),
        igdb_client_secret.as_deref(),
    )
    .await
}

fn resolve_api_key(value: &str, env_key: &str) -> Option<String> {
    if !value.trim().is_empty() {
        return Some(value.to_string());
    }
    std::env::var(env_key).ok().filter(|v| !v.trim().is_empty())
}

#[tauri::command]
pub(crate) async fn get_finance_dashboard(
    workspace_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<FinanceDashboard, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "get_finance_dashboard",
            json!({ "workspaceId": workspace_id, "range": range }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let workspaces = state.workspaces.lock().await;
    let entry = workspaces.get(&workspace_id).ok_or("workspace not found")?;

    build_finance_dashboard(&entry.path, entry.settings.obsidian_root.as_deref(), &range).await
}
