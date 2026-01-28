import type { YouTubeIdea } from "../../types";
import { YouTubeCard } from "./YouTubeCard";

type YouTubeSectionProps = {
  title: string;
  count: number;
  ideas: YouTubeIdea[];
  viewMode: "grid" | "list";
};

export function YouTubeSection({
  title,
  count,
  ideas,
  viewMode,
}: YouTubeSectionProps) {
  return (
    <section className="youtube-section">
      <div className="youtube-section-header">
        <div className="youtube-section-title">
          {title} <span className="youtube-section-count">({count})</span>
        </div>
      </div>
      <div className={`youtube-grid ${viewMode === "list" ? "is-list" : ""}`}>
        {ideas.map((idea) => (
          <YouTubeCard key={idea.id} idea={idea} viewMode={viewMode} />
        ))}
      </div>
    </section>
  );
}
