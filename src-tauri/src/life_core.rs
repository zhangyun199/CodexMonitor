use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::types::{WorkspacePurpose, WorkspaceSettings};

const LIFE_PROMPT_FILES: [&str; 4] = [
    "workspace-delivery-finance.md",
    "workspace-food-exercise.md",
    "workspace-media.md",
    "workspace-youtube.md",
];

const LIFE_PROMPT_TAIL: &str = "You are JMWillis's life assistant with full context across all domains.\nAuto-detect the domain from user messages and respond appropriately.";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct DashboardMeta {
    pub(crate) domain: String,
    pub(crate) range: String,
    #[serde(rename = "periodStart")]
    pub(crate) period_start: String,
    #[serde(rename = "periodEnd")]
    pub(crate) period_end: String,
    #[serde(rename = "generatedAt")]
    pub(crate) generated_at: String,
    pub(crate) sources: Vec<String>,
    #[serde(rename = "cacheHit", skip_serializing_if = "Option::is_none")]
    pub(crate) cache_hit: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct DeliveryStats {
    #[serde(rename = "totalEarnings")]
    pub(crate) total_earnings: f64,
    #[serde(rename = "orderCount")]
    pub(crate) order_count: u32,
    #[serde(rename = "activeHours")]
    pub(crate) active_hours: f64,
    #[serde(rename = "totalMiles", skip_serializing_if = "Option::is_none")]
    pub(crate) total_miles: Option<f64>,
    #[serde(rename = "hourlyRate")]
    pub(crate) hourly_rate: f64,
    #[serde(rename = "perMileRate")]
    pub(crate) per_mile_rate: f64,
    #[serde(rename = "avgOrderValue", skip_serializing_if = "Option::is_none")]
    pub(crate) avg_order_value: Option<f64>,
    #[serde(rename = "startingAr", skip_serializing_if = "Option::is_none")]
    pub(crate) starting_ar: Option<f64>,
    #[serde(rename = "endingAr", skip_serializing_if = "Option::is_none")]
    pub(crate) ending_ar: Option<f64>,
    #[serde(rename = "whaleCatches", skip_serializing_if = "Option::is_none")]
    pub(crate) whale_catches: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct DeliveryOrder {
    pub(crate) id: String,
    #[serde(rename = "startedAt")]
    pub(crate) started_at: String,
    #[serde(rename = "merchantName")]
    pub(crate) merchant_name: String,
    pub(crate) payout: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) miles: Option<f64>,
    #[serde(rename = "durationMinutes", skip_serializing_if = "Option::is_none")]
    pub(crate) duration_minutes: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) notes: Option<String>,
    #[serde(rename = "rewardTag", skip_serializing_if = "Option::is_none")]
    pub(crate) reward_tag: Option<String>,
    #[serde(rename = "warningTag", skip_serializing_if = "Option::is_none")]
    pub(crate) warning_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MerchantStats {
    #[serde(rename = "merchantName")]
    pub(crate) merchant_name: String,
    #[serde(rename = "orderCount")]
    pub(crate) order_count: u32,
    #[serde(rename = "totalEarnings")]
    pub(crate) total_earnings: f64,
    #[serde(rename = "avgPayout")]
    pub(crate) avg_payout: f64,
    #[serde(rename = "avgMiles", skip_serializing_if = "Option::is_none")]
    pub(crate) avg_miles: Option<f64>,
    #[serde(rename = "avgPerMile", skip_serializing_if = "Option::is_none")]
    pub(crate) avg_per_mile: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct DeliveryDashboard {
    pub(crate) meta: DashboardMeta,
    pub(crate) stats: DeliveryStats,
    pub(crate) orders: Vec<DeliveryOrder>,
    #[serde(rename = "topMerchants")]
    pub(crate) top_merchants: Vec<MerchantStats>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MediaItem {
    pub(crate) id: String,
    pub(crate) title: String,
    #[serde(rename = "type")]
    pub(crate) media_type: String,
    pub(crate) status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rating: Option<f64>,
    #[serde(rename = "coverUrl", skip_serializing_if = "Option::is_none")]
    pub(crate) cover_url: Option<String>,
    #[serde(rename = "createdAt")]
    pub(crate) created_at: String,
    #[serde(rename = "updatedAt")]
    pub(crate) updated_at: String,
    #[serde(rename = "completedAt", skip_serializing_if = "Option::is_none")]
    pub(crate) completed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MediaLibrary {
    pub(crate) meta: DashboardMeta,
    #[serde(rename = "totalCount")]
    pub(crate) total_count: u32,
    #[serde(rename = "completedCount")]
    pub(crate) completed_count: u32,
    #[serde(rename = "backlogCount")]
    pub(crate) backlog_count: u32,
    #[serde(rename = "avgRating")]
    pub(crate) avg_rating: f64,
    pub(crate) items: Vec<MediaItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct VideoIdea {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) tier: String,
    pub(crate) stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thesis: Option<String>,
    #[serde(rename = "updatedAt")]
    pub(crate) updated_at: String,
    #[serde(rename = "nextAction", skip_serializing_if = "Option::is_none")]
    pub(crate) next_action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct YouTubeDashboard {
    pub(crate) meta: DashboardMeta,
    #[serde(rename = "pipelineStats")]
    pub(crate) pipeline_stats: HashMap<String, u32>,
    #[serde(rename = "sTier")]
    pub(crate) s_tier: Vec<VideoIdea>,
    #[serde(rename = "inProgress")]
    pub(crate) in_progress: Vec<VideoIdea>,
}

#[derive(Debug, Deserialize)]
struct DeliverySessionFrontmatter {
    id: Option<String>,
    date: Option<String>,
    shift: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    hours: Option<f64>,
    mileage: Option<f64>,
    earnings: Option<f64>,
    #[serde(rename = "orders_count")]
    orders_count: Option<f64>,
    #[serde(rename = "starting_ar")]
    starting_ar: Option<f64>,
    #[serde(rename = "ending_ar")]
    ending_ar: Option<f64>,
    #[serde(rename = "whale_catches")]
    whale_catches: Option<f64>,
}

#[derive(Debug, Clone)]
struct DeliverySessionRecord {
    id: String,
    date: NaiveDate,
    hours: f64,
    mileage: Option<f64>,
    earnings: f64,
    orders_count: u32,
    starting_ar: Option<f64>,
    ending_ar: Option<f64>,
    whale_catches: u32,
    orders: Vec<DeliveryOrder>,
}

#[derive(Debug, Deserialize)]
struct DeliveryAggregationRow {
    period: Option<String>,
    period_start: String,
    period_end: String,
    total_earnings: Option<f64>,
    order_count: Option<i64>,
    total_hours: Option<f64>,
    total_miles: Option<f64>,
    hourly_rate: Option<f64>,
    per_mile_rate: Option<f64>,
    computed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeliveryMerchantIndex {
    merchants: HashMap<String, DeliveryMerchantMeta>,
}

#[derive(Debug, Deserialize)]
struct DeliveryMerchantMeta {
    tier: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MediaFrontmatter {
    id: Option<String>,
    title: Option<String>,
    #[serde(rename = "type")]
    media_type: Option<String>,
    status: Option<String>,
    rating: Option<f64>,
    created_at: Option<String>,
    completed_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YouTubeIdeaFrontmatter {
    id: Option<String>,
    title: Option<String>,
    tier: Option<String>,
    stage: Option<String>,
    updated_at: Option<String>,
    created_at: Option<String>,
}

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

pub(crate) fn default_obsidian_root() -> Option<String> {
    if let Ok(value) = std::env::var("OBSIDIAN_ROOT") {
        let trimmed = value.trim();
        if !trimmed.is_empty() && Path::new(trimmed).exists() {
            return Some(trimmed.to_string());
        }
    }
    let fallback = "/Volumes/YouTube 4TB/Obsidian";
    if Path::new(fallback).exists() {
        Some(fallback.to_string())
    } else {
        None
    }
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

pub(crate) async fn build_delivery_dashboard(
    workspace_path: &str,
    obsidian_root: Option<&str>,
    supabase_url: Option<&str>,
    supabase_key: Option<&str>,
    range: &str,
) -> Result<DeliveryDashboard, String> {
    let root = resolve_obsidian_root(workspace_path, obsidian_root);
    if !root.exists() {
        return Err(format!(
            "Obsidian root not found: {}",
            root.to_string_lossy()
        ));
    }
    let today = Utc::now().date_naive();
    let (period, start_date, end_date) = match range {
        "today" => (None, Some(today), Some(today)),
        "week" | "7d" => (Some("week"), Some(today - Duration::days(6)), Some(today)),
        "month" | "30d" => (Some("month"), Some(today - Duration::days(29)), Some(today)),
        "lifetime" => (Some("lifetime"), None, Some(today)),
        _ => (None, None, Some(today)),
    };

    let sessions_dir = root.join("Entities").join("Delivery").join("Sessions");
    if !sessions_dir.exists() {
        return Err(format!(
            "Delivery sessions not found at {}",
            sessions_dir.to_string_lossy()
        ));
    }

    if let Some(period) = period {
        if let (Some(url), Some(key)) = (supabase_url, supabase_key) {
            if let Some(row) = fetch_delivery_aggregation(url, key, period).await? {
                let stats = DeliveryStats {
                    total_earnings: row.total_earnings.unwrap_or(0.0),
                    order_count: row.order_count.unwrap_or(0).max(0) as u32,
                    active_hours: row.total_hours.unwrap_or(0.0),
                    total_miles: row.total_miles,
                    hourly_rate: row.hourly_rate.unwrap_or(0.0),
                    per_mile_rate: row.per_mile_rate.unwrap_or(0.0),
                    avg_order_value: None,
                    starting_ar: None,
                    ending_ar: None,
                    whale_catches: None,
                };
                let meta = DashboardMeta {
                    domain: "delivery".to_string(),
                    range: range.to_string(),
                    period_start: row.period_start,
                    period_end: row.period_end,
                    generated_at: row.computed_at.unwrap_or_else(|| Utc::now().to_rfc3339()),
                    sources: vec!["supabase".to_string()],
                    cache_hit: None,
                };
                return Ok(DeliveryDashboard {
                    meta,
                    stats,
                    orders: Vec::new(),
                    top_merchants: Vec::new(),
                });
            }
        }
    }

    let sessions = load_delivery_sessions(&root);
    let filtered: Vec<_> = sessions
        .into_iter()
        .filter(|session| {
            let after_start = start_date
                .map(|start| session.date >= start)
                .unwrap_or(true);
            let before_end = end_date.map(|end| session.date <= end).unwrap_or(true);
            after_start && before_end
        })
        .collect();

    let mut total_earnings = 0.0;
    let mut total_hours = 0.0;
    let mut total_miles = 0.0;
    let mut has_miles = false;
    let mut order_count = 0u32;
    let mut whale_catches = 0u32;
    let mut orders = Vec::new();

    for session in &filtered {
        total_earnings += session.earnings;
        total_hours += session.hours;
        order_count += session.orders_count;
        if let Some(mileage) = session.mileage {
            total_miles += mileage;
            has_miles = true;
        }
        orders.extend(session.orders.clone());
        whale_catches += session.whale_catches;
    }

    let hourly_rate = if total_hours > 0.0 {
        total_earnings / total_hours
    } else {
        0.0
    };
    let per_mile_rate = if has_miles && total_miles > 0.0 {
        total_earnings / total_miles
    } else {
        0.0
    };
    let avg_order_value = if order_count > 0 {
        Some(total_earnings / order_count as f64)
    } else {
        None
    };

    let mut sorted = filtered.clone();
    sorted.sort_by_key(|session| session.date);
    let starting_ar = sorted.first().and_then(|session| session.starting_ar);
    let ending_ar = sorted.last().and_then(|session| session.ending_ar);

    let stats = DeliveryStats {
        total_earnings,
        order_count,
        active_hours: total_hours,
        total_miles: if has_miles { Some(total_miles) } else { None },
        hourly_rate,
        per_mile_rate,
        avg_order_value,
        starting_ar,
        ending_ar,
        whale_catches: if whale_catches > 0 {
            Some(whale_catches)
        } else {
            None
        },
    };

    let merchant_tiers = load_delivery_merchant_tiers(&root);
    let top_merchants = build_top_merchants(&orders, &merchant_tiers);

    let period_start = start_date
        .or_else(|| filtered.iter().map(|s| s.date).min())
        .unwrap_or(today)
        .to_string();
    let period_end = end_date
        .or_else(|| filtered.iter().map(|s| s.date).max())
        .unwrap_or(today)
        .to_string();

    let meta = DashboardMeta {
        domain: "delivery".to_string(),
        range: range.to_string(),
        period_start,
        period_end,
        generated_at: Utc::now().to_rfc3339(),
        sources: vec!["obsidian".to_string()],
        cache_hit: None,
    };

    Ok(DeliveryDashboard {
        meta,
        stats,
        orders,
        top_merchants,
    })
}

pub(crate) async fn build_media_library(
    workspace_path: &str,
    obsidian_root: Option<&str>,
) -> Result<MediaLibrary, String> {
    let root = resolve_obsidian_root(workspace_path, obsidian_root);
    if !root.exists() {
        return Err(format!(
            "Obsidian root not found: {}",
            root.to_string_lossy()
        ));
    }

    let media_dir = root.join("Entities").join("Media");
    if !media_dir.exists() {
        return Err(format!(
            "Media entries not found at {}",
            media_dir.to_string_lossy()
        ));
    }

    let items = load_media_items(&root);
    let total_count = items.len() as u32;
    let mut completed_count = 0u32;
    let mut backlog_count = 0u32;
    let mut rating_total = 0.0;
    let mut rating_count = 0u32;
    let mut earliest: Option<DateTime<Utc>> = None;
    let mut latest: Option<DateTime<Utc>> = None;

    for item in &items {
        match item.status.as_str() {
            "Completed" => completed_count += 1,
            "Backlog" => backlog_count += 1,
            _ => {}
        }
        if let Some(rating) = item.rating {
            rating_total += rating;
            rating_count += 1;
        }
        if let Some(updated_at) = parse_datetime(&item.updated_at) {
            earliest = match earliest {
                Some(value) if value <= updated_at => Some(value),
                _ => Some(updated_at),
            };
            latest = match latest {
                Some(value) if value >= updated_at => Some(value),
                _ => Some(updated_at),
            };
        }
    }

    let avg_rating = if rating_count > 0 {
        rating_total / rating_count as f64
    } else {
        0.0
    };

    let period_start = earliest
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| Utc::now().date_naive().to_string());
    let period_end = latest
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| Utc::now().date_naive().to_string());

    let meta = DashboardMeta {
        domain: "media".to_string(),
        range: "all".to_string(),
        period_start,
        period_end,
        generated_at: Utc::now().to_rfc3339(),
        sources: vec!["obsidian".to_string()],
        cache_hit: None,
    };

    Ok(MediaLibrary {
        meta,
        total_count,
        completed_count,
        backlog_count,
        avg_rating,
        items,
    })
}

pub(crate) async fn build_youtube_dashboard(
    workspace_path: &str,
    obsidian_root: Option<&str>,
    range: &str,
) -> Result<YouTubeDashboard, String> {
    let root = resolve_obsidian_root(workspace_path, obsidian_root);
    if !root.exists() {
        return Err(format!(
            "Obsidian root not found: {}",
            root.to_string_lossy()
        ));
    }

    let ideas_dir = root.join("Entities").join("YouTube");
    if !ideas_dir.exists() {
        return Err(format!(
            "YouTube ideas not found at {}",
            ideas_dir.to_string_lossy()
        ));
    }

    let ideas = load_youtube_ideas(&root);
    let mut pipeline_stats: HashMap<String, u32> = HashMap::new();
    let mut earliest: Option<DateTime<Utc>> = None;
    let mut latest: Option<DateTime<Utc>> = None;

    for idea in &ideas {
        *pipeline_stats.entry(idea.stage.clone()).or_insert(0) += 1;
        if let Some(updated_at) = idea.updated_at_dt {
            earliest = match earliest {
                Some(value) if value <= updated_at => Some(value),
                _ => Some(updated_at),
            };
            latest = match latest {
                Some(value) if value >= updated_at => Some(value),
                _ => Some(updated_at),
            };
        }
    }

    for stage in [
        "brain_dump",
        "development",
        "outline",
        "evaluation",
        "script",
        "edit",
        "published",
    ] {
        pipeline_stats.entry(stage.to_string()).or_insert(0);
    }

    let period_start = earliest
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| Utc::now().date_naive().to_string());
    let period_end = latest
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| Utc::now().date_naive().to_string());

    let meta = DashboardMeta {
        domain: "youtube".to_string(),
        range: range.to_string(),
        period_start,
        period_end,
        generated_at: Utc::now().to_rfc3339(),
        sources: vec!["obsidian".to_string()],
        cache_hit: None,
    };

    let mut s_tier: Vec<VideoIdea> = ideas
        .iter()
        .filter(|idea| idea.tier == "S")
        .cloned()
        .map(|idea| idea.into_item())
        .collect();
    s_tier.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    s_tier.truncate(8);

    let mut in_progress: Vec<VideoIdea> = ideas
        .iter()
        .filter(|idea| idea.stage != "published" && idea.stage != "brain_dump")
        .cloned()
        .map(|idea| idea.into_item())
        .collect();
    in_progress.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    in_progress.truncate(8);

    Ok(YouTubeDashboard {
        meta,
        pipeline_stats,
        s_tier,
        in_progress,
    })
}

fn resolve_obsidian_root(workspace_path: &str, obsidian_root: Option<&str>) -> PathBuf {
    obsidian_root
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(workspace_path))
}

async fn fetch_delivery_aggregation(
    supabase_url: &str,
    supabase_key: &str,
    period: &str,
) -> Result<Option<DeliveryAggregationRow>, String> {
    let endpoint = format!(
        "{}/rest/v1/delivery_aggregations?period=eq.{}&order=period_start.desc&limit=1",
        supabase_url.trim_end_matches('/'),
        period
    );
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("apikey", supabase_key.parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", supabase_key).parse().unwrap(),
    );
    let resp = Client::new()
        .get(&endpoint)
        .headers(headers)
        .send()
        .await
        .map_err(|err| err.to_string())?;
    if !resp.status().is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("Supabase delivery_aggregations failed: {text}"));
    }
    let rows: Vec<DeliveryAggregationRow> = resp.json().await.map_err(|err| err.to_string())?;
    Ok(rows.into_iter().next())
}

fn load_delivery_sessions(root: &Path) -> Vec<DeliverySessionRecord> {
    let dir = root.join("Entities").join("Delivery").join("Sessions");
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut sessions = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let (frontmatter, body) = split_frontmatter(&content);
        let Some(frontmatter) = frontmatter else {
            continue;
        };
        let Ok(parsed) = serde_yaml::from_str::<DeliverySessionFrontmatter>(&frontmatter) else {
            continue;
        };
        let Some(date_str) = parsed.date else {
            continue;
        };
        let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") else {
            continue;
        };
        let session_id = parsed.id.clone().unwrap_or_else(|| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("session")
                .to_string()
        });
        let orders = parse_orders_table(&body, date, &session_id);
        let orders_count = parsed
            .orders_count
            .map(|value| value.max(0.0) as u32)
            .unwrap_or_else(|| orders.len() as u32);
        sessions.push(DeliverySessionRecord {
            id: session_id,
            date,
            hours: parsed.hours.unwrap_or(0.0),
            mileage: parsed.mileage,
            earnings: parsed.earnings.unwrap_or(0.0),
            orders_count,
            starting_ar: parsed.starting_ar,
            ending_ar: parsed.ending_ar,
            whale_catches: parsed
                .whale_catches
                .map(|value| value.max(0.0) as u32)
                .unwrap_or(0),
            orders,
        });
    }

    sessions
}

