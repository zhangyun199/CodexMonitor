import type { YouTubeIdea, YouTubeTier } from "../../types";

const TIER_CLASS: Record<YouTubeTier, string> = {
  S: "youtube-gradient-s",
  A: "youtube-gradient-a",
  B: "youtube-gradient-b",
  C: "youtube-gradient-c",
};

const STAGE_LABELS: Record<string, string> = {
  idea: "Idea",
  notes: "Notes",
  outline: "Outline",
  draft: "Draft",
  script: "Script",
  ready: "Ready",
  published: "Published",
};

type YouTubeCardProps = {
  idea: YouTubeIdea;
  viewMode: "grid" | "list";
};

export function YouTubeCard({ idea, viewMode }: YouTubeCardProps) {
  const stageLabel = STAGE_LABELS[idea.stage] ?? idea.stage;
  return (
    <article className={`youtube-card ${viewMode === "list" ? "is-list" : ""}`}>
      <div className="youtube-card__poster">
        <div className={`youtube-card__fallback ${TIER_CLASS[idea.tier]}`} />
        <span className={`youtube-card__badge youtube-card__badge--tier-${idea.tier}`}>
          {idea.tier}
        </span>
        <span className="youtube-card__badge youtube-card__badge--stage">
          {stageLabel}
        </span>
      </div>
      <div className="youtube-card__info">
        <h3 className="youtube-card__title">{idea.title}</h3>
      </div>
    </article>
  );
}
