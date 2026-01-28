use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeZone, Utc};
use reqwest::{Client, Url};
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct NutritionStats {
    pub(crate) calories: f64,
    pub(crate) protein: f64,
    pub(crate) carbs: f64,
    pub(crate) fat: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fiber: Option<f64>,
    #[serde(rename = "mealCount")]
    pub(crate) meal_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MealEntry {
    pub(crate) id: String,
    pub(crate) timestamp: String,
    #[serde(rename = "mealType")]
    pub(crate) meal_type: String,
    pub(crate) description: String,
    pub(crate) foods: Vec<String>,
    #[serde(rename = "estimatedCalories", skip_serializing_if = "Option::is_none")]
    pub(crate) estimated_calories: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct NutritionDashboard {
    pub(crate) meta: DashboardMeta,
    pub(crate) stats: NutritionStats,
    pub(crate) meals: Vec<MealEntry>,
    #[serde(rename = "weeklyTrend", skip_serializing_if = "Option::is_none")]
    pub(crate) weekly_trend: Option<HashMap<String, f64>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct ExerciseStats {
    #[serde(rename = "workoutCount")]
    pub(crate) workout_count: u32,
    #[serde(rename = "walkingMiles")]
    pub(crate) walking_miles: f64,
    #[serde(rename = "activeDays")]
    pub(crate) active_days: u32,
    #[serde(rename = "currentStreak")]
    pub(crate) current_streak: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ExerciseEntry {
    pub(crate) id: String,
    pub(crate) timestamp: String,
    #[serde(rename = "type")]
    pub(crate) entry_type: String,
    pub(crate) description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) miles: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) duration: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ExerciseDashboard {
    pub(crate) meta: DashboardMeta,
    pub(crate) stats: ExerciseStats,
    pub(crate) entries: Vec<ExerciseEntry>,
    #[serde(rename = "byType")]
    pub(crate) by_type: HashMap<String, u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Bill {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) amount: f64,
    #[serde(rename = "dueDay")]
    pub(crate) due_day: u32,
    pub(crate) frequency: String,
    pub(crate) category: String,
    #[serde(rename = "autoPay")]
    pub(crate) auto_pay: bool,
    #[serde(rename = "nextDueDate")]
    pub(crate) next_due_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct FinanceStats {
    #[serde(rename = "monthlyTotal")]
    pub(crate) monthly_total: f64,
    #[serde(rename = "dueSoonCount")]
    pub(crate) due_soon_count: u32,
    #[serde(rename = "autoPayCount")]
    pub(crate) auto_pay_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct FinanceDashboard {
    pub(crate) meta: DashboardMeta,
    pub(crate) stats: FinanceStats,
    pub(crate) bills: Vec<Bill>,
    #[serde(rename = "byCategory")]
    pub(crate) by_category: HashMap<String, f64>,
    #[serde(rename = "statusMessage", skip_serializing_if = "Option::is_none")]
    pub(crate) status_message: Option<String>,
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
pub(crate) struct YouTubeIdea {
    pub(crate) id: String,
    pub(crate) title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) slug: Option<String>,
    pub(crate) tier: String,
    pub(crate) stage: String,
    #[serde(rename = "createdAt")]
    pub(crate) created_at: String,
    #[serde(rename = "updatedAt")]
    pub(crate) updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct YouTubeLibrary {
    pub(crate) meta: DashboardMeta,
    #[serde(rename = "totalCount")]
    pub(crate) total_count: u32,
    #[serde(rename = "inProgressCount")]
    pub(crate) in_progress_count: u32,
    #[serde(rename = "publishedCount")]
    pub(crate) published_count: u32,
    pub(crate) items: Vec<YouTubeIdea>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaCoverSummary {
    pub total: u32,
    pub found: u32,
    pub skipped: u32,
    pub failed: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MediaCoverEntry {
    #[serde(rename = "coverUrl")]
    cover_url: String,
    source: String,
    #[serde(rename = "fetchedAt")]
    fetched_at: String,
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

#[derive(Debug, Deserialize)]
struct FoodFrontmatter {
    name: Option<String>,
    calories: Option<f64>,
    protein: Option<f64>,
    carbs: Option<f64>,
    fat: Option<f64>,
    fiber: Option<f64>,
    category: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BillFrontmatter {
    name: Option<String>,
    amount: Option<f64>,
    #[serde(rename = "due_day")]
    due_day: Option<u32>,
    frequency: Option<String>,
    category: Option<String>,
    #[serde(rename = "auto_pay")]
    auto_pay: Option<bool>,
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

#[derive(Debug, Clone)]
struct FoodNutrition {
    name: String,
    calories: f64,
    protein: f64,
    carbs: f64,
    fat: f64,
    fiber: f64,
    category: Option<String>,
}

#[derive(Debug, Clone)]
struct MealParseRecord {
    date: NaiveDate,
    timestamp: String,
    description: String,
    meal_type: String,
    foods: Vec<String>,
    estimated_calories: Option<f64>,
    calories: f64,
    protein: f64,
    carbs: f64,
    fat: f64,
    fiber: f64,
}

#[derive(Debug, Clone)]
struct ExerciseParseRecord {
    date: NaiveDate,
    timestamp: String,
    description: String,
    entry_type: String,
    miles: Option<f64>,
    duration: Option<f64>,
}

#[derive(Debug, Clone)]
struct BillRecord {
    bill: Bill,
    monthly_equivalent: f64,
    due_date: NaiveDate,
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
    cover_url: Option<String>,
    url: Option<String>,
    youtube_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YouTubeIdeaFrontmatter {
    id: Option<String>,
    title: Option<String>,
    slug: Option<String>,
    tier: Option<String>,
    stage: Option<String>,
    updated_at: Option<String>,
    created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TmdbSearchResponse {
    results: Vec<TmdbResult>,
}

#[derive(Debug, Deserialize)]
struct TmdbResult {
    poster_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenLibrarySearchResponse {
    docs: Vec<OpenLibraryDoc>,
}

#[derive(Debug, Deserialize)]
struct OpenLibraryDoc {
    cover_i: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct IgdbTokenResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct IgdbGameResult {
    cover: Option<IgdbCover>,
}

#[derive(Debug, Deserialize)]
struct IgdbCover {
    image_id: String,
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

pub(crate) async fn build_nutrition_dashboard(
    workspace_path: &str,
    obsidian_root: Option<&str>,
    range: &str,
) -> Result<NutritionDashboard, String> {
    let root = resolve_obsidian_root(workspace_path, obsidian_root);
    if !root.exists() {
        return Err(format!(
            "Obsidian root not found: {}",
            root.to_string_lossy()
        ));
    }
    let today = Utc::now().date_naive();
    let (start_date, end_date) = match range {
        "today" => (Some(today), Some(today)),
        "week" | "7d" => (Some(today - Duration::days(6)), Some(today)),
        "month" | "30d" => (Some(today - Duration::days(29)), Some(today)),
        "lifetime" => (None, Some(today)),
        _ => (None, Some(today)),
    };

    let food_map = load_food_library(&root);
    let meals = load_meal_entries(&root, start_date, end_date, &food_map);

    let mut stats = NutritionStats::default();
    let mut fiber_total = 0.0;
    let mut has_fiber = false;
    for meal in &meals {
        stats.calories += meal.calories;
        stats.protein += meal.protein;
        stats.carbs += meal.carbs;
        stats.fat += meal.fat;
        if meal.fiber > 0.0 {
            fiber_total += meal.fiber;
            has_fiber = true;
        }
    }
    stats.meal_count = meals.len() as u32;
    if has_fiber {
        stats.fiber = Some(fiber_total);
    }

    let period_start = start_date
        .or_else(|| meals.iter().map(|meal| meal.date).min())
        .unwrap_or(today)
        .to_string();
    let period_end = end_date
        .or_else(|| meals.iter().map(|meal| meal.date).max())
        .unwrap_or(today)
        .to_string();

    let weekly_trend = build_weekly_calorie_trend(&meals, end_date.unwrap_or(today));

    let meta = DashboardMeta {
        domain: "nutrition".to_string(),
        range: range.to_string(),
        period_start,
        period_end,
        generated_at: Utc::now().to_rfc3339(),
        sources: vec!["obsidian".to_string()],
        cache_hit: None,
    };

    let meals = meals
        .into_iter()
        .enumerate()
        .map(|(index, meal)| MealEntry {
            id: format!("meal-{}-{}", meal.date, index + 1),
            timestamp: meal.timestamp,
            meal_type: meal.meal_type,
            description: meal.description,
            foods: meal.foods,
            estimated_calories: meal.estimated_calories,
        })
        .collect();

    Ok(NutritionDashboard {
        meta,
        stats,
        meals,
        weekly_trend,
    })
}

pub(crate) async fn build_exercise_dashboard(
    workspace_path: &str,
    obsidian_root: Option<&str>,
    range: &str,
) -> Result<ExerciseDashboard, String> {
    let root = resolve_obsidian_root(workspace_path, obsidian_root);
    if !root.exists() {
        return Err(format!(
            "Obsidian root not found: {}",
            root.to_string_lossy()
        ));
    }
    let today = Utc::now().date_naive();
    let (start_date, end_date) = match range {
        "today" => (Some(today), Some(today)),
        "week" | "7d" => (Some(today - Duration::days(6)), Some(today)),
        "month" | "30d" => (Some(today - Duration::days(29)), Some(today)),
        "lifetime" => (None, Some(today)),
        _ => (None, Some(today)),
    };

    let entries = load_exercise_entries(&root, start_date, end_date);
    let all_activity_dates = load_activity_dates(&root);

    let mut stats = ExerciseStats::default();
    let mut by_type: HashMap<String, u32> = HashMap::new();
    let mut active_days: HashMap<NaiveDate, bool> = HashMap::new();

    for entry in &entries {
        let counter = by_type.entry(entry.entry_type.clone()).or_insert(0);
        *counter += 1;
        active_days.insert(entry.date, true);
        if entry.entry_type == "walk" {
            if let Some(miles) = entry.miles {
                stats.walking_miles += miles;
            }
        } else {
            stats.workout_count += 1;
        }
    }

    stats.active_days = active_days.len() as u32;
    stats.current_streak = compute_activity_streak(today, &all_activity_dates);

    let period_start = start_date
        .or_else(|| entries.iter().map(|entry| entry.date).min())
        .unwrap_or(today)
        .to_string();
    let period_end = end_date
        .or_else(|| entries.iter().map(|entry| entry.date).max())
        .unwrap_or(today)
        .to_string();

    let meta = DashboardMeta {
        domain: "exercise".to_string(),
        range: range.to_string(),
        period_start,
        period_end,
        generated_at: Utc::now().to_rfc3339(),
        sources: vec!["obsidian".to_string()],
        cache_hit: None,
    };

    let entries = entries
        .into_iter()
        .enumerate()
        .map(|(index, entry)| ExerciseEntry {
            id: format!("exercise-{}-{}", entry.date, index + 1),
            timestamp: entry.timestamp,
            entry_type: entry.entry_type,
            description: entry.description,
            miles: entry.miles,
            duration: entry.duration,
        })
        .collect();

    Ok(ExerciseDashboard {
        meta,
        stats,
        entries,
        by_type,
    })
}

pub(crate) async fn build_finance_dashboard(
    workspace_path: &str,
    obsidian_root: Option<&str>,
    range: &str,
) -> Result<FinanceDashboard, String> {
    let root = resolve_obsidian_root(workspace_path, obsidian_root);
    if !root.exists() {
        return Err(format!(
            "Obsidian root not found: {}",
            root.to_string_lossy()
        ));
    }

    let today = Utc::now().date_naive();
    let bills_dir = root.join("Entities").join("Finance").join("Bills");
    let bill_records = load_bill_records(&bills_dir, today);

    let period_start = today.to_string();
    let period_end = today.to_string();
    let meta = DashboardMeta {
        domain: "finance".to_string(),
        range: range.to_string(),
        period_start,
        period_end,
        generated_at: Utc::now().to_rfc3339(),
        sources: vec!["obsidian".to_string()],
        cache_hit: None,
    };

    if bill_records.is_empty() && bills_dir.exists() {
        return Ok(FinanceDashboard {
            meta,
            stats: FinanceStats::default(),
            bills: Vec::new(),
            by_category: HashMap::new(),
            status_message: Some(
                "Coming soon — needs data migration. Expected YAML frontmatter:\nname: \"Rent\"\namount: 1200\ndue_day: 1\nfrequency: \"monthly\"\ncategory: \"housing\"\nauto_pay: true"
                    .to_string(),
            ),
        });
    }

    let mut stats = FinanceStats::default();
    let mut by_category: HashMap<String, f64> = HashMap::new();
    let mut bills: Vec<Bill> = Vec::new();

    for record in &bill_records {
        stats.monthly_total += record.monthly_equivalent;
        if record.bill.auto_pay {
            stats.auto_pay_count += 1;
        }
        let entry = by_category
            .entry(record.bill.category.clone())
            .or_insert(0.0);
        *entry += record.monthly_equivalent;
        if record.due_date >= today && record.due_date <= today + Duration::days(7) {
            stats.due_soon_count += 1;
        }
        bills.push(record.bill.clone());
    }

    bills.sort_by_key(|bill| bill.next_due_date.clone());

    Ok(FinanceDashboard {
        meta,
        stats,
        bills,
        by_category,
        status_message: None,
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

    let mut records = load_media_items(&root);
    let cache = load_media_cover_cache(&root);
    let total_count = records.len() as u32;
    let mut completed_count = 0u32;
    let mut backlog_count = 0u32;
    let mut rating_total = 0.0;
    let mut rating_count = 0u32;
    let mut earliest: Option<DateTime<Utc>> = None;
    let mut latest: Option<DateTime<Utc>> = None;

    for record in &mut records {
        if record.item.cover_url.is_none() {
            if let Some(entry) = cache.get(&record.item.id) {
                record.item.cover_url = Some(entry.cover_url.clone());
            }
        }
        match record.item.status.as_str() {
            "Completed" => completed_count += 1,
            "Backlog" => backlog_count += 1,
            _ => {}
        }
        if let Some(rating) = record.item.rating {
            rating_total += rating;
            rating_count += 1;
        }
        if let Some(updated_at) = parse_datetime(&record.item.updated_at) {
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

    let items = records.into_iter().map(|record| record.item).collect();

    Ok(MediaLibrary {
        meta,
        total_count,
        completed_count,
        backlog_count,
        avg_rating,
        items,
    })
}

pub(crate) async fn build_youtube_library(
    workspace_path: &str,
    obsidian_root: Option<&str>,
) -> Result<YouTubeLibrary, String> {
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
    let mut earliest: Option<DateTime<Utc>> = None;
    let mut latest: Option<DateTime<Utc>> = None;
    let mut in_progress_count = 0u32;
    let mut published_count = 0u32;

    for idea in &ideas {
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
        if idea.stage == "published" {
            published_count += 1;
        } else if idea.stage != "idea" {
            in_progress_count += 1;
        }
    }

    let period_start = earliest
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| Utc::now().date_naive().to_string());
    let period_end = latest
        .map(|dt| dt.date_naive().to_string())
        .unwrap_or_else(|| Utc::now().date_naive().to_string());

    let meta = DashboardMeta {
        domain: "youtube".to_string(),
        range: "all".to_string(),
        period_start,
        period_end,
        generated_at: Utc::now().to_rfc3339(),
        sources: vec!["obsidian".to_string()],
        cache_hit: None,
    };
    let total_count = ideas.len() as u32;
    let items = ideas.into_iter().map(|idea| idea.into_item()).collect();

    Ok(YouTubeLibrary {
        meta,
        total_count,
        in_progress_count,
        published_count,
        items,
    })
}

pub async fn enrich_media_covers(
    workspace_path: &str,
    obsidian_root: Option<&str>,
    tmdb_api_key: Option<&str>,
    igdb_client_id: Option<&str>,
    igdb_client_secret: Option<&str>,
) -> Result<MediaCoverSummary, String> {
    let root = resolve_obsidian_root(workspace_path, obsidian_root);
    if !root.exists() {
        return Err(format!(
            "Obsidian root not found: {}",
            root.to_string_lossy()
        ));
    }

    let records = load_media_items(&root);
    let total = records.len() as u32;
    let mut cache = load_media_cover_cache(&root);
    let mut found = 0u32;
    let mut skipped = 0u32;
    let mut failed = 0u32;

    let igdb_token = if records
        .iter()
        .any(|record| record.item.media_type == "Game")
    {
        match (igdb_client_id, igdb_client_secret) {
            (Some(id), Some(secret)) if !id.is_empty() && !secret.is_empty() => {
                fetch_igdb_token(id, secret).await?
            }
            _ => String::new(),
        }
    } else {
        String::new()
    };

    for record in records {
        if cache.contains_key(&record.item.id) || record.item.cover_url.is_some() {
            skipped += 1;
            continue;
        }
        let maybe_cover = match record.item.media_type.as_str() {
            "Film" => fetch_tmdb_cover(&record.item.title, "movie", tmdb_api_key).await?,
            "TV" => fetch_tmdb_cover(&record.item.title, "tv", tmdb_api_key).await?,
            "Anime" => {
                let cover = fetch_tmdb_cover(&record.item.title, "tv", tmdb_api_key).await?;
                if cover.is_some() {
                    cover
                } else {
                    fetch_tmdb_cover(&record.item.title, "movie", tmdb_api_key).await?
                }
            }
            "Book" => fetch_open_library_cover(&record.item.title).await?,
            "Game" => fetch_igdb_cover(&record.item.title, igdb_client_id, &igdb_token).await?,
            "YouTube" => fetch_youtube_cover(record.youtube_id.as_deref(), record.url.as_deref()),
            _ => None,
        };

        if let Some((cover_url, source)) = maybe_cover {
            cache.insert(
                record.item.id,
                MediaCoverEntry {
                    cover_url,
                    source,
                    fetched_at: Utc::now().to_rfc3339(),
                },
            );
            found += 1;
        } else {
            failed += 1;
        }
    }

    write_media_cover_cache(&root, &cache)?;

    Ok(MediaCoverSummary {
        total,
        found,
        skipped,
        failed,
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

#[derive(Debug, Clone)]
struct MediaRecord {
    item: MediaItem,
    url: Option<String>,
    youtube_id: Option<String>,
}

fn load_media_items(root: &Path) -> Vec<MediaRecord> {
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
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("");
        if stem.starts_with('_') {
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
        let fallback_title = if stem.is_empty() {
            "untitled".to_string()
        } else {
            stem.to_string()
        };
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
        let item = MediaItem {
            id,
            title,
            media_type,
            status,
            rating: parsed.rating,
            cover_url: parsed.cover_url,
            created_at,
            updated_at,
            completed_at: parsed.completed_at,
        };
        items.push(MediaRecord {
            item,
            url: parsed.url,
            youtube_id: parsed.youtube_id,
        });
    }

    items
}

fn media_cover_cache_path(root: &Path) -> PathBuf {
    root.join("Indexes").join("media.covers.v1.json")
}

fn load_media_cover_cache(root: &Path) -> HashMap<String, MediaCoverEntry> {
    let path = media_cover_cache_path(root);
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return HashMap::new(),
    };
    serde_json::from_str::<HashMap<String, MediaCoverEntry>>(&content).unwrap_or_default()
}

fn write_media_cover_cache(
    root: &Path,
    cache: &HashMap<String, MediaCoverEntry>,
) -> Result<(), String> {
    let path = media_cover_cache_path(root);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let payload = serde_json::to_string_pretty(cache).map_err(|err| err.to_string())?;
    std::fs::write(path, payload).map_err(|err| err.to_string())?;
    Ok(())
}

#[derive(Debug, Clone)]
struct YouTubeIdeaRecord {
    id: String,
    title: String,
    slug: Option<String>,
    tier: String,
    stage: String,
    created_at: String,
    updated_at: String,
    updated_at_dt: Option<DateTime<Utc>>,
}

impl YouTubeIdeaRecord {
    fn into_item(self) -> YouTubeIdea {
        YouTubeIdea {
            id: self.id,
            title: self.title,
            slug: self.slug,
            tier: self.tier,
            stage: self.stage,
            created_at: self.created_at,
            updated_at: self.updated_at,
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
        let stage = normalize_youtube_stage(parsed.stage.as_deref());
        let created_at = parsed
            .created_at
            .clone()
            .unwrap_or_else(|| Utc::now().date_naive().to_string());
        let updated_at = parsed
            .updated_at
            .clone()
            .or_else(|| Some(created_at.clone()))
            .unwrap_or_else(|| created_at.clone());
        let updated_at_dt = parse_datetime(&updated_at);
        ideas.push(YouTubeIdeaRecord {
            id,
            title,
            slug: parsed.slug,
            tier,
            stage,
            created_at,
            updated_at,
            updated_at_dt,
        });
    }

    ideas
}

fn load_food_library(root: &Path) -> HashMap<String, FoodNutrition> {
    let dir = root.join("Entities").join("Food");
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return HashMap::new(),
    };

    let mut foods = HashMap::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let fallback_name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("food")
            .to_string();

        let (frontmatter, body) = split_frontmatter(&content);
        let nutrition = if let Some(frontmatter) = frontmatter {
            if let Ok(parsed) = serde_yaml::from_str::<FoodFrontmatter>(&frontmatter) {
                if parsed.calories.is_some()
                    || parsed.protein.is_some()
                    || parsed.carbs.is_some()
                    || parsed.fat.is_some()
                {
                    Some(FoodNutrition {
                        name: parsed.name.clone().unwrap_or_else(|| fallback_name.clone()),
                        calories: parsed.calories.unwrap_or(0.0),
                        protein: parsed.protein.unwrap_or(0.0),
                        carbs: parsed.carbs.unwrap_or(0.0),
                        fat: parsed.fat.unwrap_or(0.0),
                        fiber: parsed.fiber.unwrap_or(0.0),
                        category: parsed.category,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
        .or_else(|| parse_food_table(&body, &fallback_name));

        if let Some(nutrition) = nutrition {
            let key = normalize_food_key(&nutrition.name);
            foods.insert(key, nutrition.clone());
            let stem_key = normalize_food_key(&fallback_name);
            foods.insert(stem_key, nutrition);
        }
    }

    foods
}

fn parse_food_table(content: &str, fallback_name: &str) -> Option<FoodNutrition> {
    let mut calories = None;
    let mut protein = None;
    let mut carbs = None;
    let mut fat = None;
    let mut fiber = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim())
            .collect();
        if cells.len() < 2 {
            continue;
        }
        match cells[0].to_lowercase().as_str() {
            "calories" => calories = parse_f64(cells[1]),
            "protein" => protein = parse_f64(cells[1]),
            "carbs" | "carbohydrates" => carbs = parse_f64(cells[1]),
            "fat" => fat = parse_f64(cells[1]),
            "fiber" => fiber = parse_f64(cells[1]),
            _ => {}
        }
    }

    if calories.is_none() && protein.is_none() && carbs.is_none() && fat.is_none() {
        return None;
    }

    Some(FoodNutrition {
        name: fallback_name.to_string(),
        calories: calories.unwrap_or(0.0),
        protein: protein.unwrap_or(0.0),
        carbs: carbs.unwrap_or(0.0),
        fat: fat.unwrap_or(0.0),
        fiber: fiber.unwrap_or(0.0),
        category: None,
    })
}

fn load_meal_entries(
    root: &Path,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    food_map: &HashMap<String, FoodNutrition>,
) -> Vec<MealParseRecord> {
    let mut meals = Vec::new();
    let files = list_stream_files(root);

    for file in files {
        let year = match stream_year_from_path(&file) {
            Some(value) => value,
            None => continue,
        };
        let content = match std::fs::read_to_string(&file) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let mut current_date: Option<NaiveDate> = None;
        for line in content.lines() {
            if let Some(date) = parse_stream_date_heading(line, year) {
                current_date = Some(date);
                continue;
            }
            let Some(date) = current_date else {
                continue;
            };
            if !date_in_range(date, start_date, end_date) {
                continue;
            }
            let line_text = extract_stream_line_text(line);
            if let Some(meal) = parse_meal_entry(&line_text, date, food_map) {
                meals.push(meal);
            }
        }
    }

    meals.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    meals
}

fn load_exercise_entries(
    root: &Path,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Vec<ExerciseParseRecord> {
    let mut entries = Vec::new();
    let files = list_stream_files(root);

    for file in files {
        let year = match stream_year_from_path(&file) {
            Some(value) => value,
            None => continue,
        };
        let content = match std::fs::read_to_string(&file) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let mut current_date: Option<NaiveDate> = None;
        for line in content.lines() {
            if let Some(date) = parse_stream_date_heading(line, year) {
                current_date = Some(date);
                continue;
            }
            let Some(date) = current_date else {
                continue;
            };
            if !date_in_range(date, start_date, end_date) {
                continue;
            }
            let line_text = extract_stream_line_text(line);
            if let Some(entry) = parse_exercise_entry(&line_text, date) {
                entries.push(entry);
            }
        }
    }

    entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    entries
}

fn load_activity_dates(root: &Path) -> HashSet<NaiveDate> {
    let mut dates = HashSet::new();
    let files = list_stream_files(root);

    for file in files {
        let year = match stream_year_from_path(&file) {
            Some(value) => value,
            None => continue,
        };
        let content = match std::fs::read_to_string(&file) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let mut current_date: Option<NaiveDate> = None;
        for line in content.lines() {
            if let Some(date) = parse_stream_date_heading(line, year) {
                current_date = Some(date);
                continue;
            }
            let Some(date) = current_date else {
                continue;
            };
            let line_text = extract_stream_line_text(line);
            if parse_exercise_entry(&line_text, date).is_some() {
                dates.insert(date);
            }
        }
    }

    dates
}

fn build_weekly_calorie_trend(
    meals: &[MealParseRecord],
    end_date: NaiveDate,
) -> Option<HashMap<String, f64>> {
    if meals.is_empty() {
        return None;
    }
    let start_date = end_date - Duration::days(6);
    let mut totals: HashMap<NaiveDate, f64> = HashMap::new();
    for meal in meals {
        if meal.date < start_date || meal.date > end_date {
            continue;
        }
        let entry = totals.entry(meal.date).or_insert(0.0);
        *entry += meal.calories;
    }
    let mut trend = HashMap::new();
    let mut cursor = start_date;
    while cursor <= end_date {
        let value = totals.get(&cursor).copied().unwrap_or(0.0);
        trend.insert(cursor.to_string(), value);
        cursor += Duration::days(1);
    }
    Some(trend)
}

fn compute_activity_streak(today: NaiveDate, dates: &HashSet<NaiveDate>) -> u32 {
    let mut streak = 0u32;
    let mut cursor = today;
    loop {
        if dates.contains(&cursor) {
            streak += 1;
            cursor -= Duration::days(1);
        } else {
            break;
        }
    }
    streak
}

fn load_bill_records(bills_dir: &Path, today: NaiveDate) -> Vec<BillRecord> {
    let entries = match std::fs::read_dir(bills_dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut records = Vec::new();
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
        let Ok(parsed) = serde_yaml::from_str::<BillFrontmatter>(&frontmatter) else {
            continue;
        };

        let name = parsed.name.clone().unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Bill")
                .to_string()
        });
        let Some(amount) = parsed.amount else {
            continue;
        };
        let Some(due_day) = parsed.due_day else {
            continue;
        };
        let frequency = normalize_frequency(parsed.frequency.as_deref());
        let category = parsed
            .category
            .unwrap_or_else(|| "uncategorized".to_string());
        let auto_pay = parsed.auto_pay.unwrap_or(false);
        let due_date = compute_next_due_date(today, due_day, &frequency);
        let bill = Bill {
            id: path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or(&name)
                .to_string(),
            name,
            amount,
            due_day,
            frequency: frequency.clone(),
            category,
            auto_pay,
            next_due_date: due_date.to_string(),
        };
        let monthly_equivalent = monthly_equivalent_amount(amount, &frequency);
        records.push(BillRecord {
            bill,
            monthly_equivalent,
            due_date,
        });
    }

    records
}

fn list_stream_files(root: &Path) -> Vec<PathBuf> {
    let dir = root.join("Stream");
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("md"))
        .collect();
    files.sort();
    files
}

fn stream_year_from_path(path: &Path) -> Option<i32> {
    let stem = path.file_stem()?.to_str()?;
    let mut parts = stem.split('-');
    let year = parts.next()?.parse::<i32>().ok()?;
    Some(year)
}

fn parse_stream_date_heading(line: &str, year: i32) -> Option<NaiveDate> {
    let trimmed = line.trim();
    if !trimmed.starts_with("## ") {
        return None;
    }
    let parts: Vec<&str> = trimmed
        .trim_start_matches("## ")
        .split_whitespace()
        .collect();
    if parts.len() < 3 {
        return None;
    }
    let month = match parts[1].to_lowercase().as_str() {
        "jan" => 1,
        "feb" => 2,
        "mar" => 3,
        "apr" => 4,
        "may" => 5,
        "jun" => 6,
        "jul" => 7,
        "aug" => 8,
        "sep" | "sept" => 9,
        "oct" => 10,
        "nov" => 11,
        "dec" => 12,
        _ => return None,
    };
    let day_raw = parts[2].trim_matches(|c: char| !c.is_ascii_digit());
    let day = day_raw.parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}

fn extract_stream_line_text(line: &str) -> String {
    let trimmed = line.trim();
    if trimmed.starts_with('|') {
        let cells: Vec<&str> = trimmed.split('|').map(|cell| cell.trim()).collect();
        if cells.len() >= 3 {
            return cells[2].to_string();
        }
    }
    trimmed.to_string()
}

fn date_in_range(
    date: NaiveDate,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> bool {
    let after_start = start_date.map(|start| date >= start).unwrap_or(true);
    let before_end = end_date.map(|end| date <= end).unwrap_or(true);
    after_start && before_end
}

fn parse_meal_entry(
    content: &str,
    date: NaiveDate,
    food_map: &HashMap<String, FoodNutrition>,
) -> Option<MealParseRecord> {
    let emoji = "🍽️";
    let idx = content.find(emoji)?;
    let (prefix, suffix) = content.split_at(idx);
    let suffix = suffix.trim_start_matches(emoji);
    let time = extract_time_from_text(prefix).or_else(|| extract_time_from_text(suffix));
    let description = clean_stream_description(suffix);
    if description.is_empty() {
        return None;
    }
    let meal_type = detect_meal_type(&description);
    let foods = extract_food_links(&description, food_map);

    let (calories, protein, carbs, fat, fiber) = foods
        .iter()
        .filter_map(|link| food_name_from_link(link))
        .filter_map(|name| food_map.get(&normalize_food_key(name)))
        .fold((0.0, 0.0, 0.0, 0.0, 0.0), |mut acc, food| {
            acc.0 += food.calories;
            acc.1 += food.protein;
            acc.2 += food.carbs;
            acc.3 += food.fat;
            acc.4 += food.fiber;
            acc
        });

    let estimated_calories = if calories > 0.0 { Some(calories) } else { None };
    Some(MealParseRecord {
        date,
        timestamp: format_timestamp(date, time),
        description,
        meal_type,
        foods,
        estimated_calories,
        calories,
        protein,
        carbs,
        fat,
        fiber,
    })
}

fn parse_exercise_entry(content: &str, date: NaiveDate) -> Option<ExerciseParseRecord> {
    let emoji = if content.contains('🚶') {
        "🚶"
    } else if content.contains("🏋️") {
        "🏋️"
    } else if content.contains('🏃') {
        "🏃"
    } else if content.contains('🚴') {
        "🚴"
    } else {
        return None;
    };
    let idx = content.find(emoji)?;
    let (prefix, suffix) = content.split_at(idx);
    let suffix = suffix.trim_start_matches(emoji);
    let time = extract_time_from_text(prefix).or_else(|| extract_time_from_text(suffix));
    let mut description = clean_stream_description(suffix);
    if description.is_empty() {
        description = content.to_string();
    }
    let mut entry_type = if emoji == "🚶" {
        "walk".to_string()
    } else if emoji == "🏃" || emoji == "🚴" {
        "cardio".to_string()
    } else {
        "strength".to_string()
    };
    if entry_type == "strength" {
        let lower = description.to_lowercase();
        if lower.contains("run") || lower.contains("cardio") {
            entry_type = "cardio".to_string();
        }
    }

    let miles = extract_miles(&description);
    let duration = extract_duration_minutes(&description);

    Some(ExerciseParseRecord {
        date,
        timestamp: format_timestamp(date, time),
        description,
        entry_type,
        miles,
        duration,
    })
}

fn extract_food_links(description: &str, food_map: &HashMap<String, FoodNutrition>) -> Vec<String> {
    let mut links = Vec::new();
    let mut search_start = 0;
    while let Some(start) = description[search_start..].find("[[Food/") {
        let link_start = search_start + start;
        let after = link_start + "[[Food/".len();
        if let Some(end) = description[after..].find("]]") {
            let name = &description[after..after + end];
            links.push(format!("[[Food/{}]]", name.trim()));
            search_start = after + end + 2;
        } else {
            break;
        }
    }
    if !links.is_empty() {
        return links;
    }

    let mut raw = description;
    if let Some(idx) = description.find(':') {
        raw = &description[(idx + 1)..];
    }
    let raw = raw.replace(" and ", ",");
    for token in raw.split(',') {
        let cleaned = token.trim();
        if cleaned.is_empty() {
            continue;
        }
        let key = normalize_food_key(cleaned);
        if let Some(food) = food_map.get(&key) {
            links.push(format!("[[Food/{}]]", food.name));
        }
    }
    links
}

fn food_name_from_link(link: &str) -> Option<&str> {
    if let Some(stripped) = link.strip_prefix("[[Food/") {
        return stripped.strip_suffix("]]");
    }
    None
}

fn detect_meal_type(description: &str) -> String {
    let lower = description.to_lowercase();
    if lower.contains("breakfast") {
        "breakfast"
    } else if lower.contains("lunch") {
        "lunch"
    } else if lower.contains("dinner") {
        "dinner"
    } else if lower.contains("snack") {
        "snack"
    } else {
        "snack"
    }
    .to_string()
}

fn clean_stream_description(text: &str) -> String {
    let mut cleaned = text.split('📝').next().unwrap_or(text).trim().to_string();
    cleaned = cleaned.trim_matches('|').trim().to_string();
    cleaned = cleaned
        .trim_start_matches(|c: char| c == '-' || c == '—' || c == '–' || c == ':')
        .trim()
        .to_string();
    cleaned
}

fn normalize_food_key(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
}

fn extract_time_from_text(text: &str) -> Option<NaiveTime> {
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            if i < chars.len() && chars[i] == ':' {
                let hour_str: String = chars[start..i].iter().collect();
                i += 1;
                let minute_start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let minute_str: String = chars[minute_start..i].iter().collect();
                if minute_str.len() != 2 {
                    continue;
                }
                let mut hour: u32 = hour_str.parse().ok()?;
                let minute: u32 = minute_str.parse().ok()?;

                let mut suffix = None;
                let mut j = i;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if j + 1 < chars.len() {
                    let candidate: String = vec![chars[j], chars[j + 1]]
                        .into_iter()
                        .collect::<String>()
                        .to_lowercase();
                    if candidate == "am" || candidate == "pm" {
                        suffix = Some(candidate);
                    }
                }

                if let Some(meridian) = suffix {
                    if meridian == "pm" && hour < 12 {
                        hour += 12;
                    }
                    if meridian == "am" && hour == 12 {
                        hour = 0;
                    }
                }
                return NaiveTime::from_hms_opt(hour, minute, 0);
            }
        }
        i += 1;
    }
    None
}

fn format_timestamp(date: NaiveDate, time: Option<NaiveTime>) -> String {
    if let Some(time) = time {
        return date.and_time(time).format("%Y-%m-%dT%H:%M:%S").to_string();
    }
    date.to_string()
}

fn extract_miles(description: &str) -> Option<f64> {
    let tokens: Vec<&str> = description.split_whitespace().collect();
    for (idx, token) in tokens.iter().enumerate() {
        let lower = token.to_lowercase();
        if lower.contains("mi") {
            if let Some(value) = parse_f64(token) {
                return Some(value);
            }
        }
        if idx + 1 < tokens.len() {
            let next = tokens[idx + 1].to_lowercase();
            if next.starts_with("mi") || next.starts_with("mile") {
                if let Some(value) = parse_f64(token) {
                    return Some(value);
                }
            }
        }
    }
    None
}

fn extract_duration_minutes(description: &str) -> Option<f64> {
    let tokens: Vec<&str> = description.split_whitespace().collect();
    for (idx, token) in tokens.iter().enumerate() {
        let lower = token.to_lowercase();
        if lower.contains("min") {
            if let Some(value) = parse_f64(token) {
                return Some(value);
            }
        }
        if idx + 1 < tokens.len() {
            let next = tokens[idx + 1].to_lowercase();
            if next.starts_with("min") {
                if let Some(value) = parse_f64(token) {
                    return Some(value);
                }
            }
        }
    }
    None
}

fn normalize_frequency(value: Option<&str>) -> String {
    match value.unwrap_or("monthly").trim().to_lowercase().as_str() {
        "weekly" => "weekly".to_string(),
        "annual" | "yearly" => "annual".to_string(),
        _ => "monthly".to_string(),
    }
}

fn monthly_equivalent_amount(amount: f64, frequency: &str) -> f64 {
    match frequency {
        "weekly" => amount * 4.0,
        "annual" => amount / 12.0,
        _ => amount,
    }
}

fn compute_next_due_date(today: NaiveDate, due_day: u32, frequency: &str) -> NaiveDate {
    match frequency {
        "weekly" => next_weekday_date(today, due_day),
        "annual" => {
            let mut year = today.year();
            let month = today.month();
            let mut date = safe_date(year, month, due_day);
            if date < today {
                year += 1;
                date = safe_date(year, month, due_day);
            }
            date
        }
        _ => {
            let mut year = today.year();
            let mut month = today.month();
            let mut date = safe_date(year, month, due_day);
            if date < today {
                if month == 12 {
                    year += 1;
                    month = 1;
                } else {
                    month += 1;
                }
                date = safe_date(year, month, due_day);
            }
            date
        }
    }
}

fn safe_date(year: i32, month: u32, due_day: u32) -> NaiveDate {
    let last_day = last_day_of_month(year, month);
    let day = due_day.min(last_day);
    NaiveDate::from_ymd_opt(year, month, day)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, month, last_day).unwrap())
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let first_next = NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year, month, 28).unwrap());
    (first_next - Duration::days(1)).day()
}

fn next_weekday_date(today: NaiveDate, due_day: u32) -> NaiveDate {
    use chrono::Weekday;
    let target = match due_day {
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        6 => Weekday::Sat,
        7 => Weekday::Sun,
        _ => return today + Duration::days(7),
    };
    let mut date = today;
    for _ in 0..7 {
        if date.weekday() == target {
            break;
        }
        date += Duration::days(1);
    }
    date
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

fn normalize_youtube_stage(value: Option<&str>) -> String {
    let trimmed = value.unwrap_or("").trim().to_lowercase();
    match trimmed.as_str() {
        "idea" => "idea",
        "notes" => "notes",
        "outline" | "outlining" => "outline",
        "draft" => "draft",
        "script" | "scripting" => "script",
        "ready" => "ready",
        "published" => "published",
        _ => "idea",
    }
    .to_string()
}

fn title_variants(title: &str) -> Vec<String> {
    let mut variants = Vec::new();
    let base = collapse_whitespace(title);
    push_variant(&mut variants, base.clone());

    let no_parens = strip_parenthetical(&base);
    push_variant(&mut variants, no_parens.clone());

    let no_season = strip_season_suffix(&base);
    push_variant(&mut variants, no_season.clone());

    let no_parens_no_season = strip_season_suffix(&no_parens);
    push_variant(&mut variants, no_parens_no_season.clone());

    let no_colon = base.replace(':', "");
    push_variant(&mut variants, no_colon);
    let no_colon_space = base.replace(':', " ");
    push_variant(&mut variants, no_colon_space);
    let no_parens_colon = no_parens.replace(':', " ");
    push_variant(&mut variants, no_parens_colon);

    let no_trailing_series = strip_trailing_series_number(&no_parens_no_season);
    push_variant(&mut variants, no_trailing_series);

    variants
}

fn game_title_variants(title: &str) -> Vec<String> {
    let mut variants = title_variants(title);
    let trimmed = title.trim();
    let upper = trimmed.to_uppercase();
    if upper.starts_with("MGS ") {
        let rest = trimmed[3..].trim();
        push_variant(&mut variants, format!("Metal Gear Solid {}", rest.trim()));
    }
    variants
}

fn push_variant(variants: &mut Vec<String>, value: String) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return;
    }
    if variants
        .iter()
        .any(|existing| existing.eq_ignore_ascii_case(trimmed))
    {
        return;
    }
    variants.push(trimmed.to_string());
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn strip_parenthetical(value: &str) -> String {
    let mut result = String::new();
    let mut depth = 0u32;
    for ch in value.chars() {
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {
                if depth == 0 {
                    result.push(ch);
                }
            }
        }
    }
    collapse_whitespace(&result)
}

fn strip_season_suffix(value: &str) -> String {
    let trimmed = value.trim();
    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    if tokens.len() < 2 {
        return trimmed.to_string();
    }
    let last = tokens.last().unwrap().to_lowercase();
    let second_last = tokens[tokens.len() - 2].to_lowercase();
    let is_season_token =
        last.starts_with('s') && last.chars().skip(1).all(|c| c.is_ascii_digit() || c == '-');
    if is_season_token {
        return tokens[..tokens.len() - 1].join(" ");
    }
    if second_last == "season" && last.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return tokens[..tokens.len() - 2].join(" ");
    }
    trimmed.to_string()
}

fn strip_trailing_series_number(value: &str) -> String {
    let trimmed = value.trim();
    let tokens: Vec<&str> = trimmed.split_whitespace().collect();
    if tokens.len() < 2 {
        return trimmed.to_string();
    }
    let last = tokens.last().unwrap();
    if last.chars().all(|c| c.is_ascii_digit()) || is_roman_numeral(last) {
        return tokens[..tokens.len() - 1].join(" ");
    }
    trimmed.to_string()
}

fn is_roman_numeral(token: &str) -> bool {
    let upper = token.to_uppercase();
    !upper.is_empty()
        && upper
            .chars()
            .all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'))
}

async fn fetch_tmdb_cover(
    title: &str,
    media_type: &str,
    tmdb_api_key: Option<&str>,
) -> Result<Option<(String, String)>, String> {
    let Some(api_key) = tmdb_api_key else {
        return Ok(None);
    };
    if api_key.trim().is_empty() {
        return Ok(None);
    }
    let base = format!("https://api.themoviedb.org/3/search/{media_type}");
    for variant in title_variants(title) {
        let mut url = Url::parse(&base).map_err(|err| err.to_string())?;
        url.query_pairs_mut()
            .append_pair("api_key", api_key)
            .append_pair("query", &variant)
            .append_pair("include_adult", "false");
        let resp = Client::new()
            .get(url)
            .send()
            .await
            .map_err(|err| err.to_string())?;
        if !resp.status().is_success() {
            continue;
        }
        let payload: TmdbSearchResponse = resp.json().await.map_err(|err| err.to_string())?;
        if let Some(result) = payload
            .results
            .into_iter()
            .find(|result| result.poster_path.is_some())
        {
            let poster_path = result.poster_path.unwrap();
            let cover_url = format!("https://image.tmdb.org/t/p/w500{poster_path}");
            return Ok(Some((cover_url, "tmdb".to_string())));
        }
    }
    Ok(None)
}

async fn fetch_open_library_cover(title: &str) -> Result<Option<(String, String)>, String> {
    for variant in title_variants(title) {
        let mut url =
            Url::parse("https://openlibrary.org/search.json").map_err(|err| err.to_string())?;
        url.query_pairs_mut()
            .append_pair("title", &variant)
            .append_pair("limit", "1");
        let resp = Client::new()
            .get(url)
            .send()
            .await
            .map_err(|err| err.to_string())?;
        if !resp.status().is_success() {
            continue;
        }
        let payload: OpenLibrarySearchResponse =
            resp.json().await.map_err(|err| err.to_string())?;
        if let Some(doc) = payload.docs.into_iter().find(|doc| doc.cover_i.is_some()) {
            let cover_id = doc.cover_i.unwrap();
            let cover_url = format!("https://covers.openlibrary.org/b/id/{cover_id}-M.jpg");
            return Ok(Some((cover_url, "openlibrary".to_string())));
        }
    }
    Ok(None)
}

async fn fetch_igdb_token(client_id: &str, client_secret: &str) -> Result<String, String> {
    let url = format!(
        "https://id.twitch.tv/oauth2/token?client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials"
    );
    let resp = Client::new()
        .post(url)
        .send()
        .await
        .map_err(|err| err.to_string())?;
    if !resp.status().is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("IGDB auth failed: {text}"));
    }
    let payload: IgdbTokenResponse = resp.json().await.map_err(|err| err.to_string())?;
    Ok(payload.access_token)
}

async fn fetch_igdb_cover(
    title: &str,
    client_id: Option<&str>,
    access_token: &str,
) -> Result<Option<(String, String)>, String> {
    let Some(client_id) = client_id else {
        return Ok(None);
    };
    if client_id.trim().is_empty() || access_token.trim().is_empty() {
        return Ok(None);
    }
    for variant in game_title_variants(title) {
        let body = format!(
            "search \"{}\"; fields cover.image_id; limit 1;",
            variant.replace('\"', "")
        );
        let resp = Client::new()
            .post("https://api.igdb.com/v4/games")
            .header("Client-ID", client_id)
            .header("Authorization", format!("Bearer {access_token}"))
            .body(body)
            .send()
            .await
            .map_err(|err| err.to_string())?;
        if !resp.status().is_success() {
            continue;
        }
        let results: Vec<IgdbGameResult> = resp.json().await.map_err(|err| err.to_string())?;
        if let Some(result) = results.into_iter().find(|item| item.cover.is_some()) {
            let cover = result.cover.unwrap();
            let cover_url = format!(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/{}.jpg",
                cover.image_id
            );
            return Ok(Some((cover_url, "igdb".to_string())));
        }
    }
    Ok(None)
}

fn fetch_youtube_cover(youtube_id: Option<&str>, url: Option<&str>) -> Option<(String, String)> {
    let id = youtube_id
        .and_then(|value| {
            if value.trim().is_empty() {
                None
            } else {
                Some(value)
            }
        })
        .or_else(|| url.and_then(extract_youtube_id));
    id.map(|video_id| {
        (
            format!("https://img.youtube.com/vi/{video_id}/hqdefault.jpg"),
            "youtube".to_string(),
        )
    })
}

fn extract_youtube_id(url: &str) -> Option<&str> {
    if let Some(index) = url.find("v=") {
        let id = &url[(index + 2)..];
        let end = id.find('&').unwrap_or(id.len());
        return Some(&id[..end]);
    }
    if let Some(index) = url.find("youtu.be/") {
        let id = &url[(index + 9)..];
        let end = id.find('?').unwrap_or(id.len());
        return Some(&id[..end]);
    }
    None
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
