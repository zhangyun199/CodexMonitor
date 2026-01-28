use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

use chrono::{Duration, NaiveDate, Utc};
use serde::Deserialize;

use crate::types::{DomainTrendSnapshot, TrendCard, TrendList, TrendListItem};

#[derive(Clone)]
struct StreamEntry {
    date: NaiveDate,
    text: String,
    links: Vec<String>,
}

#[derive(Default, Clone)]
struct Nutrition {
    calories: f64,
    protein: f64,
    carbs: f64,
    fat: f64,
    fiber: f64,
}

#[derive(Clone)]
struct DeliverySession {
    date: NaiveDate,
    earnings: f64,
    hours: f64,
    mileage: f64,
    orders: f64,
}

#[derive(Clone)]
struct Bill {
    name: String,
    amount: f64,
    next_due: Option<NaiveDate>,
}

#[derive(Clone)]
struct MediaItem {
    title: String,
    status: Option<String>,
    rating: Option<f64>,
    completed_at: Option<NaiveDate>,
}

#[derive(Clone)]
struct YoutubeIdea {
    title: String,
    tier: Option<String>,
    stage: Option<String>,
    created_at: Option<NaiveDate>,
    updated_at: Option<NaiveDate>,
}

struct TrendCacheEntry {
    last_mtime: SystemTime,
    snapshot: DomainTrendSnapshot,
}

static TREND_CACHE: OnceLock<Mutex<HashMap<String, TrendCacheEntry>>> = OnceLock::new();

pub(crate) fn compute_domain_trends(
    workspace_path: &str,
    domain_id: &str,
    range: &str,
) -> Result<DomainTrendSnapshot, String> {
    let workspace_root = PathBuf::from(workspace_path);
    let normalized_domain = normalize_domain_id(domain_id);
    let cache_key = format!("{}::{}::{}", workspace_path, normalized_domain, range);
    let latest_mtime = latest_mtime_for_domain(&workspace_root, normalized_domain.as_str())?;

    let cache = TREND_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(entry) = cache.lock().unwrap().get(&cache_key) {
        if entry.last_mtime >= latest_mtime {
            return Ok(entry.snapshot.clone());
        }
    }

    let today = Utc::now().date_naive();
    let start_date = match range {
        "7d" => Some(today - Duration::days(6)),
        "30d" => Some(today - Duration::days(29)),
        _ => None,
    };

    let stream_entries = load_stream_entries(&workspace_root);
    let snapshot = match normalized_domain.as_str() {
        "delivery_finance" => build_delivery_snapshot(
            normalized_domain.as_str(),
            range,
            today,
            start_date,
            &workspace_root,
        ),
        "food_exercise" => build_food_snapshot(
            normalized_domain.as_str(),
            range,
            today,
            start_date,
            &workspace_root,
            &stream_entries,
        ),
        "media" => build_media_snapshot(
            normalized_domain.as_str(),
            range,
            today,
            start_date,
            &workspace_root,
        ),
        "youtube" => build_youtube_snapshot(
            normalized_domain.as_str(),
            range,
            today,
            start_date,
            &workspace_root,
        ),
        _ => DomainTrendSnapshot {
            domain_id: normalized_domain,
            range: range.to_string(),
            updated_at: Utc::now().to_rfc3339(),
            cards: Vec::new(),
            lists: Vec::new(),
            series: None,
        },
    };

    cache.lock().unwrap().insert(
        cache_key,
        TrendCacheEntry {
            last_mtime: latest_mtime,
            snapshot: snapshot.clone(),
        },
    );

    Ok(snapshot)
}

fn normalize_domain_id(domain_id: &str) -> String {
    domain_id
        .trim()
        .to_lowercase()
        .replace('-', "_")
        .replace(' ', "_")
}

