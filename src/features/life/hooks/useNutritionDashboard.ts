import { useCallback, useEffect, useState } from "react";
import { getNutritionDashboard } from "../../../services/tauri";
import type { LifeTimeRange, NutritionDashboard } from "../types";

export function useNutritionDashboard(
  workspaceId: string | null,
  range: LifeTimeRange,
) {
  const [dashboard, setDashboard] = useState<NutritionDashboard | null>(null);
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
      const data = await getNutritionDashboard(workspaceId, range);
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
