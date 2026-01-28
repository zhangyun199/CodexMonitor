import { useMemo } from "react";
import type { ExerciseEntry, LifeTimeRange } from "../../types";
import { useExerciseDashboard } from "../../hooks/useExerciseDashboard";
import { StatCard } from "../shared/StatCard";
import { TimeRangeSelector } from "../shared/TimeRangeSelector";

const TYPE_EMOJI: Record<ExerciseEntry["type"], string> = {
  walk: "🚶",
  strength: "🏋️",
  cardio: "🏃",
  other: "✨",
};

type ExerciseDashboardProps = {
  workspaceId: string | null;
  range: LifeTimeRange;
  onRangeChange: (range: LifeTimeRange) => void;
};

export function ExerciseDashboard({
  workspaceId,
  range,
  onRangeChange,
}: ExerciseDashboardProps) {
  const { dashboard, loading, error, refresh } = useExerciseDashboard(
    workspaceId,
    range,
  );

  const stats = dashboard?.stats;
  const entries = dashboard?.entries ?? [];
  const byType = dashboard?.byType ?? {};

  const breakdown = useMemo(() => {
    const rows = Object.entries(byType).sort(
      (a, b) => (b[1] ?? 0) - (a[1] ?? 0),
    );
    const maxValue = Math.max(1, ...rows.map(([, value]) => value ?? 0));
    return rows.map(([type, count]) => ({
      type,
      count,
      percent: maxValue > 0 ? (count / maxValue) * 100 : 0,
    }));
  }, [byType]);

  return (
    <div className="life-dashboard life-exercise-dashboard">
      <div className="life-dashboard-header">
        <div>
          <div className="life-dashboard-title">🏋️ Exercise Dashboard</div>
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
        <div className="life-dashboard-status">Loading exercise data…</div>
      )}

      {dashboard ? (
        <>
          <div className="life-stat-grid">
            <StatCard
              label="Workouts"
              value={stats ? String(stats.workoutCount) : "--"}
            />
            <StatCard
              label="Walking"
              value={stats ? `${stats.walkingMiles.toFixed(1)} mi` : "--"}
            />
            <StatCard
              label="Active Days"
              value={stats ? String(stats.activeDays) : "--"}
            />
            <StatCard
              label="Streak"
              value={stats ? `${stats.currentStreak} 🔥` : "--"}
            />
          </div>

          <section className="life-section">
            <div className="life-section-title">This Week</div>
            {entries.length ? (
              <div className="life-list">
                {entries.map((entry) => (
                  <div key={entry.id} className="life-list-item">
                    <div className="life-list-title">
                      {TYPE_EMOJI[entry.type]} {entry.description}
                    </div>
                    <div className="life-list-meta">
                      {formatEntryTime(entry.timestamp)}
                      {entry.miles ? ` • ${entry.miles.toFixed(1)} mi` : ""}
                      {entry.duration ? ` • ${entry.duration.toFixed(0)} min` : ""}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No exercise logged yet.</div>
            )}
          </section>

          <section className="life-section">
            <div className="life-section-title">Activity Breakdown</div>
            {breakdown.length ? (
              <div className="life-bar-chart">
                {breakdown.map((row) => (
                  <div key={row.type} className="life-bar-row">
                    <div className="life-bar-label">
                      {TYPE_EMOJI[row.type as ExerciseEntry["type"]] ?? "✨"} {row.type}
                    </div>
                    <div className="life-bar-track">
                      <div
                        className="life-bar-fill"
                        style={{ width: `${row.percent}%` }}
                      />
                    </div>
                    <div className="life-bar-value">{row.count}</div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No activity yet.</div>
            )}
          </section>
        </>
      ) : null}
    </div>
  );
}

function formatEntryTime(timestamp: string) {
  if (timestamp.length >= 16) {
    return timestamp.slice(11, 16);
  }
  return timestamp;
}
