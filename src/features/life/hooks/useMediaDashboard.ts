import { useCallback, useEffect, useState } from "react";
import { getMediaDashboard } from "../../../services/tauri";
import type { LifeTimeRange, MediaDashboard } from "../types";

export function useMediaDashboard(
  workspaceId: string | null,
  range: LifeTimeRange,
) {
  const [dashboard, setDashboard] = useState<MediaDashboard | null>(null);
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
      const data = await getMediaDashboard(workspaceId, range);
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
