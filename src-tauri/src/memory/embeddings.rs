//! MiniMax embeddings client
//! Reference: /Volumes/YouTube 4TB/code/_archive/life-mcp/src/clients/minimax-embeddings.js

use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

const MINIMAX_API_URL: &str = "https://api.minimax.io/v1/embeddings";
const DEFAULT_MODEL: &str = "embo-01";
const MINIMAX_MIN_INTERVAL_MS: u64 = 15_000;
const MINIMAX_RETRY_BASE_MS: u64 = 15_000;
const MINIMAX_RETRIES: u8 = 2;

#[derive(Clone)]
pub struct EmbeddingsClient {
    client: Client,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    texts: Vec<String>,
    #[serde(rename = "type")]
    embed_type: String,
}

#[derive(Debug, Clone)]
pub struct EmbeddingResult {
    pub vector: Vec<f32>,
    pub model: String,
    pub dim: usize,
}

fn extract_vector(payload: &Value) -> Option<Vec<f32>> {
    let candidates = [
        payload.get("vectors").and_then(|v| v.get(0)),
        payload.get("embeddings").and_then(|v| v.get(0)),
        payload
            .get("data")
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("embedding")),
        payload
            .get("data")
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("vector")),
        payload.get("embedding"),
        payload.get("vector"),
    ];

    for candidate in candidates {
        if let Some(Value::Array(values)) = candidate {
            let mut vector = Vec::with_capacity(values.len());
            let mut valid = true;
            for item in values {
                if let Some(value) = item.as_f64() {
                    vector.push(value as f32);
                } else {
                    valid = false;
                    break;
                }
            }
            if valid && !vector.is_empty() {
                return Some(vector);
            }
        }
    }

    None
}

fn response_keys(payload: &Value) -> String {
    payload
        .as_object()
        .map(|map| {
            let mut keys = map.keys().cloned().collect::<Vec<_>>();
            keys.sort();
            keys.join(",")
        })
        .unwrap_or_default()
}

fn last_request_clock() -> &'static Mutex<Option<Instant>> {
    static LAST_REQUEST: OnceLock<Mutex<Option<Instant>>> = OnceLock::new();
    LAST_REQUEST.get_or_init(|| Mutex::new(None))
}

async fn enforce_min_interval() {
    let mut last = last_request_clock().lock().await;
    if let Some(prev) = *last {
        let elapsed = prev.elapsed();
        if elapsed < Duration::from_millis(MINIMAX_MIN_INTERVAL_MS) {
            sleep(Duration::from_millis(MINIMAX_MIN_INTERVAL_MS) - elapsed).await;
        }
    }
    *last = Some(Instant::now());
}

impl EmbeddingsClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn generate(
        &self,
        text: &str,
        embed_type: &str, // "db" or "query"
    ) -> Result<EmbeddingResult, String> {
        if self.api_key.is_empty() {
            return Err("MINIMAX_API_KEY not set".to_string());
        }

        // Truncate to ~8000 chars like the JS client
        let truncated = if text.len() > 8000 {
            &text[..8000]
        } else {
            text
        };

        let request = EmbeddingRequest {
            model: DEFAULT_MODEL.to_string(),
            texts: vec![truncated.to_string()],
            embed_type: embed_type.to_string(),
        };

        for attempt in 0..=MINIMAX_RETRIES {
            enforce_min_interval().await;

            let resp = self
                .client
                .post(MINIMAX_API_URL)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if !resp.status().is_success() {
                let text = resp.text().await.unwrap_or_default();
                return Err(format!("MiniMax API error: {}", text));
            }

            let body = resp.text().await.map_err(|e| e.to_string())?;
            let payload: Value = serde_json::from_str(&body)
                .map_err(|e| format!("MiniMax response parse error: {e}. Body: {body}"))?;

            if let Some(base) = payload.get("base_resp") {
                if let Some(code) = base.get("status_code").and_then(|v| v.as_i64()) {
                    if code == 1002 && attempt < MINIMAX_RETRIES {
                        let wait = MINIMAX_RETRY_BASE_MS.saturating_mul(2u64.pow(attempt as u32));
                        sleep(Duration::from_millis(wait)).await;
                        continue;
                    }
                    if code != 0 {
                        let msg = base
                            .get("status_msg")
                            .and_then(|v| v.as_str())
                            .unwrap_or("MiniMax API error");
                        return Err(format!("MiniMax API error {code}: {msg}"));
                    }
                }
            }

            let vector = extract_vector(&payload).ok_or_else(|| {
                format!(
                    "MiniMax response missing embedding vector (keys: {})",
                    response_keys(&payload)
                )
            })?;

            let dim = vector.len();
            let model = payload
                .get("model")
                .and_then(|v| v.as_str())
                .unwrap_or(DEFAULT_MODEL)
                .to_string();

            return Ok(EmbeddingResult { vector, model, dim });
        }

        Err("MiniMax embeddings request failed after retries".to_string())
    }
}
