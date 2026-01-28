import { useMemo } from "react";
import type { YouTubeFilterState, YouTubeIdea, YouTubeTier } from "../../types";
import { useYouTubeLibrary } from "../../hooks/useYouTubeLibrary";
import { YouTubeFilterBar } from "./YouTubeFilterBar";
import { YouTubeSection } from "./YouTubeSection";

const TIERS: YouTubeTier[] = ["S", "A", "B", "C"];
const TIER_LABELS: Record<YouTubeTier, string> = {
  S: "⭐ S-Tier",
  A: "🥈 A-Tier",
  B: "🥉 B-Tier",
  C: "🪨 C-Tier",
};

type YouTubeDashboardProps = {
  workspaceId: string | null;
};

export function YouTubeDashboard({ workspaceId }: YouTubeDashboardProps) {
  const { library, summary, filters, updateFilters, loading, error, refresh } =
    useYouTubeLibrary(workspaceId);

  const filteredIdeas = useMemo(() => {
    if (!library) return [];
    return applyFilters(library.items, filters);
  }, [library, filters]);

  const grouped = useMemo(() => {
    const groups = new Map<YouTubeTier, YouTubeIdea[]>();
    for (const tier of TIERS) {
      groups.set(tier, []);
    }
    for (const idea of filteredIdeas) {
      const list = groups.get(idea.tier) ?? [];
      list.push(idea);
      groups.set(idea.tier, list);
    }
    for (const [key, list] of groups) {
      groups.set(key, sortIdeas(list, filters.sort));
    }
    return groups;
  }, [filteredIdeas, filters.sort]);

  const headerLine = summary
    ? `${summary.totalCount} ideas • ${summary.inProgressCount} in progress • ${summary.publishedCount} published`
    : "--";

  return (
    <div className="life-dashboard life-youtube-dashboard">
      <div className="life-dashboard-header">
        <div>
          <div className="life-dashboard-title">🎥 YouTube Ideas</div>
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

      <YouTubeFilterBar filters={filters} onChange={updateFilters} />

      {error && <div className="life-dashboard-error">{error}</div>}
      {loading && !library && (
        <div className="life-dashboard-status">Loading YouTube ideas…</div>
      )}

      {library ? (
        <div className="youtube-sections">
          {TIERS.map((tier) => {
            const ideas = grouped.get(tier) ?? [];
            if (ideas.length === 0) return null;
            return (
              <YouTubeSection
                key={tier}
                title={TIER_LABELS[tier]}
                count={ideas.length}
                ideas={ideas}
                viewMode={filters.viewMode}
              />
            );
          })}
        </div>
      ) : null}
    </div>
  );
}

function applyFilters(ideas: YouTubeIdea[], filters: YouTubeFilterState) {
  return ideas.filter((idea) => {
    if (filters.tier !== "all" && idea.tier !== filters.tier) {
      return false;
    }
    if (filters.stage !== "all" && idea.stage !== filters.stage) {
      return false;
    }
    if (filters.search.trim()) {
      const query = filters.search.toLowerCase();
      if (!idea.title.toLowerCase().includes(query)) {
        return false;
      }
    }
    return true;
  });
}

function sortIdeas(ideas: YouTubeIdea[], sort: YouTubeFilterState["sort"]) {
  const list = [...ideas];
  switch (sort) {
    case "title":
      return list.sort((a, b) => a.title.localeCompare(b.title));
    case "stage":
      return list.sort((a, b) => a.stage.localeCompare(b.stage));
    case "updated":
      return list.sort(
        (a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime(),
      );
    case "tier":
    default:
      return list.sort((a, b) => a.tier.localeCompare(b.tier));
  }
}
