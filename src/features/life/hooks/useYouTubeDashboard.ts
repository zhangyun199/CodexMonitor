import { useCallback, useEffect, useState } from "react";
import { getYouTubeDashboard } from "../../../services/tauri";
import type { LifeTimeRange, YouTubeDashboard } from "../types";

export function useYouTubeDashboard(
  workspaceId: string | null,
  range: LifeTimeRange,
) {
  const [dashboard, setDashboard] = useState<YouTubeDashboard | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    if (!workspaceId) {
      setDashboard(null);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const data = await getYouTubeDashboard(workspaceId, range);
      setDashboard(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [workspaceId, range]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  return { dashboard, loading, error, refresh };
}