fn latest_mtime_for_domain(root: &Path, domain: &str) -> Result<SystemTime, String> {
    let mut max_time = SystemTime::UNIX_EPOCH;
    let stream_dir = root.join("Stream");
    max_time = max_time.max(latest_mtime_in_dir(&stream_dir)?);
    let entities = root.join("Entities");
    let domain_dir = match domain {
        "delivery_finance" => vec![
            entities.join("Delivery").join("Sessions"),
            entities.join("Finance").join("Bills"),
        ],
        "food_exercise" => vec![entities.join("Food"), entities.join("Behaviors")],
        "media" => vec![entities.join("Media")],
        "youtube" => vec![entities.join("YouTube")],
        _ => vec![entities],
    };
    for dir in domain_dir {
        max_time = max_time.max(latest_mtime_in_dir(&dir)?);
    }
    Ok(max_time)
}

fn latest_mtime_in_dir(path: &Path) -> Result<SystemTime, String> {
    let mut max_time = SystemTime::UNIX_EPOCH;
    if !path.exists() {
        return Ok(max_time);
    }
    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        if metadata.is_dir() {
            max_time = max_time.max(latest_mtime_in_dir(&entry.path())?);
        } else if let Ok(modified) = metadata.modified() {
            max_time = max_time.max(modified);
        }
    }
    Ok(max_time)
}

fn build_delivery_snapshot(
    domain_id: &str,
    range: &str,
    today: NaiveDate,
    start_date: Option<NaiveDate>,
    root: &Path,
) -> DomainTrendSnapshot {
    let sessions = load_delivery_sessions(root);
    let mut total_earnings = 0.0;
    let mut total_hours = 0.0;
    let mut total_miles = 0.0;
    let mut total_orders = 0.0;
    let mut session_items = Vec::new();
    let mut sessions_count = 0;

    for session in sessions {
        if in_range(session.date, start_date, today) {
            total_earnings += session.earnings;
            total_hours += session.hours;
            total_miles += session.mileage;
            total_orders += session.orders;
            sessions_count += 1;
            session_items.push(TrendListItem {
                label: session.date.to_string(),
                value: format!("${:.2}", session.earnings),
                sub_label: Some(format!(
                    "{:.0} orders • {:.1} hrs",
                    session.orders, session.hours
                )),
            });
        }
    }

    let hourly = if total_hours > 0.0 {
        total_earnings / total_hours
    } else {
        0.0
    };
    let per_mile = if total_miles > 0.0 {
        total_earnings / total_miles
    } else {
        0.0
    };
    let avg_order = if total_orders > 0.0 {
        total_earnings / total_orders
    } else {
        0.0
    };

    let bills = load_bills(root);
    let bill_end = match range {
        "7d" => today + Duration::days(7),
        "30d" => today + Duration::days(30),
        _ => today + Duration::days(3650),
    };
    let mut bill_total = 0.0;
    let mut bill_entries: Vec<(NaiveDate, TrendListItem)> = Vec::new();
    for bill in bills {
        if let Some(next_due) = bill.next_due {
            if next_due >= today && next_due <= bill_end {
                bill_total += bill.amount;
                bill_entries.push((
                    next_due,
                    TrendListItem {
                        label: bill.name,
                        value: format!("${:.2}", bill.amount),
                        sub_label: Some(format!("Due {}", next_due)),
                    },
                ));
            }
        }
    }
    bill_entries.sort_by_key(|(due, _)| *due);
    let bill_items = bill_entries.into_iter().map(|(_, item)| item).collect();

    DomainTrendSnapshot {
        domain_id: domain_id.to_string(),
        range: range.to_string(),
        updated_at: Utc::now().to_rfc3339(),
        cards: vec![
            TrendCard {
                id: "earnings".to_string(),
                label: "Earnings".to_string(),
                value: format!("${:.2}", total_earnings),
                sub_label: None,
            },
            TrendCard {
                id: "hours".to_string(),
                label: "Hours".to_string(),
                value: format!("{:.1}", total_hours),
                sub_label: None,
            },
            TrendCard {
                id: "sessions".to_string(),
                label: "Sessions".to_string(),
                value: format!("{sessions_count}"),
                sub_label: None,
            },
            TrendCard {
                id: "hourly".to_string(),
                label: "$/hr".to_string(),
                value: format!("${:.2}", hourly),
                sub_label: None,
            },
            TrendCard {
                id: "per_mile".to_string(),
                label: "$/mi".to_string(),
                value: format!("${:.2}", per_mile),
                sub_label: None,
            },
            TrendCard {
                id: "orders".to_string(),
                label: "Orders".to_string(),
                value: format!("{:.0}", total_orders),
                sub_label: None,
            },
            TrendCard {
                id: "avg_order".to_string(),
                label: "Avg/Order".to_string(),
                value: format!("${:.2}", avg_order),
                sub_label: None,
            },
            TrendCard {
                id: "bills_due".to_string(),
                label: "Bills Due".to_string(),
                value: format!("${:.2}", bill_total),
                sub_label: None,
            },
        ],
        lists: vec![
            TrendList {
                id: "sessions".to_string(),
                title: "Sessions".to_string(),
                items: session_items,
            },
            TrendList {
                id: "bills".to_string(),
                title: "Upcoming Bills".to_string(),
                items: bill_items,
            },
        ],
        series: None,
    }
}

