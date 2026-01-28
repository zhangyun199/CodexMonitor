// @vitest-environment jsdom
import { act, renderHook } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { SessionThreadInfo, ThreadSummary, WorkspaceInfo } from "../../../types";
import { useWorkspaceRestore } from "./useWorkspaceRestore";

const workspace: WorkspaceInfo = {
  id: "ws-1",
  name: "project",
  path: "/tmp/project",
  connected: true,
  codex_bin: null,
  kind: "main",
  parentId: null,
  worktree: null,
  settings: { sidebarCollapsed: false },
};

describe("useWorkspaceRestore", () => {
  it("resumes session threads from disk before listing threads", async () => {
    const connectWorkspace = vi.fn();
    const listSessionThreads = vi
      .fn()
      .mockResolvedValue([
        { threadId: "thread-a", cwd: "/tmp/project", updatedAt: 2000 },
        { threadId: "thread-b", cwd: "/tmp/project", updatedAt: 1000 },
      ]) as unknown as ((
      workspacePath: string,
      limit?: number,
    ) => Promise<SessionThreadInfo[]>);
    const listThreadsForWorkspace = vi
      .fn()
      .mockResolvedValue([
        { id: "thread-a", name: "A", updatedAt: 2000 },
        { id: "thread-b", name: "B", updatedAt: 1000 },
      ]) as unknown as ((workspace: WorkspaceInfo) => Promise<ThreadSummary[]>);
    const resumeThreadForWorkspace = vi
      .fn()
      .mockResolvedValue("thread-a") as unknown as ((
      workspaceId: string,
      threadId: string,
    ) => Promise<string | null>);
    const setActiveThreadId = vi.fn();

    window.localStorage.setItem(
      "codexmonitor.lastActiveThread.ws-1",
      "thread-a",
    );

    renderHook(() =>
      useWorkspaceRestore({
        workspaces: [workspace],
        hasLoaded: true,
        connectWorkspace,
        listSessionThreads,
        listThreadsForWorkspace,
        resumeThreadForWorkspace,
        setActiveThreadId,
      }),
    );

    await act(async () => {
      await new Promise((resolve) => setTimeout(resolve, 0));
    });

    expect(listSessionThreads).toHaveBeenCalledWith("/tmp/project", 50);
    expect(resumeThreadForWorkspace).toHaveBeenCalledWith("ws-1", "thread-a");
    expect(resumeThreadForWorkspace).toHaveBeenCalledWith("ws-1", "thread-b");
    expect(listThreadsForWorkspace).toHaveBeenCalledWith(workspace);
    expect(setActiveThreadId).toHaveBeenCalledWith("thread-a", "ws-1");
  });
});
