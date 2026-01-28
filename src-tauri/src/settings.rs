use serde_json::json;
use tauri::{AppHandle, State, Window};

use crate::codex_config;
use crate::remote_backend;
use crate::state::AppState;
use crate::storage::write_settings;
use crate::types::AppSettings;
use crate::window;

#[tauri::command]
pub(crate) async fn get_app_settings(
    state: State<'_, AppState>,
    app: AppHandle,
    window: Window,
) -> Result<AppSettings, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response =
            remote_backend::call_remote(&*state, app, "get_app_settings", json!({})).await?;
        let settings: AppSettings =
            serde_json::from_value(response).map_err(|err| err.to_string())?;
        let _ = window::apply_window_appearance(&window, settings.theme.as_str());
        let mut current = state.app_settings.lock().await;
        *current = settings.clone();
        return Ok(settings);
    }
    let mut settings = state.app_settings.lock().await.clone();
    if let Ok(Some(collab_enabled)) = codex_config::read_collab_enabled() {
        settings.experimental_collab_enabled = collab_enabled;
    }
    if let Ok(Some(steer_enabled)) = codex_config::read_steer_enabled() {
        settings.experimental_steer_enabled = steer_enabled;
    }
    if let Ok(Some(unified_exec_enabled)) = codex_config::read_unified_exec_enabled() {
        settings.experimental_unified_exec_enabled = unified_exec_enabled;
    }
    let _ = window::apply_window_appearance(&window, settings.theme.as_str());
    Ok(settings)
}

#[tauri::command]
pub(crate) async fn update_app_settings(
    settings: AppSettings,
    state: State<'_, AppState>,
    app: AppHandle,
    window: Window,
) -> Result<AppSettings, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "update_app_settings",
            json!({ "settings": settings }),
        )
        .await?;
        let updated: AppSettings =
            serde_json::from_value(response).map_err(|err| err.to_string())?;
        let mut current = state.app_settings.lock().await;
        *current = updated.clone();
        let _ = window::apply_window_appearance(&window, updated.theme.as_str());
        return Ok(updated);
    }
    let _ = codex_config::write_collab_enabled(settings.experimental_collab_enabled);
    let _ = codex_config::write_steer_enabled(settings.experimental_steer_enabled);
    let _ = codex_config::write_unified_exec_enabled(settings.experimental_unified_exec_enabled);
    write_settings(&state.settings_path, &settings)?;
    let mut current = state.app_settings.lock().await;
    *current = settings.clone();
    let mut memory_lock = state.memory.write().await;
    *memory_lock = if settings.memory_enabled
        && !settings.supabase_url.is_empty()
        && !settings.supabase_anon_key.is_empty()
    {
        Some(crate::memory::MemoryService::new(
            &settings.supabase_url,
            &settings.supabase_anon_key,
            if settings.memory_embedding_enabled {
                Some(&settings.minimax_api_key)
            } else {
                None
            },
            true,
        ))
    } else {
        None
    };
    let _ = window::apply_window_appearance(&window, settings.theme.as_str());
    Ok(settings)
}
