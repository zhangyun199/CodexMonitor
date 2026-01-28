import { useMemo } from "react";
import type { LifeTimeRange } from "../../types";
import { useMediaDashboard } from "../../hooks/useMediaDashboard";
import { StatCard } from "../shared/StatCard";
import { TimeRangeSelector } from "../shared/TimeRangeSelector";

type MediaDashboardProps = {
  workspaceId: string | null;
  range: LifeTimeRange;
  onRangeChange: (range: LifeTimeRange) => void;
};

export function MediaDashboard({
  workspaceId,
  range,
  onRangeChange,
}: MediaDashboardProps) {
  const { dashboard, loading, error, refresh } = useMediaDashboard(
    workspaceId,
    range,
  );

  const stats = dashboard?.stats;
  const recent = dashboard?.recentlyActive ?? [];
  const byType = dashboard?.byType ?? {};

  const avgRating = useMemo(() => {
    if (!stats?.avgRating) return "--";
    return stats.avgRating.toFixed(1);
  }, [stats]);

  return (
    <div className="life-dashboard life-media-dashboard">
      <div className="life-dashboard-header">
        <div>
          <div className="life-dashboard-title">🎬 Media Dashboard</div>
          <div className="life-dashboard-subtitle">
            {dashboard?.meta ? `${dashboard.meta.periodStart} → ${dashboard.meta.periodEnd}` : ""}
          </div>
        </div>
        <div className="life-dashboard-actions">
          <TimeRangeSelector value={range} onChange={onRangeChange} />
          <button
            type="button"
            className="ghost life-refresh-button"
            onClick={() => void refresh()}
            disabled={loading}
          >
            Refresh
          </button>
        </div>
      </div>

      {error && <div className="life-dashboard-error">{error}</div>}
      {loading && !dashboard && (
        <div className="life-dashboard-status">Loading media data…</div>
      )}

      {dashboard ? (
        <>
          <div className="life-stat-grid">
            <StatCard label="Backlog" value={String(stats?.backlogCount ?? 0)} />
            <StatCard label="In Progress" value={String(stats?.inProgressCount ?? 0)} />
            <StatCard label="Completed" value={String(stats?.completedCount ?? 0)} />
            <StatCard label="Avg Rating" value={avgRating} />
          </div>

          <section className="life-section">
            <div className="life-section-title">By Type</div>
            {Object.keys(byType).length ? (
              <div className="life-tag-grid">
                {Object.entries(byType).map(([type, count]) => (
                  <div key={type} className="life-tag">
                    <span className="life-tag-label">{type}</span>
                    <span className="life-tag-value">{String(count)}</span>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No type breakdown yet.</div>
            )}
          </section>

          <section className="life-section">
            <div className="life-section-title">Recently Active</div>
            {recent.length ? (
              <div className="life-media-list">
                {recent.map((item) => (
                  <div key={item.id} className="life-media-card">
                    <div className="life-media-title">{item.title}</div>
                    <div className="life-media-meta">
                      {item.type} · {item.status.replace("_", " ")}
                      {item.rating ? ` · ⭐ ${item.rating.toFixed(1)}` : ""}
                    </div>
                    {item.lastActivityAt && (
                      <div className="life-media-time">
                        {new Date(item.lastActivityAt).toLocaleDateString()}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No recent activity.</div>
            )}
          </section>
        </>
      ) : null}
    </div>
  );
}
