-- Semantic search for memory table
-- Based on existing search_notes_by_embedding from life-mcp

CREATE OR REPLACE FUNCTION search_memory_by_embedding(
  query_embedding vector,
  match_count int DEFAULT 10,
  max_distance float8 DEFAULT NULL,
  filter_type text DEFAULT NULL,
  filter_workspace text DEFAULT NULL
)
RETURNS TABLE (
  id uuid,
  content text,
  memory_type text,
  tags text[],
  workspace_id text,
  created_at timestamptz,
  distance float8,
  score float8
)
LANGUAGE sql
STABLE
AS $$
  SELECT
    m.id,
    m.content,
    m.memory_type,
    m.tags,
    m.workspace_id,
    m.created_at,
    (m.embedding <=> query_embedding) AS distance,
    (1 - (m.embedding <=> query_embedding)::float8) AS score
  FROM memory m
  WHERE m.embedding IS NOT NULL
    AND m.embedding_status = 'ready'
    AND (max_distance IS NULL OR (m.embedding <=> query_embedding) <= max_distance)
    AND (filter_type IS NULL OR m.memory_type = filter_type)
    AND (filter_workspace IS NULL OR m.workspace_id = filter_workspace)
  ORDER BY m.embedding <=> query_embedding
  LIMIT match_count;
$$;

-- Full-text search fallback (when no embedding available)
CREATE OR REPLACE FUNCTION search_memory_by_text(
  search_query text,
  match_count int DEFAULT 10,
  filter_type text DEFAULT NULL
)
RETURNS TABLE (
  id uuid,
  content text,
  memory_type text,
  tags text[],
  created_at timestamptz,
  rank float4
)
LANGUAGE sql
STABLE
AS $$
  SELECT
    m.id,
    m.content,
    m.memory_type,
    m.tags,
    m.created_at,
    ts_rank(to_tsvector('english', m.content), plainto_tsquery('english', search_query)) AS rank
  FROM memory m
  WHERE to_tsvector('english', m.content) @@ plainto_tsquery('english', search_query)
    AND (filter_type IS NULL OR m.memory_type = filter_type)
  ORDER BY rank DESC
  LIMIT match_count;
$$;

-- Bootstrap function (get recent memory for context)
CREATE OR REPLACE FUNCTION get_memory_bootstrap(
  curated_limit int DEFAULT 50,
  daily_limit int DEFAULT 20
)
RETURNS TABLE (
  id uuid,
  content text,
  memory_type text,
  tags text[],
  created_at timestamptz
)
LANGUAGE sql
STABLE
AS $$
  (
    SELECT id, content, memory_type, tags, created_at
    FROM memory
    WHERE memory_type = 'curated'
    ORDER BY created_at DESC
    LIMIT curated_limit
  )
  UNION ALL
  (
    SELECT id, content, memory_type, tags, created_at
    FROM memory
    WHERE memory_type = 'daily'
      AND created_at > now() - interval '7 days'
    ORDER BY created_at DESC
    LIMIT daily_limit
  );
$$;