fn parse_orders_table(body: &str, date: NaiveDate, session_id: &str) -> Vec<DeliveryOrder> {
    let mut orders = Vec::new();
    let mut in_orders = false;
    let mut row_index = 0;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## Orders") {
            in_orders = true;
            continue;
        }
        if !in_orders {
            continue;
        }
        if trimmed.starts_with("## ") {
            break;
        }
        if !trimmed.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim())
            .collect();
        if cells.is_empty() || cells[0].starts_with('#') || cells[0].starts_with("---") {
            continue;
        }
        if cells.len() < 6 {
            continue;
        }
        let time_value = cells.get(1).copied().unwrap_or_default();
        let app_value = cells.get(2).copied().unwrap_or_default();
        let merchant_value = cells.get(3).copied().unwrap_or_default();
        let status_value = cells.get(4).copied().unwrap_or_default();
        let payout_value = cells.get(5).copied().unwrap_or_default();
        let miles_value = cells.get(6).copied().unwrap_or_default();
        let notes_value = cells.get(9).copied().unwrap_or_default();

        let payout = parse_f64(payout_value);
        if payout.is_none() && status_value.eq_ignore_ascii_case("declined") {
            continue;
        }
        row_index += 1;

        let started_at = build_timestamp(date, time_value);
        orders.push(DeliveryOrder {
            id: format!("{session_id}-{row_index}"),
            started_at,
            merchant_name: merchant_value.to_string(),
            payout: payout.unwrap_or(0.0),
            miles: parse_f64(miles_value),
            duration_minutes: None,
            platform: normalize_platform(app_value),
            notes: normalize_optional_text(notes_value),
            reward_tag: None,
            warning_tag: None,
        });
    }

    orders
}

