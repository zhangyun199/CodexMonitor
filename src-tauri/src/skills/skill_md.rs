use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Requirements {
    #[serde(default)]
    pub bins: Vec<String>,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub os: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDescriptor {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub requirements: Requirements,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SkillFrontmatter {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub requirements: Requirements,
}

pub fn parse_skill_md(path: &Path) -> Result<SkillDescriptor, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let (frontmatter, body) = split_frontmatter(&content);
    let mut name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|v| v.to_str())
        .unwrap_or("skill")
        .to_string();
    let mut description = None;
    let mut requirements = Requirements::default();

    if let Some(frontmatter) = frontmatter {
        let fm: SkillFrontmatter = serde_yaml::from_str(&frontmatter).map_err(|e| e.to_string())?;
        if let Some(fm_name) = fm.name {
            name = fm_name;
        }
        description = fm.description;
        requirements = fm.requirements;
    }

    if description.is_none() {
        description = body
            .lines()
            .find(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string());
    }

    Ok(SkillDescriptor {
        name,
        description,
        path: path.parent().unwrap_or(Path::new("")).display().to_string(),
        requirements,
    })
}

pub fn validate_skill(skill: &SkillDescriptor) -> Vec<String> {
    let mut issues = Vec::new();
    let os = std::env::consts::OS;
    if !skill.requirements.os.is_empty()
        && !skill
            .requirements
            .os
            .iter()
            .any(|value| value.eq_ignore_ascii_case(os))
    {
        issues.push(format!("unsupported OS: {os}"));
    }

    for bin in &skill.requirements.bins {
        if which::which(bin).is_err() {
            issues.push(format!("missing binary: {bin}"));
        }
    }

    for env_key in &skill.requirements.env {
        if env::var(env_key).is_err() {
            issues.push(format!("missing env var: {env_key}"));
        }
    }

    issues
}

fn split_frontmatter(content: &str) -> (Option<String>, String) {
    let mut lines = content.lines();
    if lines.next().map(|l| l.trim()) != Some("---") {
        return (None, content.to_string());
    }
    let mut frontmatter_lines = Vec::new();
    let mut in_frontmatter = true;
    let mut body_start = 0usize;
    for (idx, line) in content.lines().enumerate().skip(1) {
        if in_frontmatter && line.trim() == "---" {
            in_frontmatter = false;
            body_start = idx + 1;
            break;
        }
        frontmatter_lines.push(line);
    }
    if in_frontmatter {
        return (None, content.to_string());
    }
    let frontmatter = frontmatter_lines.join("\n");
    let body = content
        .lines()
        .skip(body_start)
        .collect::<Vec<_>>()
        .join("\n");
    (Some(frontmatter), body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn parse_skill_md_with_frontmatter() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("SKILL.md");
        let content = r#"---
name: Test Skill
description: Fancy skill
requirements:
  bins: ["git"]
  env: ["API_KEY"]
  os: ["macos"]
---

Body content
"#;
        std::fs::write(&path, content).expect("write");
        let desc = parse_skill_md(&path).expect("parse");
        assert_eq!(desc.name, "Test Skill");
        assert_eq!(desc.description.as_deref(), Some("Fancy skill"));
        assert!(desc.requirements.bins.contains(&"git".to_string()));
        assert!(desc.requirements.env.contains(&"API_KEY".to_string()));
        assert!(desc.requirements.os.contains(&"macos".to_string()));
    }

    #[test]
    fn parse_skill_md_without_frontmatter_uses_body() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("SKILL.md");
        let content = "First line description\nMore details";
        std::fs::write(&path, content).expect("write");
        let desc = parse_skill_md(&path).expect("parse");
        assert_eq!(desc.description.as_deref(), Some("First line description"));
    }
}
