## 2026-01-27

### Problem
Thread list appears empty after app restart because the Codex app-server starts with no in-memory threads and `thread/list` returns nothing.

### Files Modified
- `src-tauri/src/codex.rs` — added session JSONL scanning + new `list_session_threads` command.
- `src-tauri/src/lib.rs` — registered `list_session_threads` tauri command.
- `src/services/tauri.ts` — added `listSessionThreads` wrapper.
- `src/types.ts` — added `SessionThreadInfo` type.
- `src/features/workspaces/hooks/useWorkspaceRestore.ts` — resume session threads from disk before list.
- `src/features/workspaces/hooks/useWorkspaceRestore.test.tsx` — unit test for restore flow.
- `src/services/tauri.test.ts` — unit test for `list_session_threads` invoke.

### Details
**Before**
```ts
const summaries = await listThreadsForWorkspace(workspace);
if (!summaries.length) {
  return;
}
```

**After**
```ts
const sessionThreads = await listSessionThreads(workspace.path, 50);
await Promise.allSettled(
  sessionThreads.map((entry) =>
    resumeThreadForWorkspace(workspace.id, entry.threadId),
  ),
);
const summaries = await listThreadsForWorkspace(workspace);
```

### Status
Fixed (tests added; rebuild required to ship)

### ---
### Problem
Reasoning blocks in older threads render as unformatted white text after reopening the app, even though fresh reasoning shows styled tool blocks.

### Files Modified
- `src/utils/threadItems.ts` — normalize reasoning summary/content arrays into text lines.
- `src/utils/threadItems.test.ts` — add coverage for structured reasoning arrays.

### Details
**Before**
```ts
const summary = Array.isArray(item.summary)
  ? item.summary.map((entry) => asString(entry)).join("\n")
  : asString(item.summary ?? "");
```

**After**
```ts
const summary = extractTextLines(item.summary ?? "").join("\n");
```

### Status
Fixed (tests added; rebuild required to ship)
