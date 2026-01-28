import { useEffect, useRef } from "react";
import type { SessionThreadInfo, ThreadSummary, WorkspaceInfo } from "../../../types";
import { loadLastActiveThread } from "../../../utils/threadStorage";

type WorkspaceRestoreOptions = {
  workspaces: WorkspaceInfo[];
  hasLoaded: boolean;
  connectWorkspace: (workspace: WorkspaceInfo) => Promise<void>;
  listSessionThreads: (
    workspacePath: string,
    limit?: number,
  ) => Promise<SessionThreadInfo[]>;
  listThreadsForWorkspace: (workspace: WorkspaceInfo) => Promise<ThreadSummary[]>;
  resumeThreadForWorkspace: (
    workspaceId: string,
    threadId: string,
  ) => Promise<string | null>;
  setActiveThreadId: (threadId: string | null, workspaceId?: string) => void;
};

export function useWorkspaceRestore({
  workspaces,
  hasLoaded,
  connectWorkspace,
  listSessionThreads,
  listThreadsForWorkspace,
  resumeThreadForWorkspace,
  setActiveThreadId,
}: WorkspaceRestoreOptions) {
  const restoredWorkspaces = useRef(new Set<string>());
  const maxAutoResume = 3;
  const maxSessionResume = 50;

  useEffect(() => {
    if (!hasLoaded) {
      return;
    }
    workspaces.forEach((workspace) => {
      if (restoredWorkspaces.current.has(workspace.id)) {
        return;
      }
      restoredWorkspaces.current.add(workspace.id);
      void (async () => {
        try {
          if (!workspace.connected) {
            await connectWorkspace(workspace);
          }
          const sessionThreads = await listSessionThreads(
            workspace.path,
            maxSessionResume,
          );
          const uniqueSessionIds = Array.from(
            new Set(sessionThreads.map((entry) => entry.threadId)),
          );
          if (uniqueSessionIds.length > 0) {
            await Promise.allSettled(
              uniqueSessionIds.map((threadId) =>
                resumeThreadForWorkspace(workspace.id, threadId),
              ),
            );
          }
          const summaries = await listThreadsForWorkspace(workspace);
          if (!summaries.length) {
            return;
          }
          const sorted = [...summaries].sort(
            (a, b) => b.updatedAt - a.updatedAt,
          );
          const storedThreadId = loadLastActiveThread(workspace.id);
          const storedMatch = storedThreadId
            ? sorted.find((thread) => thread.id === storedThreadId)?.id ?? null
            : null;
          if (storedMatch) {
            await resumeThreadForWorkspace(workspace.id, storedMatch);
          }
          const recentThreadIds = sorted
            .slice(0, maxAutoResume)
            .map((thread) => thread.id)
            .filter(Boolean)
            .filter((threadId) => threadId !== storedMatch);
          if (recentThreadIds.length > 0) {
            await Promise.allSettled(
              recentThreadIds.map((threadId) =>
                resumeThreadForWorkspace(workspace.id, threadId),
              ),
            );
          }
          const activeId = storedMatch ?? sorted[0]?.id;
          if (activeId) {
            setActiveThreadId(activeId, workspace.id);
          }
        } catch {
          // Silent: connection errors show in debug panel.
        }
      })();
    });
  }, [
    connectWorkspace,
    hasLoaded,
    listSessionThreads,
    listThreadsForWorkspace,
    resumeThreadForWorkspace,
    setActiveThreadId,
    workspaces,
  ]);
}
