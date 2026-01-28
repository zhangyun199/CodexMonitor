//! Memory service combining Supabase + MiniMax
//! Reference: /Volumes/YouTube 4TB/code/_archive/life-mcp/src/supabase/note-embeddings.js

use super::embeddings::EmbeddingsClient;
use super::supabase::{MemoryEntry, MemorySearchResult, SupabaseClient};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MemoryService {
    supabase: SupabaseClient,
    embeddings: Option<EmbeddingsClient>,
    enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatus {
    pub enabled: bool,
    pub embeddings_enabled: bool,
    pub total: usize,
    pub pending: usize,
    pub ready: usize,
    pub error: usize,
}

impl MemoryService {
    pub fn new(
        supabase_url: &str,
        supabase_anon_key: &str,
        minimax_api_key: Option<&str>,
        enabled: bool,
    ) -> Self {
        let embeddings = minimax_api_key
            .filter(|k| !k.is_empty())
            .map(EmbeddingsClient::new);

        Self {
            supabase: SupabaseClient::new(supabase_url, supabase_anon_key),
            embeddings,
            enabled,
        }
    }

    #[cfg(test)]
    pub fn with_clients(
        supabase: SupabaseClient,
        embeddings: Option<EmbeddingsClient>,
        enabled: bool,
    ) -> Self {
        Self {
            supabase,
            embeddings,
            enabled,
        }
    }

    pub async fn status(&self) -> Result<MemoryStatus, String> {
        if !self.enabled {
            return Ok(MemoryStatus {
                enabled: false,
                embeddings_enabled: false,
                total: 0,
                pending: 0,
                ready: 0,
                error: 0,
            });
        }

        let status = self.supabase.get_status().await?;

        Ok(MemoryStatus {
            enabled: true,
            embeddings_enabled: self.embeddings.is_some(),
            total: status.get("total").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
            pending: status.get("pending").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
            ready: status.get("ready").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
            error: status.get("error").and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        })
    }

    pub async fn search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<MemorySearchResult>, String> {
        if !self.enabled {
            return Err("Memory not enabled".to_string());
        }

        // Hybrid: embeddings + text (dedupe + merge). If embeddings fail, fall back to text.
        if let Some(ref embeddings) = self.embeddings {
            let embedding_result = embeddings.generate(query, "query").await;
            match embedding_result {
                Ok(result) => {
                    let (semantic, text) = tokio::join!(
                        self.supabase
                            .search_by_embedding(&result.vector, limit, Some(0.5)),
                        self.supabase.search_by_text(query, limit)
                    );
                    let semantic = semantic?;
                    let text = text?;
                    Ok(merge_results(semantic, text, limit))
                }
                Err(err) => {
                    eprintln!("Embeddings search failed, falling back to text: {err}");
                    self.supabase.search_by_text(query, limit).await
                }
            }
        } else {
            self.supabase.search_by_text(query, limit).await
        }
    }

    pub async fn append(
        &self,
        memory_type: &str,
        content: &str,
        tags: Vec<String>,
        workspace_id: Option<String>,
    ) -> Result<MemoryEntry, String> {
        if !self.enabled {
            return Err("Memory not enabled".to_string());
        }

        let entry = MemoryEntry {
            id: None,
            content: content.to_string(),
            memory_type: memory_type.to_string(),
            tags,
            workspace_id,
            embedding_status: Some("pending".to_string()),
            created_at: None,
        };

        let inserted = self.supabase.insert_memory(&entry).await?;

        // Queue embedding generation (fire and forget)
        if let (Some(ref embeddings), Some(ref id)) = (&self.embeddings, &inserted.id) {
            let embeddings = embeddings.clone();
            let supabase = self.supabase.clone();
            let id = id.clone();
            let content = content.to_string();

            tokio::spawn(async move {
                match embeddings.generate(&content, "db").await {
                    Ok(result) => {
                        if let Err(e) = supabase
                            .update_memory_embedding(&id, &result.vector, &result.model, result.dim)
                            .await
                        {
                            eprintln!("Failed to update embedding: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to generate embedding: {}", e);
                    }
                }
            });
        }

        Ok(inserted)
    }

    pub async fn bootstrap(&self) -> Result<Vec<MemorySearchResult>, String> {
        if !self.enabled {
            return Err("Memory not enabled".to_string());
        }

        self.supabase.get_bootstrap().await
    }
}

fn merge_results(
    semantic: Vec<MemorySearchResult>,
    text: Vec<MemorySearchResult>,
    limit: usize,
) -> Vec<MemorySearchResult> {
    use std::collections::HashMap;

    let mut by_id: HashMap<String, MemorySearchResult> = HashMap::new();
    let mut scores: HashMap<String, f64> = HashMap::new();

    for item in semantic {
        let key = item.id.clone();
        let score = item
            .score
            .or_else(|| item.distance.map(|d| 1.0 - d))
            .unwrap_or(0.0);
        scores.insert(key.clone(), score);
        by_id.insert(key, item);
    }

    for item in text {
        let key = item.id.clone();
        let rank = item.rank.map(|r| r as f64).unwrap_or(0.0);
        scores
            .entry(key.clone())
            .and_modify(|s| {
                if rank > *s {
                    *s = rank;
                }
            })
            .or_insert(rank);
        by_id.entry(key).or_insert(item);
    }

    let mut entries: Vec<_> = by_id.into_values().collect();
    entries.sort_by(|a, b| {
        let score_a = scores.get(&a.id).copied().unwrap_or(0.0);
        let score_b = scores.get(&b.id).copied().unwrap_or(0.0);
        score_b
            .partial_cmp(&score_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    if entries.len() > limit {
        entries.truncate(limit);
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::embeddings::EmbeddingsClient;
    use crate::memory::supabase::SupabaseClient;
    use httpmock::Method::POST;
    use httpmock::MockServer;
    use serde_json::json;

    #[tokio::test]
    async fn search_merges_semantic_and_text() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(POST).path("/v1/embeddings");
            then.status(200).json_body(json!({
                "data": [{ "embedding": [0.1, 0.2, 0.3] }],
                "model": "embo-01"
            }));
        });

        server.mock(|when, then| {
            when.method(POST)
                .path("/rest/v1/rpc/search_memory_by_embedding");
            then.status(200).json_body(json!([{
                "id": "a",
                "content": "semantic",
                "memory_type": "daily",
                "tags": [],
                "workspace_id": null,
                "created_at": "2026-01-01T00:00:00Z",
                "distance": 0.1,
                "score": 0.9
            }]));
        });

        server.mock(|when, then| {
            when.method(POST).path("/rest/v1/rpc/search_memory_by_text");
            then.status(200).json_body(json!([{
                "id": "b",
                "content": "text",
                "memory_type": "daily",
                "tags": [],
                "created_at": "2026-01-02T00:00:00Z",
                "rank": 0.7
            }]));
        });

        let supabase = SupabaseClient::new(&server.base_url(), "anon");
        let embeddings = EmbeddingsClient::with_base_url("test", &server.url("/v1/embeddings"));
        let service = MemoryService::with_clients(supabase, Some(embeddings), true);

        let results = service.search("hello", 10).await.unwrap();
        let ids: Vec<_> = results.iter().map(|r| r.id.as_str()).collect();
        assert!(ids.contains(&"a"));
        assert!(ids.contains(&"b"));
    }

    #[tokio::test]
    async fn search_falls_back_to_text_on_embedding_error() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(POST).path("/v1/embeddings");
            then.status(500).body("error");
        });

        server.mock(|when, then| {
            when.method(POST).path("/rest/v1/rpc/search_memory_by_text");
            then.status(200).json_body(json!([{
                "id": "c",
                "content": "fallback",
                "memory_type": "daily",
                "tags": [],
                "created_at": "2026-01-03T00:00:00Z",
                "rank": 0.8
            }]));
        });

        let supabase = SupabaseClient::new(&server.base_url(), "anon");
        let embeddings = EmbeddingsClient::with_base_url("test", &server.url("/v1/embeddings"));
        let service = MemoryService::with_clients(supabase, Some(embeddings), true);

        let results = service.search("hello", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "c");
    }
}