fn build_food_snapshot(
    domain_id: &str,
    range: &str,
    today: NaiveDate,
    start_date: Option<NaiveDate>,
    root: &Path,
    stream_entries: &[StreamEntry],
) -> DomainTrendSnapshot {
    let food_map = load_food_map(root);
    let mut total = Nutrition::default();
    let mut meals_count = 0;
    let mut workout_count = 0;
    let mut food_counts: HashMap<String, usize> = HashMap::new();
    let mut entry_dates: HashSet<NaiveDate> = HashSet::new();

    for entry in stream_entries {
        if !in_range(entry.date, start_date, today) {
            continue;
        }
        entry_dates.insert(entry.date);
        let mut matched_food = false;
        for link in &entry.links {
            if let Some(name) = food_link_name(link) {
                if let Some(nutrition) = food_map.get(&name) {
                    total.calories += nutrition.calories;
                    total.protein += nutrition.protein;
                    total.carbs += nutrition.carbs;
                    total.fat += nutrition.fat;
                    total.fiber += nutrition.fiber;
                    *food_counts.entry(name.clone()).or_default() += 1;
                    matched_food = true;
                }
            }
        }
        if matched_food {
            meals_count += 1;
        }
        if entry.text.contains("🏋️") || entry.text.to_lowercase().contains("workout") {
            workout_count += 1;
        }
        if entry.text.contains("🚶") || entry.text.to_lowercase().contains("walk") {
            workout_count += 1;
        }
    }

    let mut top_foods: Vec<_> = food_counts.into_iter().collect();
    top_foods.sort_by(|a, b| b.1.cmp(&a.1));
    let top_food_items = top_foods
        .into_iter()
        .take(5)
        .map(|(name, count)| TrendListItem {
            label: name,
            value: format!("{count}"),
            sub_label: None,
        })
        .collect();

    let range_days = if let Some(start) = start_date {
        (today - start).num_days().max(0) + 1
    } else {
        entry_dates.len().max(1) as i64
    };
    let avg_calories = if range_days > 0 {
        total.calories / range_days as f64
    } else {
        0.0
    };
    let avg_protein = if range_days > 0 {
        total.protein / range_days as f64
    } else {
        0.0
    };

    DomainTrendSnapshot {
        domain_id: domain_id.to_string(),
        range: range.to_string(),
        updated_at: Utc::now().to_rfc3339(),
        cards: vec![
            TrendCard {
                id: "calories".to_string(),
                label: "Calories".to_string(),
                value: format!("{:.0}", total.calories),
                sub_label: None,
            },
            TrendCard {
                id: "calories_avg".to_string(),
                label: "Calories/Day".to_string(),
                value: format!("{:.0}", avg_calories),
                sub_label: None,
            },
            TrendCard {
                id: "protein".to_string(),
                label: "Protein (g)".to_string(),
                value: format!("{:.0}", total.protein),
                sub_label: None,
            },
            TrendCard {
                id: "protein_avg".to_string(),
                label: "Protein/Day".to_string(),
                value: format!("{:.0}g", avg_protein),
                sub_label: None,
            },
            TrendCard {
                id: "meals".to_string(),
                label: "Meals".to_string(),
                value: format!("{meals_count}"),
                sub_label: None,
            },
            TrendCard {
                id: "workouts".to_string(),
                label: "Workouts".to_string(),
                value: format!("{workout_count}"),
                sub_label: None,
            },
        ],
        lists: vec![
            TrendList {
                id: "macros".to_string(),
                title: "Macro Totals".to_string(),
                items: vec![
                    TrendListItem {
                        label: "Carbs".to_string(),
                        value: format!("{:.0}g", total.carbs),
                        sub_label: None,
                    },
                    TrendListItem {
                        label: "Fat".to_string(),
                        value: format!("{:.0}g", total.fat),
                        sub_label: None,
                    },
                    TrendListItem {
                        label: "Fiber".to_string(),
                        value: format!("{:.0}g", total.fiber),
                        sub_label: None,
                    },
                ],
            },
            TrendList {
                id: "top_foods".to_string(),
                title: "Top Foods".to_string(),
                items: top_food_items,
            },
        ],
        series: None,
    }
}

