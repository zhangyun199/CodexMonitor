import type { MediaFilterState, MediaType } from "../../types";

const MEDIA_TYPES: Array<MediaType | "all"> = [
  "all",
  "Film",
  "TV",
  "Anime",
  "Game",
  "Book",
  "YouTube",
];

const STATUS_OPTIONS: Array<MediaFilterState["status"]> = [
  "all",
  "Completed",
  "Backlog",
];

const SORT_LABELS: Record<MediaFilterState["sort"], string> = {
  rating: "Rating",
  title: "Title",
  updated: "Recently Updated",
  type: "Type",
};

type MediaFilterBarProps = {
  filters: MediaFilterState;
  onChange: (next: Partial<MediaFilterState>) => void;
};

export function MediaFilterBar({ filters, onChange }: MediaFilterBarProps) {
  return (
    <div className="media-filter-bar">
      <div className="media-filter-row">
        <div className="life-segment-control">
          {MEDIA_TYPES.map((type) => (
            <button
              key={type}
              type="button"
              className={`life-segment-button ${filters.type === type ? "is-active" : ""}`}
              onClick={() => onChange({ type })}
            >
              {type === "all" ? "All" : type}
            </button>
          ))}
        </div>
      </div>

      <div className="media-filter-row">
        <div className="life-segment-control">
          {STATUS_OPTIONS.map((status) => (
            <button
              key={status}
              type="button"
              className={`life-segment-button ${filters.status === status ? "is-active" : ""}`}
              onClick={() => onChange({ status })}
            >
              {status === "all" ? "All" : status}
            </button>
          ))}
        </div>

        <div className="media-filter-controls">
          <select
            className="media-select"
            value={filters.sort}
            onChange={(event) =>
              onChange({ sort: event.target.value as MediaFilterState["sort"] })
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
            className="media-search"
            type="search"
            placeholder="Search titles…"
            value={filters.search}
            onChange={(event) => onChange({ search: event.target.value })}
          />
        </div>
      </div>
    </div>
  );
}
