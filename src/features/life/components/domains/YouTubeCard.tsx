import type { YouTubeIdea, YouTubeTier } from "../../types";

const TIER_CLASS: Record<YouTubeTier, string> = {
  S: "youtube-gradient-s",
  A: "youtube-gradient-a",
  B: "youtube-gradient-b",
  C: "youtube-gradient-c",
};

type YouTubeCardProps = {
  idea: YouTubeIdea;
  viewMode: "grid" | "list";
};

export function YouTubeCard({ idea, viewMode }: YouTubeCardProps) {
  return (
    <div
      className={`youtube-card ${TIER_CLASS[idea.tier]} ${
        viewMode === "list" ? "is-list" : ""
      }`}
    >
      <div className="youtube-card-overlay">
        <div className="youtube-card-title">{idea.title}</div>
        <div className="youtube-card-meta">
          {idea.stage} • Tier {idea.tier}
        </div>
      </div>
    </div>
  );
}
