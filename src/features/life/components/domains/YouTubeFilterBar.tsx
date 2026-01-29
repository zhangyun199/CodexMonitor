import type {
  YouTubeFilterState,
  YouTubeStage,
  YouTubeTier,
} from "../../types";

const TIERS: Array<YouTubeTier | "all"> = ["all", "S", "A", "B", "C"];
const STAGES: Array<YouTubeStage | "all"> = [
  "all",
  "idea",
  "notes",
  "outline",
  "draft",
  "script",
  "ready",
  "published",
];

const SORT_LABELS: Record<YouTubeFilterState["sort"], string> = {
  tier: "Tier",
  stage: "Stage",
  title: "Title",
  updated: "Updated",
};

type YouTubeFilterBarProps = {
  filters: YouTubeFilterState;
  onChange: (next: Partial<YouTubeFilterState>) => void;
};

export function YouTubeFilterBar({ filters, onChange }: YouTubeFilterBarProps) {
  return (
    <div className="youtube-filter-bar">
      <div className="youtube-filter-row">
        <div className="life-segment-control">
          {TIERS.map((tier) => (
            <button
              key={tier}
              type="button"
              className={`life-segment-button ${filters.tier === tier ? "is-active" : ""}`}
              onClick={() => onChange({ tier })}
            >
              {tier === "all" ? "All" : tier}
            </button>
          ))}
        </div>

        <select
          className="youtube-select"
          value={filters.stage}
          onChange={(event) =>
            onChange({ stage: event.target.value as YouTubeFilterState["stage"] })
          }
        >
          {STAGES.map((stage) => (
            <option key={stage} value={stage}>
              {stage === "all" ? "All stages" : stage}
            </option>
          ))}
        </select>
      </div>

      <div className="youtube-filter-row">
        <div className="youtube-filter-controls">
          <select
            className="youtube-select"
            value={filters.sort}
            onChange={(event) =>
              onChange({ sort: event.target.value as YouTubeFilterState["sort"] })
            }
          >
            {Object.entries(SORT_LABELS).map(([value, label]) => (
              <option key={value} value={value}>
                {label}
              </option>
            ))}
          </select>

          <div className="life-segment-control">
            <button
              type="button"
              className={`life-segment-button ${filters.viewMode === "grid" ? "is-active" : ""}`}
              onClick={() => onChange({ viewMode: "grid" })}
            >
              Grid
            </button>
            <button
              type="button"
              className={`life-segment-button ${filters.viewMode === "list" ? "is-active" : ""}`}
              onClick={() => onChange({ viewMode: "list" })}
            >
              List
            </button>
          </div>

          <input
            className="youtube-search"
            type="search"
            placeholder="Search ideas…"
            value={filters.search}
            onChange={(event) => onChange({ search: event.target.value })}
          />
        </div>
      </div>
    </div>
  );
}
