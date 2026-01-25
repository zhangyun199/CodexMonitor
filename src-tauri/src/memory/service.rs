//! Memory service combining Supabase + MiniMax
//! Reference: /Volumes/YouTube 4TB/code/_archive/life-mcp/src/supabase/note-embeddings.js

use super::embeddings::EmbeddingsClient;
use super::supabase::{MemoryEntry, MemorySearchResult, SupabaseClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

        // If embeddings available, use semantic search
        if let Some(ref embeddings) = self.embeddings {
            let result = embeddings.generate(query, "query").await?;
            self.supabase
                .search_by_embedding(&result.vector, limit, Some(0.5))
                .await
        } else {
            // Fall back to text search
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
