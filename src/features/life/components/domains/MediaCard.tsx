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
  const statusIcon = item.status === "Completed" ? "✓" : "◎";
  return (
    <article className={`media-card ${viewMode === "list" ? "is-list" : ""}`}>
      <div className="media-card__poster">
        {item.coverUrl ? (
          <img src={item.coverUrl} alt={item.title} loading="lazy" />
        ) : (
          <div className={`media-card__fallback ${TYPE_CLASS[item.type]}`}>
            <span className="media-card__fallback-title">{item.title}</span>
          </div>
        )}
        <span
          className={`media-card__badge media-card__badge--status-${item.status}`}
        >
          {statusIcon}
        </span>
        {item.rating != null ? (
          <span className="media-card__badge media-card__badge--rating">
            {item.rating.toFixed(1)}
          </span>
        ) : null}
      </div>
      <div className="media-card__info">
        <h3 className="media-card__title">{item.title}</h3>
        <span className="media-card__type">{item.type}</span>
      </div>
    </article>
  );
}