fn build_media_snapshot(
    domain_id: &str,
    range: &str,
    today: NaiveDate,
    start_date: Option<NaiveDate>,
    root: &Path,
) -> DomainTrendSnapshot {
    let items = load_media_items(root);
    let mut completed = 0;
    let mut rating_sum = 0.0;
    let mut rating_count = 0;
    let mut recent_items = Vec::new();
    let mut top_rated_items = Vec::new();
    let mut backlog = 0;

    for item in items {
        if matches!(item.status.as_deref(), Some("Backlog")) {
            backlog += 1;
        }
        if let Some(completed_at) = item.completed_at {
            if in_range(completed_at, start_date, today) {
                completed += 1;
                if let Some(rating) = item.rating {
                    rating_sum += rating;
                    rating_count += 1;
                    top_rated_items.push((rating, item.title.clone(), completed_at));
                }
                recent_items.push(TrendListItem {
                    label: item.title,
                    value: item
                        .rating
                        .map(|r| format!("{:.0}/10", r))
                        .unwrap_or_else(|| "-".to_string()),
                    sub_label: Some(completed_at.to_string()),
                });
            }
        }
    }

    let avg_rating = if rating_count > 0 {
        rating_sum / rating_count as f64
    } else {
        0.0
    };

    top_rated_items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    let top_rated_list = top_rated_items
        .into_iter()
        .take(5)
        .map(|(rating, title, completed_at)| TrendListItem {
            label: title,
            value: format!("{rating:.0}/10"),
            sub_label: Some(completed_at.to_string()),
        })
        .collect::<Vec<_>>();

    DomainTrendSnapshot {
        domain_id: domain_id.to_string(),
        range: range.to_string(),
        updated_at: Utc::now().to_rfc3339(),
        cards: vec![
            TrendCard {
                id: "completed".to_string(),
                label: "Completed".to_string(),
                value: format!("{completed}"),
                sub_label: None,
            },
            TrendCard {
                id: "avg_rating".to_string(),
                label: "Avg Rating".to_string(),
                value: format!("{:.1}", avg_rating),
                sub_label: None,
            },
            TrendCard {
                id: "rated".to_string(),
                label: "Rated".to_string(),
                value: format!("{rating_count}"),
                sub_label: None,
            },
            TrendCard {
                id: "backlog".to_string(),
                label: "Backlog".to_string(),
                value: format!("{backlog}"),
                sub_label: None,
            },
        ],
        lists: vec![
            TrendList {
                id: "recent_media".to_string(),
                title: "Recent Completions".to_string(),
                items: recent_items,
            },
            TrendList {
                id: "top_rated".to_string(),
                title: "Top Rated".to_string(),
                items: top_rated_list,
            },
        ],
        series: None,
    }
}

