use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::{AppSettings, Domain, DomainTheme, WorkspaceEntry};

pub(crate) fn read_workspaces(path: &PathBuf) -> Result<HashMap<String, WorkspaceEntry>, String> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let list: Vec<WorkspaceEntry> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(list
        .into_iter()
        .map(|entry| (entry.id.clone(), entry))
        .collect())
}

pub(crate) fn write_workspaces(path: &PathBuf, entries: &[WorkspaceEntry]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(entries).map_err(|e| e.to_string())?;
    std::fs::write(path, data).map_err(|e| e.to_string())
}

pub(crate) fn read_settings(path: &PathBuf) -> Result<AppSettings, String> {
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

pub(crate) fn write_settings(path: &PathBuf, settings: &AppSettings) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    std::fs::write(path, data).map_err(|e| e.to_string())
}

pub(crate) fn read_domains(path: &PathBuf) -> Result<Vec<Domain>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

pub(crate) fn write_domains(path: &PathBuf, domains: &[Domain]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(domains).map_err(|e| e.to_string())?;
    std::fs::write(path, data).map_err(|e| e.to_string())
}

pub(crate) fn seed_domains_from_files() -> Vec<Domain> {
    struct SeedSpec<'a> {
        id: &'a str,
        name: &'a str,
        description: &'a str,
        icon: &'a str,
        color: &'a str,
        accent: &'a str,
        background: Option<&'a str>,
        path: &'a str,
    }

    let seeds = [
        SeedSpec {
            id: "delivery-finance",
            name: "Delivery + Finance",
            description: "Food delivery shifts, earnings, and bills",
            icon: "🚗",
            color: "#10b981",
            accent: "#10b981",
            background: None,
            path: "/Users/jmwillis/Desktop/workspace-delivery-finance.md",
        },
        SeedSpec {
            id: "food-exercise",
            name: "Food + Exercise",
            description: "Nutrition, workouts, and weight loss tracking",
            icon: "🍽️",
            color: "#f97316",
            accent: "#f97316",
            background: None,
            path: "/Users/jmwillis/Desktop/workspace-food-exercise.md",
        },
        SeedSpec {
            id: "media",
            name: "Media",
            description: "Movies, shows, games, and ratings",
            icon: "🎬",
            color: "#6366f1",
            accent: "#6366f1",
            background: None,
            path: "/Users/jmwillis/Desktop/workspace-media.md",
        },
        SeedSpec {
            id: "youtube",
            name: "YouTube Channel",
            description: "Video ideas, pipeline, and publishing",
            icon: "🎥",
            color: "#ef4444",
            accent: "#ef4444",
            background: None,
            path: "/Users/jmwillis/Desktop/workspace-youtube.md",
        },
    ];

    let mut domains = Vec::new();
    for seed in seeds {
        if let Ok(content) = std::fs::read_to_string(seed.path) {
            if content.trim().is_empty() {
                continue;
            }
            domains.push(Domain {
                id: seed.id.to_string(),
                name: seed.name.to_string(),
                description: Some(seed.description.to_string()),
                system_prompt: content,
                view_type: "dashboard".to_string(),
                theme: DomainTheme {
                    icon: seed.icon.to_string(),
                    color: seed.color.to_string(),
                    accent: seed.accent.to_string(),
                    background: seed.background.map(|value| value.to_string()),
                },
                default_model: None,
                default_access_mode: None,
                default_reasoning_effort: None,
                default_approval_policy: None,
            });
        }
    }

    domains
}

#[cfg(test)]
mod tests {
    use super::{read_workspaces, write_workspaces};
    use crate::types::{WorkspaceEntry, WorkspaceKind, WorkspaceSettings};
    use uuid::Uuid;

    #[test]
    fn write_read_workspaces_persists_sort_and_group() {
        let temp_dir = std::env::temp_dir().join(format!("codex-monitor-test-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).expect("create temp dir");
        let path = temp_dir.join("workspaces.json");

        let mut settings = WorkspaceSettings::default();
        settings.sort_order = Some(5);
        settings.group_id = Some("group-42".to_string());
        settings.sidebar_collapsed = true;
        settings.git_root = Some("/tmp".to_string());

        let entry = WorkspaceEntry {
            id: "w1".to_string(),
            name: "Workspace".to_string(),
            path: "/tmp".to_string(),
            codex_bin: None,
            kind: WorkspaceKind::Main,
            parent_id: None,
            worktree: None,
            settings: settings.clone(),
        };

        write_workspaces(&path, &[entry]).expect("write workspaces");
        let read = read_workspaces(&path).expect("read workspaces");
        let stored = read.get("w1").expect("stored workspace");
        assert_eq!(stored.settings.sort_order, Some(5));
        assert_eq!(stored.settings.group_id.as_deref(), Some("group-42"));
        assert!(stored.settings.sidebar_collapsed);
        assert_eq!(stored.settings.git_root.as_deref(), Some("/tmp"));
    }
}