fn build_top_merchants(
    orders: &[DeliveryOrder],
    tier_map: &HashMap<String, String>,
) -> Vec<MerchantStats> {
    let mut map: HashMap<String, (u32, f64, f64, u32)> = HashMap::new();
    for order in orders {
        if order.payout <= 0.0 {
            continue;
        }
        let entry = map
            .entry(order.merchant_name.clone())
            .or_insert((0, 0.0, 0.0, 0));
        entry.0 += 1;
        entry.1 += order.payout;
        if let Some(miles) = order.miles {
            entry.2 += miles;
            entry.3 += 1;
        }
    }

    let mut merchants: Vec<MerchantStats> = map
        .into_iter()
        .map(
            |(merchant_name, (count, total, miles_total, miles_count))| {
                let avg_payout = if count > 0 { total / count as f64 } else { 0.0 };
                let avg_miles = if miles_count > 0 {
                    Some(miles_total / miles_count as f64)
                } else {
                    None
                };
                let avg_per_mile = avg_miles.and_then(|avg| {
                    if avg > 0.0 {
                        Some(avg_payout / avg)
                    } else {
                        None
                    }
                });
                let tier = tier_map
                    .get(&merchant_name.to_lowercase())
                    .cloned()
                    .or_else(|| tier_map.get(&merchant_name).cloned());
                MerchantStats {
                    merchant_name,
                    order_count: count,
                    total_earnings: total,
                    avg_payout,
                    avg_miles,
                    avg_per_mile,
                    tier,
                }
            },
        )
        .collect();

    merchants.sort_by(|a, b| {
        b.total_earnings
            .partial_cmp(&a.total_earnings)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    merchants.truncate(8);
    merchants
}

fn load_delivery_merchant_tiers(root: &Path) -> HashMap<String, String> {
    let path = root.join("Indexes").join("delivery.merchants.v1.json");
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return HashMap::new(),
    };
    let Ok(parsed) = serde_json::from_str::<DeliveryMerchantIndex>(&content) else {
        return HashMap::new();
    };
    parsed
        .merchants
        .into_iter()
        .filter_map(|(name, meta)| meta.tier.map(|tier| (name.to_lowercase(), tier)))
        .collect()
}

fn load_media_items(root: &Path) -> Vec<MediaItem> {
    let dir = root.join("Entities").join("Media");
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut items = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let (frontmatter, _) = split_frontmatter(&content);
        let Some(frontmatter) = frontmatter else {
            continue;
        };
        let Ok(parsed) = serde_yaml::from_str::<MediaFrontmatter>(&frontmatter) else {
            continue;
        };
        let fallback_title = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("untitled")
            .to_string();
        let title = parsed
            .title
            .clone()
            .unwrap_or_else(|| fallback_title.clone());
        let id = parsed.id.unwrap_or_else(|| fallback_title.clone());
        let media_type = normalize_media_type(parsed.media_type.as_deref());
        let status = normalize_media_status(parsed.status.as_deref());
        let created_at = parsed
            .created_at
            .clone()
            .or_else(|| parsed.updated_at.clone())
            .unwrap_or_else(|| Utc::now().to_rfc3339());
        let updated_at = parsed
            .updated_at
            .clone()
            .or_else(|| Some(created_at.clone()))
            .unwrap_or_else(|| created_at.clone());
        items.push(MediaItem {
            id,
            title,
            media_type,
            status,
            rating: parsed.rating,
            cover_url: None,
            created_at,
            updated_at,
            completed_at: parsed.completed_at,
        });
    }

    items
}