fn build_youtube_snapshot(
    domain_id: &str,
    range: &str,
    today: NaiveDate,
    start_date: Option<NaiveDate>,
    root: &Path,
) -> DomainTrendSnapshot {
    let ideas = load_youtube_items(root);
    let mut created_count = 0;
    let mut stage_counts: HashMap<String, usize> = HashMap::new();
    let mut tier_counts: HashMap<String, usize> = HashMap::new();
    let mut total = 0;
    let mut ready_count = 0;
    let mut published_count = 0;
    let mut newest_items: Vec<(NaiveDate, String, Option<String>)> = Vec::new();

    for idea in ideas {
        total += 1;
        if let Some(stage) = idea.stage.clone() {
            let normalized = stage.to_lowercase();
            *stage_counts.entry(stage).or_default() += 1;
            if normalized.contains("ready") {
                ready_count += 1;
            }
            if normalized.contains("published") {
                published_count += 1;
            }
        }
        if let Some(tier) = idea.tier.clone() {
            *tier_counts.entry(tier).or_default() += 1;
        }
        if let Some(created) = idea.created_at {
            newest_items.push((created, idea.title.clone(), idea.stage.clone()));
            if in_range(created, start_date, today) {
                created_count += 1;
            }
        }
    }

    let mut stage_items: Vec<_> = stage_counts.into_iter().collect();
    stage_items.sort_by(|a, b| b.1.cmp(&a.1));
    let stage_list = stage_items
        .into_iter()
        .map(|(stage, count)| TrendListItem {
            label: stage,
            value: format!("{count}"),
            sub_label: None,
        })
        .collect();

    let mut tier_items: Vec<_> = tier_counts.into_iter().collect();
    tier_items.sort_by(|a, b| b.1.cmp(&a.1));
    let tier_list = tier_items
        .into_iter()
        .map(|(tier, count)| TrendListItem {
            label: tier,
            value: format!("{count}"),
            sub_label: None,
        })
        .collect();

    newest_items.sort_by(|a, b| b.0.cmp(&a.0));
    let newest_list = newest_items
        .into_iter()
        .take(5)
        .map(|(created, title, stage)| TrendListItem {
            label: title,
            value: stage.unwrap_or_else(|| "-".to_string()),
            sub_label: Some(created.to_string()),
        })
        .collect();

    DomainTrendSnapshot {
        domain_id: domain_id.to_string(),
        range: range.to_string(),
        updated_at: Utc::now().to_rfc3339(),
        cards: vec![
            TrendCard {
                id: "created".to_string(),
                label: "Ideas Created".to_string(),
                value: format!("{created_count}"),
                sub_label: None,
            },
            TrendCard {
                id: "total".to_string(),
                label: "Total Ideas".to_string(),
                value: format!("{total}"),
                sub_label: None,
            },
            TrendCard {
                id: "ready".to_string(),
                label: "Ready".to_string(),
                value: format!("{ready_count}"),
                sub_label: None,
            },
            TrendCard {
                id: "published".to_string(),
                label: "Published".to_string(),
                value: format!("{published_count}"),
                sub_label: None,
            },
        ],
        lists: vec![
            TrendList {
                id: "stages".to_string(),
                title: "Stages".to_string(),
                items: stage_list,
            },
            TrendList {
                id: "tiers".to_string(),
                title: "Tiers".to_string(),
                items: tier_list,
            },
            TrendList {
                id: "newest".to_string(),
                title: "Newest Ideas".to_string(),
                items: newest_list,
            },
        ],
        series: None,
    }
}

