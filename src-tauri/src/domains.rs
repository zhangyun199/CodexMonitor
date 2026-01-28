use serde_json::json;
use tauri::{AppHandle, State};

use crate::remote_backend;
use crate::state::AppState;
use crate::storage::write_domains;
use crate::types::{Domain, DomainTrendSnapshot};
use crate::obsidian::compute_domain_trends;

fn normalize_domain(mut domain: Domain) -> Domain {
    if domain.view_type.trim().is_empty() {
        domain.view_type = "chat".to_string();
    }
    domain
}

#[tauri::command]
pub(crate) async fn domains_list(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Vec<Domain>, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response =
            remote_backend::call_remote(&*state, app, "domains_list", json!({})).await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let domains = state.domains.lock().await;
    Ok(domains.clone())
}

#[tauri::command]
pub(crate) async fn domains_create(
    mut domain: Domain,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Domain, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "domains_create",
            serde_json::to_value(&domain).map_err(|err| err.to_string())?,
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    domain.id = uuid::Uuid::new_v4().to_string();
    let domain = normalize_domain(domain);
    let mut domains = state.domains.lock().await;
    domains.push(domain.clone());
    write_domains(&state.domains_path, &domains)?;
    Ok(domain)
}

#[tauri::command]
pub(crate) async fn domains_update(
    domain: Domain,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<Domain, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "domains_update",
            serde_json::to_value(&domain).map_err(|err| err.to_string())?,
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }
    let domain = normalize_domain(domain);
    let mut domains = state.domains.lock().await;
    if let Some(idx) = domains.iter().position(|item| item.id == domain.id) {
        domains[idx] = domain.clone();
        write_domains(&state.domains_path, &domains)?;
        Ok(domain)
    } else {
        Err(format!("Domain not found: {}", domain.id))
    }
}

#[tauri::command]
pub(crate) async fn domains_delete(
    domain_id: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    if remote_backend::is_remote_mode(&*state).await {
        remote_backend::call_remote(
            &*state,
            app,
            "domains_delete",
            json!({ "domainId": domain_id }),
        )
        .await?;
        return Ok(());
    }
    let mut domains = state.domains.lock().await;
    domains.retain(|domain| domain.id != domain_id);
    write_domains(&state.domains_path, &domains)?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn domain_trends(
    workspace_id: String,
    domain_id: String,
    range: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<DomainTrendSnapshot, String> {
    if remote_backend::is_remote_mode(&*state).await {
        let response = remote_backend::call_remote(
            &*state,
            app,
            "domain_trends",
            json!({
                "workspaceId": workspace_id,
                "domainId": domain_id,
                "range": range
            }),
        )
        .await?;
        return serde_json::from_value(response).map_err(|err| err.to_string());
    }

    let workspaces = state.workspaces.lock().await;
    let workspace = workspaces
        .get(&workspace_id)
        .ok_or_else(|| "workspace not found".to_string())?;
    compute_domain_trends(&workspace.path, &domain_id, &range)
}

#[tauri::command]
pub(crate) async fn read_text_file(path: String) -> Result<String, String> {
    tokio::fs::read_to_string(path).await.map_err(|e| e.to_string())
}
