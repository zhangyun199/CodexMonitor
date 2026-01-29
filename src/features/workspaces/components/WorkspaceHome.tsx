import type { ThreadSummary, WorkspaceInfo } from "../../../types";
import { formatRelativeTimeShort } from "../../../utils/time";

type WorkspaceHomeProps = {
  workspace: WorkspaceInfo;
  threads: ThreadSummary[];
  onSelectThread: (threadId: string) => void;
};

export function WorkspaceHome({
  workspace,
  threads,
  onSelectThread,
}: WorkspaceHomeProps) {
  const recentThreads = [...threads]
    .sort((a, b) => (b.updatedAt ?? 0) - (a.updatedAt ?? 0))
    .slice(0, 8);

  return (
    <div className="workspace-home">
      <div className="workspace-home-header">
        <div className="workspace-home-title">{workspace.name}</div>
        <div className="workspace-home-subtitle">Recent Threads</div>
      </div>
      {recentThreads.length === 0 ? (
        <div className="workspace-home-empty">No recent threads yet.</div>
      ) : (
        <div className="workspace-home-list">
          {recentThreads.map((thread) => (
            <button
              key={thread.id}
              type="button"
              className="workspace-home-item"
              onClick={() => onSelectThread(thread.id)}
            >
              <div className="workspace-home-item-title">
                {thread.name || "Untitled thread"}
              </div>
              <div className="workspace-home-item-meta">
                {thread.updatedAt ? formatRelativeTimeShort(thread.updatedAt) : ""}
              </div>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