fn load_stream_entries(root: &Path) -> Vec<StreamEntry> {
    let stream_dir = root.join("Stream");
    if !stream_dir.exists() {
        return Vec::new();
    }
    let mut entries = Vec::new();
    let dir = match fs::read_dir(&stream_dir) {
        Ok(dir) => dir,
        Err(_) => return entries,
    };
    for entry in dir.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path) {
            let year = parse_year_from_filename(&path);
            entries.extend(parse_stream_file(&content, year));
        }
    }
    entries
}

fn parse_stream_file(content: &str, year: Option<i32>) -> Vec<StreamEntry> {
    let mut entries = Vec::new();
    let mut current_date: Option<NaiveDate> = None;
    for line in content.lines() {
        if let Some(date) = parse_header_date(line, year) {
            current_date = Some(date);
            continue;
        }
        let Some(date) = current_date else {
            continue;
        };
        if let Some(text) = extract_entry_text(line) {
            let links = extract_links(&text);
            entries.push(StreamEntry { date, text, links });
        }
    }
    entries
}

fn parse_year_from_filename(path: &Path) -> Option<i32> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.split('-').next())
        .and_then(|year| year.parse::<i32>().ok())
}

fn parse_header_date(line: &str, year: Option<i32>) -> Option<NaiveDate> {
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
    let month = month_number(parts[1])?;
    let day: u32 = parts[2].parse().ok()?;
    let year = year?;
    NaiveDate::from_ymd_opt(year, month, day)
}

fn extract_entry_text(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with("|") {
        return extract_table_entry(trimmed);
    }
    if trimmed.starts_with("**") && trimmed.contains("|") {
        let segments: Vec<&str> = trimmed.split('|').collect();
        if segments.len() >= 2 {
            let rest = segments[1].trim();
            return Some(rest.to_string());
        }
    }
    if trimmed.starts_with("**") && trimmed.contains("**") {
        // Timeline format: **5:30pm** | ...
        if let Some(after) = trimmed.split("**").nth(2) {
            if after.contains('|') {
                let rest = after.split('|').nth(1).unwrap_or("").trim();
                if !rest.is_empty() {
                    return Some(rest.to_string());
                }
            }
        }
    }
    None
}

fn extract_table_entry(line: &str) -> Option<String> {
    let cells: Vec<&str> = line.split('|').collect();
    for cell in cells.iter().skip(1) {
        let cell = cell.trim();
        if cell.is_empty() || cell == "—" {
            continue;
        }
        if has_time_prefix(cell) {
            let stripped = strip_time_prefix(cell);
            if let Some(rest) = stripped {
                return Some(rest.trim().to_string());
            }
        }
    }
    None
}

fn has_time_prefix(value: &str) -> bool {
    strip_time_prefix(value).is_some()
}

fn strip_time_prefix(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let mut chars = trimmed.chars().peekable();
    let mut digits = String::new();
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            digits.push(*c);
            chars.next();
        } else {
            break;
        }
    }
    if digits.is_empty() {
        return None;
    }
    if chars.next()? != ':' {
        return None;
    }
    let mut minutes = String::new();
    for _ in 0..2 {
        if let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                minutes.push(c);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    let mut suffix = String::new();
    while let Some(c) = chars.peek() {
        if c.is_ascii_alphabetic() {
            suffix.push(c.to_ascii_lowercase());
            chars.next();
        } else {
            break;
        }
    }
    if suffix != "am" && suffix != "pm" && !suffix.is_empty() {
        return None;
    }
    let rest: String = chars.collect();
    Some(rest)
}

fn extract_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut remaining = text;
    while let Some(start) = remaining.find("[[") {
        let after = &remaining[start + 2..];
        if let Some(end) = after.find("]]") {
            let link = &after[..end];
            links.push(link.trim().to_string());
            remaining = &after[end + 2..];
        } else {
            break;
        }
    }
    links
}

