import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import RefreshCcw from "lucide-react/dist/esm/icons/refresh-ccw";
import Plus from "lucide-react/dist/esm/icons/plus";
import Globe from "lucide-react/dist/esm/icons/globe";
import {
  browserClick,
  browserCreateSession,
  browserListSessions,
  browserNavigate,
  browserScreenshot,
} from "../../../services/tauri";

export function BrowserPanel() {
  const [sessions, setSessions] = useState<string[]>([]);
  const [selectedSession, setSelectedSession] = useState<string | null>(null);
  const [url, setUrl] = useState<string>("https://example.com");
  const [imageSrc, setImageSrc] = useState<string | null>(null);
  const [imageSize, setImageSize] = useState<{ width: number; height: number } | null>(null);
  const [loading, setLoading] = useState(false);
  const imageRef = useRef<HTMLImageElement | null>(null);

  const refreshSessions = useCallback(async () => {
    const list = await browserListSessions();
    const sessionIds = (list as { sessions?: string[] })?.sessions ?? [];
    setSessions(sessionIds);
    if (!selectedSession && sessionIds.length > 0) {
      setSelectedSession(sessionIds[0]);
    }
  }, [selectedSession]);

  const createSession = useCallback(async () => {
    setLoading(true);
    try {
      const created = (await browserCreateSession({ headless: true })) as {
        sessionId?: string;
      };
      if (created?.sessionId) {
        setSelectedSession(created.sessionId);
      }
      await refreshSessions();
    } finally {
      setLoading(false);
    }
  }, [refreshSessions]);

  const navigate = useCallback(async () => {
    if (!selectedSession) return;
    setLoading(true);
    try {
      await browserNavigate({ sessionId: selectedSession, url });
      await refreshScreenshot();
    } finally {
      setLoading(false);
    }
  }, [selectedSession, url]);

  const refreshScreenshot = useCallback(async () => {
    if (!selectedSession) return;
    setLoading(true);
    try {
      const shot = (await browserScreenshot({
        sessionId: selectedSession,
        fullPage: true,
      })) as {
        base64Png?: string;
        width?: number;
        height?: number;
      };
      if (shot?.base64Png) {
        setImageSrc(`data:image/png;base64,${shot.base64Png}`);
        if (shot.width && shot.height) {
          setImageSize({ width: shot.width, height: shot.height });
        }
      }
    } finally {
      setLoading(false);
    }
  }, [selectedSession]);

  const handleImageClick = useCallback(
    async (event: React.MouseEvent<HTMLImageElement>) => {
      if (!selectedSession || !imageRef.current || !imageSize) return;
      const rect = imageRef.current.getBoundingClientRect();
      const scaleX = imageSize.width / rect.width;
      const scaleY = imageSize.height / rect.height;
      const x = (event.clientX - rect.left) * scaleX;
      const y = (event.clientY - rect.top) * scaleY;
      await browserClick({ sessionId: selectedSession, x, y });
      await refreshScreenshot();
    },
    [imageSize, refreshScreenshot, selectedSession],
  );

  useEffect(() => {
    void refreshSessions();
  }, [refreshSessions]);

  const sessionOptions = useMemo(
    () => sessions.map((session) => (
      <option key={session} value={session}>
        {session}
      </option>
    )),
    [sessions],
  );

  return (
    <div className="memory-panel">
      <div className="memory-panel-header">
        <div className="memory-panel-title">Browser</div>
        <div className="memory-panel-actions">
          <button
            type="button"
            className="ghost icon-button"
            onClick={() => void refreshSessions()}
            title="Refresh sessions"
          >
            <RefreshCcw aria-hidden />
          </button>
          <button
            type="button"
            className="ghost icon-button"
            onClick={() => void createSession()}
            title="New session"
          >
            <Plus aria-hidden />
          </button>
        </div>
      </div>

      <div className="memory-panel-form">
        <label className="memory-panel-label">Session</label>
        <select
          className="memory-panel-select"
          value={selectedSession ?? ""}
          onChange={(event) => setSelectedSession(event.target.value)}
        >
          <option value="">Select session</option>
          {sessionOptions}
        </select>
      </div>

      <div className="memory-panel-form">
        <label className="memory-panel-label">URL</label>
        <div className="memory-panel-row">
          <input
            className="memory-panel-input"
            value={url}
            onChange={(event) => setUrl(event.target.value)}
          />
          <button type="button" className="ghost" onClick={() => void navigate()}>
            <Globe aria-hidden />
            Go
          </button>
          <button type="button" className="ghost" onClick={() => void refreshScreenshot()}>
            Refresh
          </button>
        </div>
      </div>

      <div className="memory-panel-results">
        {loading && <div className="memory-panel-status">Loading…</div>}
        {!imageSrc && !loading && (
          <div className="memory-panel-status">No screenshot yet.</div>
        )}
        {imageSrc && (
          <img
            ref={imageRef}
            src={imageSrc}
            alt="Browser screenshot"
            onClick={(event) => void handleImageClick(event)}
            style={{ width: "100%", borderRadius: 12, cursor: "crosshair" }}
          />
        )}
      </div>
    </div>
  );
}
