use codex_monitor_lib::life_core::enrich_media_covers;
use serde_json::Value;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), String> {
    let force_refresh = std::env::args().any(|arg| arg == "--force");
    let obsidian_root = std::env::var("OBSIDIAN_ROOT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "/Volumes/YouTube 4TB/Obsidian".to_string());
    let settings_path = std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("Library")
        .join("Application Support")
        .join("com.codexmonitor.app")
        .join("settings.json");

    let settings_value = std::fs::read_to_string(&settings_path)
        .ok()
        .and_then(|content| serde_json::from_str::<Value>(&content).ok())
        .unwrap_or(Value::Null);

    let tmdb_api_key = settings_value
        .get("tmdb_api_key")
        .and_then(|value| value.as_str())
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.to_string());
    let igdb_client_id = settings_value
        .get("igdb_client_id")
        .and_then(|value| value.as_str())
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.to_string());
    let igdb_client_secret = settings_value
        .get("igdb_client_secret")
        .and_then(|value| value.as_str())
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.to_string());
    let exa_api_key = settings_value
        .get("exa_api_key")
        .and_then(|value| value.as_str())
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.to_string())
        .or_else(|| {
            std::env::var("EXA_API_KEY")
                .ok()
                .filter(|value| !value.trim().is_empty())
        });

    let summary = enrich_media_covers(
        &obsidian_root,
        Some(&obsidian_root),
        tmdb_api_key.as_deref(),
        igdb_client_id.as_deref(),
        igdb_client_secret.as_deref(),
        exa_api_key.as_deref(),
        force_refresh,
    )
    .await?;

    println!(
        "Media cover enrichment complete: total={}, found={}, skipped={}, failed={}",
        summary.total, summary.found, summary.skipped, summary.failed
    );

    Ok(())
}