#[derive(Debug, Clone)]
struct YouTubeIdeaRecord {
    id: String,
    title: String,
    tier: String,
    stage: String,
    updated_at: String,
    updated_at_dt: Option<DateTime<Utc>>,
}

impl YouTubeIdeaRecord {
    fn into_item(self) -> VideoIdea {
        VideoIdea {
            id: self.id,
            title: self.title,
            tier: self.tier,
            stage: self.stage,
            thesis: None,
            updated_at: self.updated_at,
            next_action: None,
        }
    }
}

fn load_youtube_ideas(root: &Path) -> Vec<YouTubeIdeaRecord> {
    let dir = root.join("Entities").join("YouTube");
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut ideas = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let (frontmatter, _) = split_frontmatter(&content);
        let Some(frontmatter) = frontmatter else {
            continue;
        };
        let Ok(parsed) = serde_yaml::from_str::<YouTubeIdeaFrontmatter>(&frontmatter) else {
            continue;
        };
        let fallback_title = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("idea")
            .to_string();
        let title = parsed
            .title
            .clone()
            .unwrap_or_else(|| fallback_title.clone());
        let id = parsed.id.unwrap_or_else(|| fallback_title.clone());
        let tier = normalize_idea_tier(parsed.tier.as_deref());
        let stage = normalize_pipeline_stage(parsed.stage.as_deref());
        let updated_at = parsed
            .updated_at
            .clone()
            .or_else(|| parsed.created_at.clone())
            .unwrap_or_else(|| Utc::now().to_rfc3339());
        let updated_at_dt = parse_datetime(&updated_at);
        ideas.push(YouTubeIdeaRecord {
            id,
            title,
            tier,
            stage,
            updated_at,
            updated_at_dt,
        });
    }

    ideas
}

