-- CodexMonitor Memory Table
-- Based on existing notes table pattern from life-mcp

-- Enable pgvector if not already (should already be enabled)
CREATE EXTENSION IF NOT EXISTS vector;

-- Memory table for CodexMonitor
CREATE TABLE IF NOT EXISTS memory (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

  -- Content
  content TEXT NOT NULL,

  -- Classification
  memory_type TEXT NOT NULL DEFAULT 'daily',  -- 'daily' or 'curated'
  tags TEXT[] DEFAULT ARRAY[]::TEXT[],

  -- Source tracking
  source TEXT NOT NULL DEFAULT 'codexmonitor',  -- 'codexmonitor', 'ios', 'daemon'
  workspace_id TEXT,  -- Optional: link to CodexMonitor workspace

  -- Embedding columns (same pattern as notes table)
  embedding vector,
  embedding_model TEXT,
  embedding_dim INT,
  embedding_status TEXT DEFAULT 'pending',
  embedding_updated_at TIMESTAMPTZ,
  embedding_error TEXT,

  -- Timestamps
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now(),

  -- Constraints
  CONSTRAINT memory_type_check CHECK (memory_type IN ('daily', 'curated')),
  CONSTRAINT memory_embedding_status_check CHECK (embedding_status IN ('pending', 'ready', 'error'))
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_memory_created_at ON memory(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_memory_type ON memory(memory_type);
CREATE INDEX IF NOT EXISTS idx_memory_tags ON memory USING GIN(tags);
CREATE INDEX IF NOT EXISTS idx_memory_workspace ON memory(workspace_id);
CREATE INDEX IF NOT EXISTS idx_memory_embedding_status ON memory(embedding_status);

-- Full-text search index (BM25 fallback)
CREATE INDEX IF NOT EXISTS idx_memory_content_fts ON memory USING GIN(to_tsvector('english', content));

-- RLS (allow all for now - single user)
ALTER TABLE memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Allow all for memory" ON memory FOR ALL USING (true);

-- Updated_at trigger
CREATE OR REPLACE FUNCTION update_memory_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER memory_updated_at_trigger
  BEFORE UPDATE ON memory
  FOR EACH ROW
  EXECUTE FUNCTION update_memory_updated_at();
