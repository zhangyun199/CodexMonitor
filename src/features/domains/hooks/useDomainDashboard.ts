import { useCallback, useEffect, useState } from "react";
import { getDomainTrends } from "../../../services/tauri";
import type { DomainTrendSnapshot } from "../../../types";

export type DomainTrendRange = "7d" | "30d" | "lifetime";

export function useDomainDashboard(
  workspaceId: string | null,
  domainId: string | null,
  range: DomainTrendRange,
) {
  const [snapshot, setSnapshot] = useState<DomainTrendSnapshot | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    if (!workspaceId || !domainId) {
      setSnapshot(null);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const data = await getDomainTrends(workspaceId, domainId, range);
      setSnapshot(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [workspaceId, domainId, range]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  return { snapshot, loading, error, refresh };
}
