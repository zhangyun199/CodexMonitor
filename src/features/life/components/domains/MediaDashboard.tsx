import { useMemo } from "react";
import type { MediaFilterState, MediaItem, MediaType } from "../../types";
import { useMediaLibrary } from "../../hooks/useMediaLibrary";
import { MediaFilterBar } from "./MediaFilterBar";
import { MediaSection } from "./MediaSection";

const MEDIA_TYPES: MediaType[] = [
  "Anime",
  "Film",
  "TV",
  "Game",
  "Book",
  "YouTube",
];

const TYPE_EMOJI: Record<MediaType, string> = {
  Film: "🎬",
  TV: "📺",
  Anime: "🎌",
  Game: "🎮",
  Book: "📚",
  YouTube: "🎥",
};

type MediaDashboardProps = {
  workspaceId: string | null;
};

export function MediaDashboard({ workspaceId }: MediaDashboardProps) {
  const { library, summary, filters, updateFilters, loading, error, refresh } =
    useMediaLibrary(workspaceId);

  const filteredItems = useMemo(() => {
    if (!library) return [];
    return applyFilters(library.items, filters);
  }, [library, filters]);

  const grouped = useMemo(() => {
    const groups = new Map<MediaType, MediaItem[]>();
    for (const type of MEDIA_TYPES) {
      groups.set(type, []);
    }
    for (const item of filteredItems) {
      const list = groups.get(item.type) ?? [];
      list.push(item);
      groups.set(item.type, list);
    }
    for (const [key, list] of groups) {
      groups.set(key, sortItems(list, filters.sort));
    }
    return groups;
  }, [filteredItems, filters.sort]);

  const headerLine = summary
    ? `${summary.totalCount} items • ${summary.completedCount} completed • ⭐ ${summary.avgRating.toFixed(
        1,
      )} avg`
    : "--";

  return (
    <div className="life-dashboard life-media-dashboard">
      <div className="life-dashboard-header">
        <div>
          <div className="life-dashboard-title">🎬 Media Library</div>
          <div className="life-dashboard-subtitle">{headerLine}</div>
        </div>
        <div className="life-dashboard-actions">
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

      <MediaFilterBar filters={filters} onChange={updateFilters} />

      {error && <div className="life-dashboard-error">{error}</div>}
      {loading && !library && (
        <div className="life-dashboard-status">Loading media library…</div>
      )}

      {library ? (
        <div className="media-sections">
          {MEDIA_TYPES.map((type) => {
            const items = grouped.get(type) ?? [];
            if (items.length === 0) return null;
            return (
              <MediaSection
                key={type}
                title={`${TYPE_EMOJI[type]} ${type}`}
                count={items.length}
                items={items}
                viewMode={filters.viewMode}
              />
            );
          })}
        </div>
      ) : null}
    </div>
  );
}

function applyFilters(items: MediaItem[], filters: MediaFilterState) {
  return items.filter((item) => {
    if (filters.type !== "all" && item.type !== filters.type) {
      return false;
    }
    if (filters.status !== "all" && item.status !== filters.status) {
      return false;
    }
    if (filters.search.trim()) {
      const query = filters.search.toLowerCase();
      if (!item.title.toLowerCase().includes(query)) {
        return false;
      }
    }
    return true;
  });
}

function sortItems(items: MediaItem[], sort: MediaFilterState["sort"]) {
  const list = [...items];
  switch (sort) {
    case "rating":
      return list.sort((a, b) => (b.rating ?? 0) - (a.rating ?? 0));
    case "title":
      return list.sort((a, b) => a.title.localeCompare(b.title));
    case "updated":
      return list.sort(
        (a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime(),
      );
    case "type":
      return list.sort((a, b) => a.type.localeCompare(b.type));
    default:
      return list;
  }
}
