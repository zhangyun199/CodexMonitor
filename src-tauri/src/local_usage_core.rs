use chrono::{Duration, Local, TimeZone, Utc};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::{LocalUsageDay, LocalUsageModel, LocalUsageSnapshot, LocalUsageTotals};

#[derive(Default, Clone, Copy)]
struct DailyTotals {
    input: i64,
    cached: i64,
    output: i64,
    agent_ms: i64,
    agent_runs: i64,
}

#[derive(Default, Clone, Copy)]
struct UsageTotals {
    input: i64,
    cached: i64,
    output: i64,
}

const MAX_ACTIVITY_GAP_MS: i64 = 2 * 60 * 1000;

pub(crate) async fn local_usage_snapshot_core(
    days: Option<u32>,
    workspace_path: Option<String>,
) -> Result<LocalUsageSnapshot, String> {
    let days = days.unwrap_or(30).clamp(1, 90);
    let workspace_path = workspace_path.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(PathBuf::from(trimmed))
        }
    });
    let snapshot =
        tokio::task::spawn_blocking(move || scan_local_usage(days, workspace_path.as_deref()))
            .await
            .map_err(|err| err.to_string())??;
    Ok(snapshot)
}

fn scan_local_usage(
    days: u32,
    workspace_path: Option<&Path>,
) -> Result<LocalUsageSnapshot, String> {
    let updated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let day_keys = make_day_keys(days);
    let mut daily: HashMap<String, DailyTotals> = day_keys
        .iter()
        .map(|key| (key.clone(), DailyTotals::default()))
        .collect();
    let mut model_totals: HashMap<String, i64> = HashMap::new();

    let Some(root) = resolve_codex_sessions_root() else {
        return Ok(build_snapshot(updated_at, day_keys, daily, HashMap::new()));
    };

    for day_key in &day_keys {
        let day_dir = day_dir_for_key(&root, day_key);
        if !day_dir.exists() {
            continue;
        }
        let entries = match std::fs::read_dir(&day_dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("jsonl") {
                continue;
            }
            scan_file(&path, &mut daily, &mut model_totals, workspace_path)?;
        }
    }

    Ok(build_snapshot(updated_at, day_keys, daily, model_totals))
}

