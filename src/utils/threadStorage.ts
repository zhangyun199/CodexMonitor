const STORAGE_KEY_LAST_ACTIVE_THREAD_PREFIX = "codexmonitor.lastActiveThread.";

export function getLastActiveThreadKey(workspaceId: string) {
  return `${STORAGE_KEY_LAST_ACTIVE_THREAD_PREFIX}${workspaceId}`;
}

export function loadLastActiveThread(workspaceId: string): string | null {
  if (typeof window === "undefined") {
    return null;
  }
  try {
    return window.localStorage.getItem(getLastActiveThreadKey(workspaceId));
  } catch {
    return null;
  }
}

export function saveLastActiveThread(
  workspaceId: string,
  threadId: string | null,
): void {
  if (typeof window === "undefined") {
    return;
  }
  try {
    const key = getLastActiveThreadKey(workspaceId);
    if (!threadId) {
      window.localStorage.removeItem(key);
      return;
    }
    window.localStorage.setItem(key, threadId);
  } catch {
    // Best-effort persistence.
  }
}
