import type { MediaItem, MediaType } from "../../types";

const TYPE_CLASS: Record<MediaType, string> = {
  Film: "media-gradient-film",
  TV: "media-gradient-tv",
  Anime: "media-gradient-anime",
  Game: "media-gradient-game",
  Book: "media-gradient-book",
  YouTube: "media-gradient-youtube",
};

type MediaCardProps = {
  item: MediaItem;
  viewMode: "grid" | "list";
};

export function MediaCard({ item, viewMode }: MediaCardProps) {
  return (
    <div
      className={`media-card ${TYPE_CLASS[item.type]} ${
        item.status === "Backlog" ? "backlog" : ""
      } ${viewMode === "list" ? "is-list" : ""}`}
    >
      <div className="media-card-overlay">
        <div className="media-card-title">{item.title}</div>
        <div className="media-card-rating">
          {item.rating ? `⭐ ${item.rating.toFixed(1)}` : "—"}
          <span className="media-card-status">{item.status}</span>
        </div>
      </div>
    </div>
  );
}
