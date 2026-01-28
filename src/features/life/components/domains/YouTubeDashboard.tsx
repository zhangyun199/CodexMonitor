import type { LifeTimeRange, PipelineStage } from "../../types";
import { useYouTubeDashboard } from "../../hooks/useYouTubeDashboard";
import { StatCard } from "../shared/StatCard";
import { TimeRangeSelector } from "../shared/TimeRangeSelector";

const STAGE_LABELS: Record<PipelineStage, string> = {
  brain_dump: "Brain Dump",
  development: "Development",
  outline: "Outline",
  evaluation: "Evaluation",
  script: "Script",
  edit: "Edit",
  published: "Published",
};

const STAGE_ORDER: PipelineStage[] = [
  "brain_dump",
  "development",
  "outline",
  "evaluation",
  "script",
  "edit",
  "published",
];

type YouTubeDashboardProps = {
  workspaceId: string | null;
  range: LifeTimeRange;
  onRangeChange: (range: LifeTimeRange) => void;
};

export function YouTubeDashboard({
  workspaceId,
  range,
  onRangeChange,
}: YouTubeDashboardProps) {
  const { dashboard, loading, error, refresh } = useYouTubeDashboard(
    workspaceId,
    range,
  );

  const stats = dashboard?.pipelineStats ?? ({} as Record<PipelineStage, number>);

  return (
    <div className="life-dashboard life-youtube-dashboard">
      <div className="life-dashboard-header">
        <div>
          <div className="life-dashboard-title">🎥 YouTube Pipeline</div>
          <div className="life-dashboard-subtitle">
            {dashboard?.meta
              ? `${dashboard.meta.periodStart} → ${dashboard.meta.periodEnd}`
              : ""}
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
        <div className="life-dashboard-status">Loading YouTube data…</div>
      )}

      {dashboard ? (
        <>
          <div className="life-stat-grid">
            {STAGE_ORDER.map((stage) => (
              <StatCard
                key={stage}
                label={STAGE_LABELS[stage]}
                value={String(stats[stage] ?? 0)}
              />
            ))}
          </div>

          <section className="life-section">
            <div className="life-section-title">S-Tier Ideas</div>
            {dashboard.sTier.length ? (
              <div className="life-media-list">
                {dashboard.sTier.map((idea) => (
                  <div key={idea.id} className="life-media-card">
                    <div className="life-media-title">{idea.title}</div>
                    <div className="life-media-meta">
                      {idea.stage.replace("_", " ")} • Tier {idea.tier}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No S-tier ideas yet.</div>
            )}
          </section>

          <section className="life-section">
            <div className="life-section-title">In Progress</div>
            {dashboard.inProgress.length ? (
              <div className="life-media-list">
                {dashboard.inProgress.map((idea) => (
                  <div key={idea.id} className="life-media-card">
                    <div className="life-media-title">{idea.title}</div>
                    <div className="life-media-meta">
                      {idea.stage.replace("_", " ")} • Tier {idea.tier}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No ideas in progress.</div>
            )}
          </section>
        </>
      ) : null}
    </div>
  );
}
