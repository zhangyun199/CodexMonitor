import { useCallback, useEffect, useMemo, useState } from "react";
import { getMediaDashboard } from "../../../services/tauri";
import type { MediaFilterState, MediaLibrary } from "../types";

const DEFAULT_FILTERS: MediaFilterState = {
  type: "all",
  status: "all",
  search: "",
  sort: "rating",
  viewMode: "grid",
};

export function useMediaLibrary(workspaceId: string | null) {
  const [library, setLibrary] = useState<MediaLibrary | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [filters, setFilters] = useState<MediaFilterState>(DEFAULT_FILTERS);

  const refresh = useCallback(async () => {
    if (!workspaceId) {
      setLibrary(null);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const data = await getMediaDashboard(workspaceId);
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

  const updateFilters = useCallback(
    (next: Partial<MediaFilterState>) => {
      setFilters((prev) => ({ ...prev, ...next }));
    },
    [],
  );

  const summary = useMemo(() => {
    if (!library) return null;
    return {
      totalCount: library.totalCount,
      completedCount: library.completedCount,
      backlogCount: library.backlogCount,
      avgRating: library.avgRating,
    };
  }, [library]);

  return { library, summary, filters, updateFilters, loading, error, refresh };
}
