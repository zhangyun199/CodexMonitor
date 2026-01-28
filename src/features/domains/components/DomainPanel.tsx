import RefreshCcw from "lucide-react/dist/esm/icons/refresh-ccw";
import { useState } from "react";
import type { Domain } from "../../../types";
import { formatRelativeTime } from "../../../utils/time";
import { useDomainDashboard } from "../hooks/useDomainDashboard";
import { DomainDashboard } from "./DomainDashboard";

export type DomainPanelProps = {
  workspaceId: string | null;
  domain: Domain | null;
};

export function DomainPanel({ workspaceId, domain }: DomainPanelProps) {
  const [range, setRange] = useState<"7d" | "30d" | "lifetime">("7d");
  const { snapshot, loading, error, refresh } = useDomainDashboard(
    workspaceId,
    domain?.id ?? null,
    range,
  );

  const updatedAt = snapshot?.updatedAt ?? null;
  const parsedUpdatedAt = updatedAt ? Date.parse(updatedAt) : Number.NaN;
  const updatedLabel = updatedAt
    ? Number.isNaN(parsedUpdatedAt)
      ? `Updated ${updatedAt}`
      : `Updated ${formatRelativeTime(parsedUpdatedAt)}`
    : "No snapshot yet";
  const stale = false;

  return (
    <div className="domain-panel">
      <div className="domain-panel-header">
        <div>
          <div className="domain-panel-title">
            {domain ? `${domain.theme.icon} ${domain.name}` : "Domain"}
          </div>
          <div className="domain-panel-subtitle">{updatedLabel}</div>
          {stale && <div className="domain-panel-chip">Stale data</div>}
        </div>
        <div className="domain-panel-actions">
          <div className="domain-panel-range">
            {(["7d", "30d", "lifetime"] as const).map((value) => (
              <button
                key={value}
                type="button"
                className={`domain-range-button${
                  range === value ? " is-active" : ""
                }`}
                onClick={() => setRange(value)}
              >
                {value}
              </button>
            ))}
          </div>
          <button
            type="button"
            className="ghost icon-button"
            onClick={() => void refresh()}
            title="Refresh dashboard"
            disabled={loading}
          >
            <RefreshCcw aria-hidden />
          </button>
        </div>
      </div>

      {error && <div className="domain-panel-error">{error}</div>}
      {loading && !snapshot && (
        <div className="domain-panel-status">Loading dashboard…</div>
      )}

      {snapshot ? (
        <DomainDashboard snapshot={snapshot} />
      ) : !loading ? (
        <div className="domain-panel-status">No dashboard data.</div>
      ) : null}
    </div>
  );
}
