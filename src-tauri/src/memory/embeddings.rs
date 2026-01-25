//! MiniMax embeddings client
//! Reference: /Volumes/YouTube 4TB/code/_archive/life-mcp/src/clients/minimax-embeddings.js

use reqwest::Client;
use serde::{Deserialize, Serialize};

const MINIMAX_API_URL: &str = "https://api.minimax.chat/v1/embeddings";
const DEFAULT_MODEL: &str = "embo-01";

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

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    vectors: Vec<Vec<f32>>,
    model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EmbeddingResult {
    pub vector: Vec<f32>,
    pub model: String,
    pub dim: usize,
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

        let response: EmbeddingResponse = resp.json().await.map_err(|e| e.to_string())?;

        let vector = response
            .vectors
            .into_iter()
            .next()
            .ok_or("No vector returned")?;

        let dim = vector.len();

        Ok(EmbeddingResult {
            vector,
            model: response.model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
            dim,
        })
    }
}
