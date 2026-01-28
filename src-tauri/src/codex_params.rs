use serde_json::{json, Map, Value};

pub(crate) fn normalize_collaboration_mode(value: Option<Value>) -> Option<Value> {
    let Some(value) = value else {
        return None;
    };
    if value.is_null() {
        return None;
    }
    let Some(obj) = value.as_object() else {
        return Some(value);
    };

    let mode = obj
        .get("mode")
        .cloned()
        .or_else(|| obj.get("name").cloned());
    let model = obj.get("model").cloned();
    let reasoning_effort = obj
        .get("reasoning_effort")
        .cloned()
        .or_else(|| obj.get("reasoningEffort").cloned());
    let developer_instructions = obj
        .get("developer_instructions")
        .cloned()
        .or_else(|| obj.get("developerInstructions").cloned());

    let settings_value = obj.get("settings").cloned();

    let normalized_settings = match settings_value {
        Some(Value::Object(mut settings)) => {
            if !settings.contains_key("model") {
                if let Some(model) = model.clone() {
                    settings.insert("model".to_string(), model);
                }
            }
            if !settings.contains_key("reasoning_effort") {
                if let Some(reasoning_effort) = reasoning_effort.clone() {
                    settings.insert("reasoning_effort".to_string(), reasoning_effort);
                }
            }
            if !settings.contains_key("developer_instructions") {
                if let Some(developer_instructions) = developer_instructions.clone() {
                    settings.insert("developer_instructions".to_string(), developer_instructions);
                }
            }
            Some(Value::Object(settings))
        }
        Some(_) => None,
        None => {
            let Some(model) = model.clone() else {
                return None;
            };
            let mut settings = Map::new();
            settings.insert("model".to_string(), model);
            if let Some(reasoning_effort) = reasoning_effort.clone() {
                settings.insert("reasoning_effort".to_string(), reasoning_effort);
            }
            if let Some(developer_instructions) = developer_instructions.clone() {
                settings.insert("developer_instructions".to_string(), developer_instructions);
            }
            Some(Value::Object(settings))
        }
    }?;

    let mut normalized = Map::new();
    if let Some(mode) = mode {
        normalized.insert("mode".to_string(), mode);
    }
    normalized.insert("settings".to_string(), normalized_settings);
    Some(Value::Object(normalized))
}

pub(crate) fn build_user_input(
    text: &str,
    images: Option<&[String]>,
) -> Result<Vec<Value>, String> {
    let trimmed_text = text.trim();
    let mut input: Vec<Value> = Vec::new();
    if !trimmed_text.is_empty() {
        input.push(json!({ "type": "text", "text": trimmed_text }));
    }
    if let Some(paths) = images {
        for path in paths {
            let trimmed = path.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.starts_with("data:")
                || trimmed.starts_with("http://")
                || trimmed.starts_with("https://")
            {
                input.push(json!({ "type": "image", "url": trimmed }));
            } else {
                input.push(json!({ "type": "localImage", "path": trimmed }));
            }
        }
    }
    if input.is_empty() {
        return Err("empty user message".to_string());
    }
    Ok(input)
}

pub(crate) fn build_turn_start_params(
    thread_id: &str,
    input: Vec<Value>,
    cwd: &str,
    approval_policy: &str,
    sandbox_policy: Value,
    model: Option<String>,
    effort: Option<String>,
    collaboration_mode: Option<Value>,
    instruction_injection: Option<String>,
) -> Value {
    let collaboration_mode = normalize_collaboration_mode(collaboration_mode);
    let merged_instructions = merge_instruction_injection(
        instruction_injection,
        extract_developer_instructions(&collaboration_mode),
    );
    json!({
        "threadId": thread_id,
        "input": input,
        "cwd": cwd,
        "approvalPolicy": approval_policy,
        "sandboxPolicy": sandbox_policy,
        "model": model,
        "effort": effort,
        "instructionInjection": merged_instructions,
        "collaborationMode": collaboration_mode,
    })
}

fn extract_developer_instructions(collaboration_mode: &Option<Value>) -> Option<String> {
    let Some(Value::Object(obj)) = collaboration_mode else {
        return None;
    };
    let direct = obj
        .get("developer_instructions")
        .or_else(|| obj.get("developerInstructions"))
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());
    if direct.is_some() {
        return direct;
    }
    obj.get("settings")
        .and_then(|value| value.as_object())
        .and_then(|settings| {
            settings
                .get("developer_instructions")
                .or_else(|| settings.get("developerInstructions"))
        })
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn merge_instruction_injection(base: Option<String>, extra: Option<String>) -> Option<String> {
    let base = base.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });
    let extra = extra.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });
    match (base, extra) {
        (None, None) => None,
        (Some(value), None) | (None, Some(value)) => Some(value),
        (Some(base), Some(extra)) => Some(format!("{base}\n\n---\n\n{extra}")),
    }
}

