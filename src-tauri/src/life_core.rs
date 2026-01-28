use std::path::PathBuf;

use crate::types::{WorkspacePurpose, WorkspaceSettings};

const LIFE_PROMPT_FILES: [&str; 4] = [
    "workspace-delivery-finance.md",
    "workspace-food-exercise.md",
    "workspace-media.md",
    "workspace-youtube.md",
];

const LIFE_PROMPT_TAIL: &str = "You are JMWillis's life assistant with full context across all domains.\nAuto-detect the domain from user messages and respond appropriately.";

pub(crate) fn is_life_workspace(settings: &WorkspaceSettings) -> bool {
    matches!(settings.purpose, Some(WorkspacePurpose::Life))
}

pub(crate) fn life_debug_enabled() -> bool {
    std::env::var("LIFE_DEBUG")
        .map(|value| {
            let trimmed = value.trim();
            !trimmed.is_empty() && trimmed != "0"
        })
        .unwrap_or(false)
}

fn resolve_prompt_root() -> Result<PathBuf, String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut candidates = Vec::new();
    if let Some(parent) = manifest_dir.parent() {
        candidates.push(parent.to_path_buf());
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd);
    }

    for candidate in &candidates {
        let has_all = LIFE_PROMPT_FILES
            .iter()
            .all(|name| candidate.join(name).exists());
        if has_all {
            return Ok(candidate.clone());
        }
    }

    candidates
        .into_iter()
        .next()
        .ok_or_else(|| "Unable to resolve life prompt root.".to_string())
}

pub(crate) fn build_life_workspace_prompt() -> Result<String, String> {
    let root = resolve_prompt_root()?;
    let mut parts = Vec::with_capacity(LIFE_PROMPT_FILES.len() + 1);
    for name in LIFE_PROMPT_FILES.iter() {
        let path = root.join(name);
        let content = std::fs::read_to_string(&path)
            .map_err(|err| format!("Failed to read {name}: {err}"))?;
        parts.push(content);
    }
    parts.push(LIFE_PROMPT_TAIL.to_string());
    Ok(parts.join("\n---\n"))
}

#[cfg(test)]
mod tests {
    use super::{build_life_workspace_prompt, LIFE_PROMPT_FILES, LIFE_PROMPT_TAIL};
    use std::path::PathBuf;

    fn prompt_files_present() -> bool {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let Some(root) = manifest_dir.parent() else {
            return false;
        };
        LIFE_PROMPT_FILES
            .iter()
            .all(|name| root.join(name).exists())
    }

    #[test]
    fn build_life_workspace_prompt_includes_tail_and_separators() {
        if !prompt_files_present() {
            return;
        }
        let prompt = build_life_workspace_prompt().expect("prompt build");
        assert!(prompt.contains(LIFE_PROMPT_TAIL));
        let separator_count = prompt.matches("\n---\n").count();
        assert!(separator_count >= LIFE_PROMPT_FILES.len());
    }
}