fn build_snapshot(
    updated_at: i64,
    day_keys: Vec<String>,
    daily: HashMap<String, DailyTotals>,
    model_totals: HashMap<String, i64>,
) -> LocalUsageSnapshot {
    let mut days: Vec<LocalUsageDay> = Vec::with_capacity(day_keys.len());
    let mut total_tokens = 0;

    for day_key in &day_keys {
        let totals = daily.get(day_key).copied().unwrap_or_default();
        let total = totals.input + totals.output;
        total_tokens += total;
        days.push(LocalUsageDay {
            day: day_key.clone(),
            input_tokens: totals.input,
            cached_input_tokens: totals.cached,
            output_tokens: totals.output,
            total_tokens: total,
            agent_time_ms: totals.agent_ms,
            agent_runs: totals.agent_runs,
        });
    }

    let last7 = days.iter().rev().take(7).cloned().collect::<Vec<_>>();
    let last7_tokens: i64 = last7.iter().map(|day| day.total_tokens).sum();
    let last7_input: i64 = last7.iter().map(|day| day.input_tokens).sum();
    let last7_cached: i64 = last7.iter().map(|day| day.cached_input_tokens).sum();

    let average_daily_tokens = if last7.is_empty() {
        0
    } else {
        ((last7_tokens as f64) / (last7.len() as f64)).round() as i64
    };

    let cache_hit_rate_percent = if last7_input > 0 {
        ((last7_cached as f64) / (last7_input as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };

    let peak = days
        .iter()
        .max_by_key(|day| day.total_tokens)
        .filter(|day| day.total_tokens > 0);
    let peak_day = peak.map(|day| day.day.clone());
    let peak_day_tokens = peak.map(|day| day.total_tokens).unwrap_or(0);

    let mut top_models: Vec<LocalUsageModel> = model_totals
        .into_iter()
        .filter(|(model, tokens)| model != "unknown" && *tokens > 0)
        .map(|(model, tokens)| LocalUsageModel {
            model,
            tokens,
            share_percent: if total_tokens > 0 {
                ((tokens as f64) / (total_tokens as f64) * 1000.0).round() / 10.0
            } else {
                0.0
            },
        })
        .collect();
    top_models.sort_by(|a, b| b.tokens.cmp(&a.tokens));
    top_models.truncate(4);

    LocalUsageSnapshot {
        updated_at,
        days,
        totals: LocalUsageTotals {
            last7_days_tokens: last7_tokens,
            last30_days_tokens: total_tokens,
            average_daily_tokens,
            cache_hit_rate_percent,
            peak_day,
            peak_day_tokens,
        },
        top_models,
    }
}

fn scan_file(
    path: &Path,
    daily: &mut HashMap<String, DailyTotals>,
    model_totals: &mut HashMap<String, i64>,
    workspace_path: Option<&Path>,
) -> Result<(), String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            return Ok(());
        }
    };
    let reader = BufReader::new(file);
    let mut previous_totals: Option<UsageTotals> = None;
    let mut current_model: Option<String> = None;
    let mut last_activity_ms: Option<i64> = None;
    let mut seen_runs: HashSet<i64> = HashSet::new();
    let mut match_known = workspace_path.is_none();
    let mut matches_workspace = workspace_path.is_none();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };
        if line.len() > 512_000 {
            continue;
        }

        let value = match serde_json::from_str::<Value>(&line) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let entry_type = value
            .get("type")
            .and_then(|value| value.as_str())
            .unwrap_or("");

        if entry_type == "session_meta" || entry_type == "turn_context" {
            if let Some(cwd) = extract_cwd(&value) {
                if let Some(filter) = workspace_path {
                    matches_workspace = path_matches_workspace(&cwd, filter);
                    match_known = true;
                    if !matches_workspace {
                        break;
                    }
                }
            }
        }

        if entry_type == "turn_context" {
            if let Some(model) = extract_model_from_turn_context(&value) {
                current_model = Some(model);
            }
            continue;
        }

        if entry_type == "session_meta" {
            continue;
        }

        if !matches_workspace {
            if match_known {
                break;
            }
            continue;
        }

        if !match_known {
            continue;
        }

        if entry_type == "event_msg" || entry_type.is_empty() {
            let payload = value.get("payload").and_then(|value| value.as_object());
            let payload_type = payload
                .and_then(|payload| payload.get("type"))
                .and_then(|value| value.as_str());

            if payload_type == Some("agent_message") {
                if let Some(timestamp_ms) = read_timestamp_ms(&value) {
                    if seen_runs.insert(timestamp_ms) {
                        if let Some(day_key) = day_key_for_timestamp_ms(timestamp_ms) {
                            if let Some(entry) = daily.get_mut(&day_key) {
                                entry.agent_runs += 1;
                            }
                        }
                    }
                    track_activity(daily, &mut last_activity_ms, timestamp_ms);
                }
                continue;
            }

            if payload_type == Some("agent_reasoning") {
                if let Some(timestamp_ms) = read_timestamp_ms(&value) {
                    track_activity(daily, &mut last_activity_ms, timestamp_ms);
                }
                continue;
            }

            if payload_type != Some("token_count") {
                continue;
            }

            let info = payload
                .and_then(|payload| payload.get("info"))
                .and_then(|v| v.as_object());
            let (input, cached, output, used_total) = if let Some(info) = info {
                if let Some(total) = find_usage_map(info, &["total_token_usage", "totalTokenUsage"])
                {
                    (
                        read_i64(total, &["input_tokens", "inputTokens"]),
                        read_i64(
                            total,
                            &[
                                "cached_input_tokens",
                                "cache_read_input_tokens",
                                "cachedInputTokens",
                                "cacheReadInputTokens",
                            ],
                        ),
                        read_i64(total, &["output_tokens", "outputTokens"]),
                        true,
                    )
                } else if let Some(last) =
                    find_usage_map(info, &["last_token_usage", "lastTokenUsage"])
                {
                    (
                        read_i64(last, &["input_tokens", "inputTokens"]),
                        read_i64(
                            last,
                            &[
                                "cached_input_tokens",
                                "cache_read_input_tokens",
                                "cachedInputTokens",
                                "cacheReadInputTokens",
                            ],
                        ),
                        read_i64(last, &["output_tokens", "outputTokens"]),
                        false,
                    )
                } else {
                    continue;
                }
            } else {
                continue;
            };

            let mut delta = UsageTotals {
                input,
                cached,
                output,
            };

            if used_total {
                let prev = previous_totals.unwrap_or_default();
                delta = UsageTotals {
                    input: (input - prev.input).max(0),
                    cached: (cached - prev.cached).max(0),
                    output: (output - prev.output).max(0),
                };
                previous_totals = Some(UsageTotals {
                    input,
                    cached,
                    output,
                });
            } else {
                let mut next = previous_totals.unwrap_or_default();
                next.input += input;
                next.cached += cached;
                next.output += output;
                previous_totals = Some(next);
            }

            if let Some(timestamp_ms) = read_timestamp_ms(&value) {
                if let Some(day_key) = day_key_for_timestamp_ms(timestamp_ms) {
                    if let Some(entry) = daily.get_mut(&day_key) {
                        entry.input += delta.input;
                        entry.cached += delta.cached;
                        entry.output += delta.output;
                    }
                }
                track_activity(daily, &mut last_activity_ms, timestamp_ms);
            }

            if let Some(model) = current_model.clone() {
                let entry = model_totals.entry(model).or_insert(0);
                *entry += delta.input + delta.output;
            }
        }
    }

    Ok(())
}

