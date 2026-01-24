# CodexMonitor Agent Reference

> Quick reference for AI coding assistants working on this codebase.

## Architecture (1-paragraph summary)

CodexMonitor is a multi-client UI (Desktop Tauri+React, iOS SwiftUI, and an optional remote Rust daemon) for driving **Codex `app-server`** sessions. The core backend concept is **one `codex app-server` process per workspace**, with UI state driven by streamed JSON notifications from Codex. The desktop app can run the backend locally, or proxy all calls to a daemon over TCP. iOS is a thin remote client that always talks to the daemon over newline-delimited JSON “RPC” with a shared token.

## Key paths

| What | Path |
|------|------|
| Daemon entry | `src-tauri/src/bin/codex_monitor_daemon.rs` |
| Desktop Rust backend entry | `src-tauri/src/main.rs` → `codex_monitor_lib::run()` |
| Tauri command registry | `src-tauri/src/lib.rs` |
| React entry | `src/main.tsx` |
| React root | `src/App.tsx` |
| Desktop types | `src/types.ts` |
| Shared Rust types | `src-tauri/src/types.rs` |
| Shared Swift models | `ios/Packages/CodexMonitorRPC/Sources/CodexMonitorModels/Models.swift` |
| iOS RPC client | `ios/Packages/CodexMonitorRPC/Sources/CodexMonitorRPC/RPCClient.swift` |
| iOS app store | `ios/CodexMonitorMobile/CodexMonitorMobile/CodexStore.swift` |

## RPC Quick Reference (daemon)

Full details: `docs/API_REFERENCE.md`

| Method | Purpose |
|---|---|
| `auth` | Authenticate a TCP connection (token) |
| `list_workspaces` | List known workspaces |
| `add_workspace` / `remove_workspace` | Manage workspaces |
| `connect_workspace` | Spawn/attach `codex app-server` for a workspace |
| `list_threads` | List Codex threads for a workspace |
| `start_thread` / `resume_thread` / `archive_thread` | Thread lifecycle |
| `send_user_message` | Start a Codex turn (`turn/start`) |
| `turn_interrupt` | Interrupt a running turn |
| `respond_to_server_request` | Reply to Codex request (approvals) |
| `remember_approval_rule` | Append allowed-command rule to CODEX_HOME rules |
| `get_git_status` / `get_git_diffs` / `commit_git` | Git operations |
| `terminal_open` / `terminal_write` | PTY terminal sessions |
| `prompts_list` / `prompts_create` | Prompt library CRUD |
| notifications: `app-server-event` | Streamed Codex events |
| notifications: `terminal-output` | Terminal output stream |

## Common tasks

### Add a new daemon RPC method

1. Implement it in `src-tauri/src/bin/codex_monitor_daemon.rs` inside `handle_rpc_request`:
   - Parse params with existing `parse_*` helpers (or add one).
   - If it’s Codex-backed, call `session.send_request(...)` with an app-server method string.
2. Add/extend typed wrappers:
   - Desktop remote mode: `src-tauri/src/remote_backend.rs` (method string + params)
   - iOS: `ios/Packages/CodexMonitorRPC/Sources/CodexMonitorRPC/CodexMonitorAPI.swift`
3. Update docs:
   - `docs/API_REFERENCE.md` (method entry)
4. If the method adds/changes payload types:
   - update `src/types.ts`
   - update Swift `Models.swift`
   - update Rust `types.rs`
   - update `docs/DATA_MODELS.md`

### Add a new iOS view

1. Create the view in `ios/CodexMonitorMobile/CodexMonitorMobile/Views/`.
2. Add navigation entry in `RootView` / `PhoneRootView` / `TabletRootView`.
3. Bind state via `@EnvironmentObject var store: CodexStore`.
4. If the view needs new backend behavior, add RPC methods and typed wrappers.

### Modify a shared data model

1. Decide the *wire name* of each field (camelCase vs snake_case — existing code mixes both).
2. Update:
   - TypeScript: `src/types.ts`
   - Swift: `Models.swift` (+ `CodingKeys` if wire name differs)
   - Rust: `src-tauri/src/types.rs` (`#[serde(rename="...")]` if needed)
3. Confirm daemon serialization matches clients:
   - daemon RPC payloads in `handle_rpc_request`
4. Update `docs/DATA_MODELS.md`.

## Gotchas & landmines

- **Mixed JSON naming**: some payloads are snake_case (`codex_bin`, `workspace_id`), others camelCase (`parentId`, request params). Don’t “normalize” without changing all clients.
- **Codex responses are nested**: many RPC methods return the raw app-server response envelope; clients often need `result.result`.
- **Auth vs insecure mode**: iOS always performs `auth` and will not connect to a daemon started with `--insecure-no-auth` (unless mobile is changed).
- **Terminal output is unbounded**: output streams can flood UI; trim buffers and consider backpressure if adding new streams.
- **Approval rules are security-sensitive**: `remember_approval_rule` changes CODEX_HOME rules; treat it like editing sudoers.

---
