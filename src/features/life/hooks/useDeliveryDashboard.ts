import { useCallback, useEffect, useState } from "react";
import { getDeliveryDashboard } from "../../../services/tauri";
import type { DeliveryDashboard, LifeTimeRange } from "../types";

export function useDeliveryDashboard(
  workspaceId: string | null,
  range: LifeTimeRange,
) {
  const [dashboard, setDashboard] = useState<DeliveryDashboard | null>(null);
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
      const data = await getDeliveryDashboard(workspaceId, range);
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
