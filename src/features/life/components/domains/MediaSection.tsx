import type { MediaItem } from "../../types";
import { MediaCard } from "./MediaCard";

type MediaSectionProps = {
  title: string;
  count: number;
  items: MediaItem[];
  viewMode: "grid" | "list";
};

export function MediaSection({ title, count, items, viewMode }: MediaSectionProps) {
  return (
    <section className="media-section">
      <div className="media-section-header">
        <div className="media-section-title">
          {title} <span className="media-section-count">({count})</span>
        </div>
      </div>
      <div className={`media-grid ${viewMode === "list" ? "is-list" : ""}`}>
        {items.map((item) => (
          <MediaCard key={item.id} item={item} viewMode={viewMode} />
        ))}
      </div>
    </section>
  );
}
