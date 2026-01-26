import { useCallback, useEffect, useMemo, useState } from "react";
import Search from "lucide-react/dist/esm/icons/search";
import RotateCcw from "lucide-react/dist/esm/icons/rotate-ccw";
import PlusCircle from "lucide-react/dist/esm/icons/plus-circle";
import type { MemorySearchResult, MemoryStatus } from "../../../types";
import {
  memoryAppend,
  memoryBootstrap,
  memorySearch,
  memoryStatus,
} from "../../../services/tauri";
import { formatRelativeTime } from "../../../utils/time";

export type MemoryPanelProps = {
  workspaceId: string | null;
};

const DEFAULT_LIMIT = 20;

export function MemoryPanel({ workspaceId }: MemoryPanelProps) {
  const [status, setStatus] = useState<MemoryStatus | null>(null);
  const [results, setResults] = useState<MemorySearchResult[]>([]);
  const [query, setQuery] = useState("");
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [memoryType, setMemoryType] = useState<"daily" | "curated">("daily");
  const [newContent, setNewContent] = useState("");
  const [newTags, setNewTags] = useState("");

  const parseTags = useCallback((value: string) => {
    return value
      .split(",")
      .map((item) => item.trim())
      .filter(Boolean);
  }, []);

  const refreshAll = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const latestStatus = await memoryStatus();
      setStatus(latestStatus);
      if (latestStatus.enabled) {
        const entries = await memoryBootstrap();
        setResults(entries);
      } else {
        setResults([]);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  const runSearch = useCallback(async () => {
    const trimmed = query.trim();
    if (!trimmed) {
      await refreshAll();
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const entries = await memorySearch(trimmed, DEFAULT_LIMIT);
      setResults(entries);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [query, refreshAll]);

  const handleSave = useCallback(async () => {
    const trimmed = newContent.trim();
    if (!trimmed || saving) {
      return;
    }
    setSaving(true);
    setError(null);
    try {
      await memoryAppend(memoryType, trimmed, parseTags(newTags), workspaceId);
      setNewContent("");
      setNewTags("");
      await refreshAll();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setSaving(false);
    }
  }, [memoryType, newContent, newTags, parseTags, refreshAll, saving, workspaceId]);

  useEffect(() => {
    void refreshAll();
  }, [refreshAll]);

  const enabled = status?.enabled ?? false;
  const statusLabel = status
    ? status.enabled
      ? status.embeddings_enabled
        ? "Embeddings"
        : "Text"
      : "Disabled"
    : "Loading";

  const statusSummary = useMemo(() => {
    if (!status) {
      return "";
    }
    if (!status.enabled) {
      return "Memory is disabled in settings.";
    }
    return `${status.total} total • ${status.ready} ready • ${status.pending} pending`;
  }, [status]);

  return (
    <div className="memory-panel">
      <div className="memory-panel-header">
        <div>
          <div className="memory-panel-title">Memory</div>
          <div className="memory-panel-subtitle">{statusSummary}</div>
        </div>
        <div className="memory-panel-actions">
          <span className={`memory-panel-chip ${enabled ? "is-on" : "is-off"}`}>
            {statusLabel}
          </span>
          <button
            type="button"
            className="ghost memory-panel-icon"
            onClick={() => void refreshAll()}
            title="Refresh"
            disabled={loading}
          >
            <RotateCcw size={14} aria-hidden />
          </button>
        </div>
      </div>

      <div className="memory-panel-search">
        <Search size={14} aria-hidden />
        <input
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Search memory"
          onKeyDown={(event) => {
            if (event.key === "Enter") {
              event.preventDefault();
              void runSearch();
            }
          }}
        />
        <button type="button" className="ghost" onClick={() => void runSearch()}>
          Search
        </button>
      </div>

      {error && <div className="memory-panel-error">{error}</div>}

      <div className="memory-panel-results">
        {loading ? (
          <div className="memory-panel-placeholder">Loading memories…</div>
        ) : results.length === 0 ? (
          <div className="memory-panel-placeholder">
            {enabled ? "No memory entries yet." : "Memory is disabled."}
          </div>
        ) : (
          results.map((entry) => (
            <article key={entry.id} className="memory-panel-entry">
              <div className="memory-panel-entry-content">{entry.content}</div>
              <div className="memory-panel-entry-meta">
                <span className="memory-panel-tag memory-panel-tag--type">
                  {entry.memory_type}
                </span>
                {typeof entry.score === "number" && (
                  <span className="memory-panel-tag">score {entry.score.toFixed(2)}</span>
                )}
                {typeof entry.rank === "number" && (
                  <span className="memory-panel-tag">rank {entry.rank.toFixed(2)}</span>
                )}
                {entry.created_at && (
                  <span className="memory-panel-timestamp">
                    {Number.isNaN(Date.parse(entry.created_at))
                      ? entry.created_at
                      : formatRelativeTime(Date.parse(entry.created_at))}
                  </span>
                )}
              </div>
              {entry.tags.length > 0 && (
                <div className="memory-panel-tags">
                  {entry.tags.map((tag) => (
                    <span key={tag} className="memory-panel-tag">
                      {tag}
                    </span>
                  ))}
                </div>
              )}
            </article>
          ))
        )}
      </div>

      <div className="memory-panel-compose">
        <div className="memory-panel-compose-header">
          <PlusCircle size={14} aria-hidden />
          <span>Add Memory</span>
        </div>
        <div className="memory-panel-compose-row">
          <label>
            Type
            <select
              value={memoryType}
              onChange={(event) => setMemoryType(event.target.value as "daily" | "curated")}
            >
              <option value="daily">Daily</option>
              <option value="curated">Curated</option>
            </select>
          </label>
          <label>
            Tags
            <input
              value={newTags}
              onChange={(event) => setNewTags(event.target.value)}
              placeholder="comma,separated"
            />
          </label>
        </div>
        <textarea
          value={newContent}
          onChange={(event) => setNewContent(event.target.value)}
          placeholder="Capture a memory…"
          rows={3}
        />
        <div className="memory-panel-compose-actions">
          <button
            type="button"
            className="primary"
            onClick={() => void handleSave()}
            disabled={!enabled || !newContent.trim() || saving}
          >
            {saving ? "Saving…" : "Save memory"}
          </button>
        </div>
      </div>
    </div>
  );
}
