import { useCallback, useEffect, useMemo, useState } from "react";
import { getYouTubeDashboard } from "../../../services/tauri";
import type { YouTubeLibrary, YouTubeFilterState } from "../types";

const DEFAULT_FILTERS: YouTubeFilterState = {
  tier: "all",
  stage: "all",
  search: "",
  sort: "tier",
  viewMode: "grid",
};

export function useYouTubeLibrary(workspaceId: string | null) {
  const [library, setLibrary] = useState<YouTubeLibrary | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [filters, setFilters] = useState<YouTubeFilterState>(DEFAULT_FILTERS);

  const refresh = useCallback(async () => {
    if (!workspaceId) {
      setLibrary(null);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const data = await getYouTubeDashboard(workspaceId);
      setLibrary(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [workspaceId]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const updateFilters = useCallback((next: Partial<YouTubeFilterState>) => {
    setFilters((prev) => ({ ...prev, ...next }));
  }, []);

  const summary = useMemo(() => {
    if (!library) return null;
    return {
      totalCount: library.totalCount,
      inProgressCount: library.inProgressCount,
      publishedCount: library.publishedCount,
    };
  }, [library]);

  return { library, summary, filters, updateFilters, loading, error, refresh };
}