fn load_food_map(root: &Path) -> HashMap<String, Nutrition> {
    let mut map = HashMap::new();
    let dir = root.join("Entities").join("Food");
    if !dir.exists() {
        return map;
    }
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return map,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path) {
            let (frontmatter, body) = split_frontmatter(&content);
            let mut nutrition = Nutrition::default();
            let mut name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();

            if let Some(frontmatter) = frontmatter {
                if let Ok(parsed) = serde_yaml::from_str::<FoodFrontmatter>(&frontmatter) {
                    if let Some(value) = parsed.calories {
                        nutrition.calories = value;
                    }
                    if let Some(value) = parsed.protein {
                        nutrition.protein = value;
                    }
                    if let Some(value) = parsed.carbs {
                        nutrition.carbs = value;
                    }
                    if let Some(value) = parsed.fat {
                        nutrition.fat = value;
                    }
                    if let Some(value) = parsed.fiber {
                        nutrition.fiber = value;
                    }
                    if let Some(value) = parsed.name {
                        name = value;
                    }
                }
            } else {
                nutrition = nutrition_from_table(&body);
            }

            if nutrition.calories > 0.0 || nutrition.protein > 0.0 {
                map.insert(name.clone(), nutrition.clone());
                map.insert(format!("Food/{}", name), nutrition);
            }
        }
    }
    map
}

fn nutrition_from_table(body: &str) -> Nutrition {
    let mut nutrition = Nutrition::default();
    for line in body.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = trimmed.split('|').collect();
        if cells.len() < 3 {
            continue;
        }
        let metric = cells[1].trim().to_lowercase();
        let value = parse_number(cells[2]);
        if metric.contains("calorie") {
            nutrition.calories = value;
        } else if metric.contains("protein") {
            nutrition.protein = value;
        } else if metric.contains("carb") {
            nutrition.carbs = value;
        } else if metric.contains("fat") {
            nutrition.fat = value;
        } else if metric.contains("fiber") {
            nutrition.fiber = value;
        }
    }
    nutrition
}

fn load_delivery_sessions(root: &Path) -> Vec<DeliverySession> {
    let mut sessions = Vec::new();
    let dir = root.join("Entities").join("Delivery").join("Sessions");
    if !dir.exists() {
        return sessions;
    }
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return sessions,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path) {
            let (frontmatter, _) = split_frontmatter(&content);
            if let Some(frontmatter) = frontmatter {
                if let Ok(parsed) = serde_yaml::from_str::<DeliverySessionFrontmatter>(&frontmatter)
                {
                    if let Some(date) = parse_date(&parsed.date) {
                        sessions.push(DeliverySession {
                            date,
                            earnings: parsed.earnings.unwrap_or(0.0),
                            hours: parsed.hours.unwrap_or(0.0),
                            mileage: parsed.mileage.unwrap_or(0.0),
                            orders: parsed.orders_count.unwrap_or(0.0),
                        });
                    }
                }
            }
        }
    }
    sessions
}

fn load_bills(root: &Path) -> Vec<Bill> {
    let mut bills = Vec::new();
    let dir = root.join("Entities").join("Finance").join("Bills");
    if !dir.exists() {
        return bills;
    }
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return bills,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path) {
            let (frontmatter, _) = split_frontmatter(&content);
            if let Some(frontmatter) = frontmatter {
                if let Ok(parsed) = serde_yaml::from_str::<BillFrontmatter>(&frontmatter) {
                    bills.push(Bill {
                        name: parsed.name.unwrap_or_else(|| {
                            path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("")
                                .to_string()
                        }),
                        amount: parsed.amount.unwrap_or(0.0),
                        next_due: parsed.next_due.and_then(|d| parse_date(&d)),
                    });
                }
            }
        }
    }
    bills
}

fn load_media_items(root: &Path) -> Vec<MediaItem> {
    let mut items = Vec::new();
    let dir = root.join("Entities").join("Media");
    if !dir.exists() {
        return items;
    }
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return items,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path) {
            let (frontmatter, _) = split_frontmatter(&content);
            if let Some(frontmatter) = frontmatter {
                if let Ok(parsed) = serde_yaml::from_str::<MediaFrontmatter>(&frontmatter) {
                    items.push(MediaItem {
                        title: parsed.title.unwrap_or_else(|| {
                            path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("")
                                .to_string()
                        }),
                        status: parsed.status,
                        rating: parsed.rating,
                        completed_at: parsed.completed_at.and_then(|d| parse_date(&d)),
                    });
                }
            }
        }
    }
    items
}