fn build_timestamp(date: NaiveDate, time_value: &str) -> String {
    if let Ok(time) = NaiveTime::parse_from_str(time_value, "%H:%M") {
        let dt = date.and_time(time);
        return dt.format("%Y-%m-%dT%H:%M:%S").to_string();
    }
    date.to_string()
}

fn parse_f64(value: &str) -> Option<f64> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "-" {
        return None;
    }
    let cleaned: String = trimmed
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect();
    if cleaned.is_empty() {
        return None;
    }
    cleaned.parse::<f64>().ok()
}

fn normalize_platform(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "-" {
        return None;
    }
    let normalized = match trimmed.to_lowercase().as_str() {
        "dd" | "doordash" => "doordash",
        "ue" | "uber" | "ubereats" => "uber",
        "gh" | "grubhub" => "grubhub",
        _ => "other",
    };
    Some(normalized.to_string())
}

fn normalize_optional_text(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "-" {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn normalize_media_type(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim().to_lowercase();
    match trimmed.as_str() {
        "film" | "movie" => "Film",
        "tv" | "series" => "TV",
        "anime" => "Anime",
        "game" | "games" => "Game",
        "book" | "books" => "Book",
        "youtube" | "yt" => "YouTube",
        _ => "Film",
    }
    .to_string()
}

fn normalize_media_status(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim().to_lowercase();
    match trimmed.as_str() {
        "completed" | "complete" => "Completed",
        "backlog" | "queue" | "queued" => "Backlog",
        _ => "Backlog",
    }
    .to_string()
}

fn normalize_idea_tier(value: Option<&str>) -> String {
    match value.unwrap_or("C").trim().to_uppercase().as_str() {
        "S" => "S",
        "A" => "A",
        "B" => "B",
        _ => "C",
    }
    .to_string()
}

fn normalize_pipeline_stage(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim().to_lowercase();
    match trimmed.as_str() {
        "idea" | "brain_dump" => "brain_dump",
        "notes" | "development" => "development",
        "outline" | "outlining" => "outline",
        "draft" | "evaluation" => "evaluation",
        "script" | "scripting" => "script",
        "ready" | "edit" | "editing" => "edit",
        "published" => "published",
        _ => "brain_dump",
    }
    .to_string()
}

fn parse_datetime(value: &str) -> Option<DateTime<Utc>> {
    if let Ok(parsed) = DateTime::parse_from_rfc3339(value) {
        return Some(parsed.with_timezone(&Utc));
    }
    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        if let Some(datetime) = date.and_hms_opt(0, 0, 0) {
            return Some(Utc.from_utc_datetime(&datetime));
        }
    }
    None
}

fn split_frontmatter(content: &str) -> (Option<String>, String) {
    let mut frontmatter = Vec::new();
    let mut body = Vec::new();
    let mut in_frontmatter = false;
    let mut frontmatter_done = false;

    for line in content.lines() {
        if line.trim() == "---" {
            if !in_frontmatter {
                in_frontmatter = true;
                continue;
            }
            if in_frontmatter {
                frontmatter_done = true;
                in_frontmatter = false;
                continue;
            }
        }

        if in_frontmatter {
            frontmatter.push(line);
        } else if frontmatter_done {
            body.push(line);
        }
    }

    let frontmatter = if frontmatter.is_empty() {
        None
    } else {
        Some(frontmatter.join("\n"))
    };
    (frontmatter, body.join("\n"))
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
