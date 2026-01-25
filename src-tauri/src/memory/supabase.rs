//! Supabase client for memory operations
//! Reference: /Volumes/YouTube 4TB/code/_archive/life-mcp/src/supabase/client.js

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone)]
pub struct SupabaseClient {
    client: Client,
    url: String,
    anon_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Option<String>,
    pub content: String,
    pub memory_type: String,
    pub tags: Vec<String>,
    pub workspace_id: Option<String>,
    pub embedding_status: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchResult {
    pub id: String,
    pub content: String,
    pub memory_type: String,
    pub tags: Vec<String>,
    pub workspace_id: Option<String>,
    pub created_at: String,
    pub distance: Option<f64>,
    pub score: Option<f64>,
    pub rank: Option<f32>,
}

impl SupabaseClient {
    pub fn new(url: &str, anon_key: &str) -> Self {
        Self {
            client: Client::new(),
            url: url.trim_end_matches('/').to_string(),
            anon_key: anon_key.to_string(),
        }
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.anon_key.parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.anon_key).parse().unwrap(),
        );
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers
    }

    /// Insert a new memory entry
    pub async fn insert_memory(&self, entry: &MemoryEntry) -> Result<MemoryEntry, String> {
        let url = format!("{}/rest/v1/memory", self.url);

        let body = json!({
            "content": entry.content,
            "memory_type": entry.memory_type,
            "tags": entry.tags,
            "workspace_id": entry.workspace_id,
            "embedding_status": "pending"
        });

        let resp = self
            .client
            .post(&url)
            .headers(self.headers())
            .header("Prefer", "return=representation")
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Supabase insert failed: {}", text));
        }

        let entries: Vec<MemoryEntry> = resp.json().await.map_err(|e| e.to_string())?;
        entries.into_iter().next().ok_or("No entry returned".to_string())
    }

    /// Update memory with embedding
    pub async fn update_memory_embedding(
        &self,
        id: &str,
        embedding: &[f32],
        model: &str,
        dim: usize,
    ) -> Result<(), String> {
        let url = format!("{}/rest/v1/memory?id=eq.{}", self.url, id);

        // Format embedding as pgvector literal
        let embedding_str = format!(
            "[{}]",
            embedding
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let body = json!({
            "embedding": embedding_str,
            "embedding_model": model,
            "embedding_dim": dim,
            "embedding_status": "ready",
            "embedding_updated_at": chrono::Utc::now().to_rfc3339()
        });

        let resp = self
            .client
            .patch(&url)
            .headers(self.headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Supabase update failed: {}", text));
        }

        Ok(())
    }

    /// Search memory by embedding (semantic search)
    pub async fn search_by_embedding(
        &self,
        embedding: &[f32],
        limit: usize,
        max_distance: Option<f64>,
    ) -> Result<Vec<MemorySearchResult>, String> {
        let url = format!("{}/rest/v1/rpc/search_memory_by_embedding", self.url);

        let embedding_str = format!(
            "[{}]",
            embedding
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let body = json!({
            "query_embedding": embedding_str,
            "match_count": limit,
            "max_distance": max_distance
        });

        let resp = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Supabase search failed: {}", text));
        }

        resp.json().await.map_err(|e| e.to_string())
    }

    /// Search memory by text (BM25 fallback)
    pub async fn search_by_text(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<MemorySearchResult>, String> {
        let url = format!("{}/rest/v1/rpc/search_memory_by_text", self.url);

        let body = json!({
            "search_query": query,
            "match_count": limit
        });

        let resp = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Supabase text search failed: {}", text));
        }

        resp.json().await.map_err(|e| e.to_string())
    }

    /// Get memory bootstrap (recent curated + daily)
    pub async fn get_bootstrap(&self) -> Result<Vec<MemorySearchResult>, String> {
        let url = format!("{}/rest/v1/rpc/get_memory_bootstrap", self.url);

        let resp = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&json!({}))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Supabase bootstrap failed: {}", text));
        }

        resp.json().await.map_err(|e| e.to_string())
    }

    /// Get memory status (counts by status)
    pub async fn get_status(&self) -> Result<Value, String> {
        // Count total, pending, ready, error
        let url = format!(
            "{}/rest/v1/memory?select=embedding_status",
            self.url
        );

        let resp = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Supabase status failed: {}", text));
        }

        let entries: Vec<Value> = resp.json().await.map_err(|e| e.to_string())?;

        let mut pending = 0;
        let mut ready = 0;
        let mut error = 0;

        for entry in &entries {
            match entry.get("embedding_status").and_then(|v| v.as_str()) {
                Some("pending") => pending += 1,
                Some("ready") => ready += 1,
                Some("error") => error += 1,
                _ => {}
            }
        }

        Ok(json!({
            "total": entries.len(),
            "pending": pending,
            "ready": ready,
            "error": error,
            "enabled": true
        }))
    }
}