fn track_activity(
    daily: &mut HashMap<String, DailyTotals>,
    last_activity_ms: &mut Option<i64>,
    timestamp_ms: i64,
) {
    if let Some(last) = *last_activity_ms {
        if timestamp_ms - last > MAX_ACTIVITY_GAP_MS {
            *last_activity_ms = Some(timestamp_ms);
        }
    } else {
        *last_activity_ms = Some(timestamp_ms);
    }

    if let Some(day_key) = day_key_for_timestamp_ms(timestamp_ms) {
        if let Some(entry) = daily.get_mut(&day_key) {
            entry.agent_ms += MAX_ACTIVITY_GAP_MS;
        }
    }
}

fn find_usage_map<'a>(
    map: &'a serde_json::Map<String, Value>,
    keys: &[&str],
) -> Option<&'a serde_json::Map<String, Value>> {
    keys.iter()
        .find_map(|key| map.get(*key).and_then(|value| value.as_object()))
}

fn read_i64(map: &serde_json::Map<String, Value>, keys: &[&str]) -> i64 {
    keys.iter()
        .find_map(|key| map.get(*key))
        .and_then(|value| value.as_i64().or_else(|| value.as_f64().map(|value| value as i64)))
        .unwrap_or(0)
}

fn read_timestamp_ms(value: &Value) -> Option<i64> {
    value
        .get("timestamp")
        .and_then(|value| value.as_i64().or_else(|| value.as_f64().map(|value| value as i64)))
}

fn day_key_for_timestamp_ms(timestamp_ms: i64) -> Option<String> {
    let timestamp = Utc.timestamp_millis_opt(timestamp_ms).single()?;
    Some(timestamp.with_timezone(&Local).format("%Y-%m-%d").to_string())
}

fn make_day_keys(days: u32) -> Vec<String> {
    let now = Local::now();
    (0..days)
        .map(|offset| {
            let day = now - Duration::days(offset as i64);
            day.format("%Y-%m-%d").to_string()
        })
        .collect::<Vec<_>>()
}

fn resolve_codex_sessions_root() -> Option<PathBuf> {
    let Ok(home) = std::env::var("HOME") else {
        return None;
    };
    let path = PathBuf::from(home).join(".codex").join("sessions");
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

fn day_dir_for_key(root: &Path, day_key: &str) -> PathBuf {
    root.join(day_key)
}

fn extract_cwd(value: &Value) -> Option<String> {
    value
        .get("payload")
        .and_then(|value| value.get("cwd"))
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn extract_model_from_turn_context(value: &Value) -> Option<String> {
    value
        .get("payload")
        .and_then(|payload| payload.get("model"))
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn path_matches_workspace(path: &str, workspace: &Path) -> bool {
    let Ok(canonical) = std::fs::canonicalize(path) else {
        return false;
    };
    canonical.starts_with(workspace)
}