fn load_youtube_items(root: &Path) -> Vec<YoutubeIdea> {
    let mut items = Vec::new();
    let dir = root.join("Entities").join("YouTube");
    if !dir.exists() {
        return items;
    }
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(_) => return items,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path) {
            let (frontmatter, _) = split_frontmatter(&content);
            if let Some(frontmatter) = frontmatter {
                if let Ok(parsed) = serde_yaml::from_str::<YoutubeFrontmatter>(&frontmatter) {
                    items.push(YoutubeIdea {
                        title: parsed.title.unwrap_or_else(|| {
                            path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("")
                                .to_string()
                        }),
                        tier: parsed.tier,
                        stage: parsed.stage,
                        created_at: parsed.created_at.and_then(|d| parse_date(&d)),
                        updated_at: parsed.updated_at.and_then(|d| parse_date(&d)),
                    });
                }
            }
        }
    }
    items
}

fn split_frontmatter(content: &str) -> (Option<String>, String) {
    let mut lines = content.lines();
    let mut frontmatter = Vec::new();
    let mut in_frontmatter = false;

    if let Some(line) = lines.next() {
        if line.trim() == "---" {
            in_frontmatter = true;
        } else {
            return (None, content.to_string());
        }
    }

    for line in lines.by_ref() {
        if line.trim() == "---" && in_frontmatter {
            in_frontmatter = false;
            break;
        }
        if in_frontmatter {
            frontmatter.push(line);
        }
    }

    let body: String = lines.collect::<Vec<&str>>().join("\n");
    let frontmatter = if frontmatter.is_empty() {
        None
    } else {
        Some(frontmatter.join("\n"))
    };
    (frontmatter, body)
}

fn parse_date(value: &str) -> Option<NaiveDate> {
    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return Some(date);
    }
    if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(value) {
        return Some(datetime.date_naive());
    }
    None
}

fn parse_number(value: &str) -> f64 {
    let cleaned: String = value
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    cleaned.parse::<f64>().unwrap_or(0.0)
}

fn food_link_name(link: &str) -> Option<String> {
    if link.starts_with("Food/") {
        return Some(link.trim_start_matches("Food/").to_string());
    }
    Some(link.to_string())
}

fn in_range(date: NaiveDate, start: Option<NaiveDate>, end: NaiveDate) -> bool {
    if let Some(start) = start {
        date >= start && date <= end
    } else {
        date <= end
    }
}

fn month_number(month: &str) -> Option<u32> {
    match month.to_lowercase().as_str() {
        "jan" => Some(1),
        "feb" => Some(2),
        "mar" => Some(3),
        "apr" => Some(4),
        "may" => Some(5),
        "jun" => Some(6),
        "jul" => Some(7),
        "aug" => Some(8),
        "sep" | "sept" => Some(9),
        "oct" => Some(10),
        "nov" => Some(11),
        "dec" => Some(12),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
struct FoodFrontmatter {
    name: Option<String>,
    calories: Option<f64>,
    protein: Option<f64>,
    carbs: Option<f64>,
    fat: Option<f64>,
    fiber: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct DeliverySessionFrontmatter {
    date: String,
    earnings: Option<f64>,
    hours: Option<f64>,
    mileage: Option<f64>,
    #[serde(rename = "orders_count")]
    orders_count: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct BillFrontmatter {
    name: Option<String>,
    amount: Option<f64>,
    next_due: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MediaFrontmatter {
    title: Option<String>,
    status: Option<String>,
    rating: Option<f64>,
    completed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YoutubeFrontmatter {
    title: Option<String>,
    tier: Option<String>,
    stage: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}
