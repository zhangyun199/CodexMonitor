use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{command, AppHandle, State};

use crate::remote_backend;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub(crate) struct TextFileResponse {
    pub exists: bool,
    pub content: String,
    pub truncated: bool,
}

fn resolve_home_dir() -> Option<PathBuf> {
    if let Ok(value) = std::env::var("HOME") {
        if !value.trim().is_empty() {
            return Some(PathBuf::from(value));
        }
    }
    if let Ok(value) = std::env::var("USERPROFILE") {
        if !value.trim().is_empty() {
            return Some(PathBuf::from(value));
        }
    }
    None
}

fn codex_root() -> Result<PathBuf, String> {
    resolve_home_dir()
        .map(|home| home.join(".codex"))
        .ok_or_else(|| "No home directory".to_string())
}

fn read_text_file(path: &Path) -> Result<TextFileResponse, String> {
    if !path.exists() {
        return Ok(TextFileResponse {
            exists: false,
            content: String::new(),
            truncated: false,
        });
    }
    let content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    Ok(TextFileResponse {
        exists: true,
        content,
        truncated: false,
    })
}

fn write_text_file(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    std::fs::write(path, content).map_err(|err| err.to_string())
}

#[command]
pub async fn read_global_agents_md(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<TextFileResponse, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response =
            remote_backend::call_remote(&*state, app, "read_global_agents_md", json!({})).await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let root = codex_root()?;
    let path = root.join("AGENTS.md");
    read_text_file(&path)
}

#[command]
pub async fn write_global_agents_md(
    content: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    if remote_backend::is_remote_mode(&*state).await {
        remote_backend::call_remote(
            &*state,
            app,
            "write_global_agents_md",
            json!({ "content": content }),
        )
        .await?;
        return Ok(());
    }
    let root = codex_root()?;
    let path = root.join("AGENTS.md");
    write_text_file(&path, &content)
}

#[command]
pub async fn read_global_config_toml(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<TextFileResponse, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response =
            remote_backend::call_remote(&*state, app, "read_global_config_toml", json!({})).await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let root = codex_root()?;
    let path = root.join("config.toml");
    read_text_file(&path)
}

#[command]
pub async fn write_global_config_toml(
    content: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    if remote_backend::is_remote_mode(&*state).await {
        remote_backend::call_remote(
            &*state,
            app,
            "write_global_config_toml",
            json!({ "content": content }),
        )
        .await?;
        return Ok(());
    }
    let root = codex_root()?;
    let path = root.join("config.toml");
    write_text_file(&path, &content)
}