#[cfg(test)]
mod tests {
    use super::{build_turn_start_params, build_user_input, normalize_collaboration_mode};
    use serde_json::json;

    #[test]
    fn normalize_collaboration_mode_returns_none_for_null() {
        assert_eq!(
            normalize_collaboration_mode(Some(serde_json::Value::Null)),
            None
        );
    }

    #[test]
    fn normalize_collaboration_mode_passes_through_with_settings() {
        let input = json!({
            "mode": "plan",
            "settings": {
                "model": "gpt-5.1-codex",
                "reasoning_effort": "medium"
            }
        });
        let output = normalize_collaboration_mode(Some(input.clone()));
        assert_eq!(output, Some(input));
    }

    #[test]
    fn normalize_collaboration_mode_builds_settings_from_mask() {
        let input = json!({
            "mode": "plan",
            "model": "gpt-5.1-codex",
            "reasoningEffort": "high",
            "developerInstructions": "Be concise"
        });
        let output = normalize_collaboration_mode(Some(input)).unwrap();
        let expected = json!({
            "mode": "plan",
            "settings": {
                "model": "gpt-5.1-codex",
                "reasoning_effort": "high",
                "developer_instructions": "Be concise"
            }
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn normalize_collaboration_mode_backfills_settings_model() {
        let input = json!({
            "mode": "execute",
            "model": "gpt-5.1-codex",
            "settings": {
                "reasoning_effort": "low"
            }
        });
        let output = normalize_collaboration_mode(Some(input)).unwrap();
        let expected = json!({
            "mode": "execute",
            "settings": {
                "model": "gpt-5.1-codex",
                "reasoning_effort": "low"
            }
        });
        assert_eq!(output, expected);
    }

    #[test]
    fn normalize_collaboration_mode_returns_none_without_model() {
        let input = json!({
            "mode": "plan"
        });
        let output = normalize_collaboration_mode(Some(input));
        assert_eq!(output, None);
    }

    #[test]
    fn build_user_input_includes_text_and_local_image() {
        let images = vec!["/tmp/screenshot.png".to_string(), "  ".to_string()];
        let input = build_user_input("hello", Some(&images)).expect("input");
        let expected = vec![
            json!({ "type": "text", "text": "hello" }),
            json!({ "type": "localImage", "path": "/tmp/screenshot.png" }),
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn build_user_input_includes_text_and_data_url_image() {
        let images = vec!["data:image/png;base64,ABC".to_string()];
        let input = build_user_input("See this", Some(&images)).expect("input");
        let expected = vec![
            json!({ "type": "text", "text": "See this" }),
            json!({ "type": "image", "url": "data:image/png;base64,ABC" }),
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn build_turn_start_params_normalizes_collaboration_mode() {
        let input = vec![json!({ "type": "text", "text": "hi" })];
        let collaboration_mode = json!({
            "mode": "plan",
            "model": "gpt-5.1-codex",
            "reasoningEffort": "high"
        });
        let params = build_turn_start_params(
            "thread-1",
            input,
            "/tmp",
            "never",
            json!({ "type": "readOnly" }),
            None,
            None,
            Some(collaboration_mode),
            None,
        );
        assert!(params.get("settings").is_none());
        let expected = json!({
            "mode": "plan",
            "settings": {
                "model": "gpt-5.1-codex",
                "reasoning_effort": "high"
            }
        });
        assert_eq!(params.get("collaborationMode"), Some(&expected));
    }

    #[test]
    fn build_turn_start_params_merges_domain_instructions() {
        let input = vec![json!({ "type": "text", "text": "hi" })];
        let collaboration_mode = json!({
            "mode": "plan",
            "settings": {
                "model": "gpt-5.1-codex",
                "developer_instructions": "User instructions"
            }
        });
        let params = build_turn_start_params(
            "thread-1",
            input,
            "/tmp",
            "never",
            json!({ "type": "readOnly" }),
            Some("gpt-5.1-codex".to_string()),
            None,
            Some(collaboration_mode),
            Some("Domain instructions".to_string()),
        );
        let merged = params
            .get("instructionInjection")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        assert!(merged.contains("Domain instructions"));
        assert!(merged.contains("User instructions"));
    }
}
