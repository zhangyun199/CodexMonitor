# Daemon Internals (Rust)

This document describes the **CodexMonitor daemon** implementation and the shared Rust backend modules it reuses.

The daemon is implemented as an extra Rust binary in the Tauri crate:

- `src-tauri/src/bin/codex_monitor_daemon.rs`

It reuses the same core backend modules as the desktop app via `#[path = "../..."] mod ...` imports.

---

## Entry point and lifecycle

### Binary entry

File: `src-tauri/src/bin/codex_monitor_daemon.rs`

Main steps:

1. Parse CLI args (`Args`):
   - `--port` (default `4732`)
   - `--bind` (default `127.0.0.1`)
   - `--data-dir` (optional)
   - `--token` or env `CODEX_MONITOR_DAEMON_TOKEN`
   - `--insecure-no-auth` (dev-only)

2. Resolve data directory:
   - If `--data-dir` specified: use it
   - Else: Linux-style default (`$XDG_DATA_HOME/codex-monitor-daemon` or `~/.local/share/codex-monitor-daemon`)
   - **Note:** production deployment on macOS uses an explicit `--data-dir` (see `docs/DEPLOYMENT.md`).

3. Load persistent state:
   - `workspaces.json` → `HashMap<WorkspaceId, WorkspaceEntry>`
   - `settings.json` → `AppSettings`

4. Start TCP listener:
   - Accept connections
   - Spawn `handle_client(...)` per connection

---

## High-level architecture

### Core state container: `DaemonState`

`DaemonState` owns:

- `workspaces: Mutex<HashMap<String, WorkspaceEntry>>`
- `app_settings: Mutex<AppSettings>`
- `sessions: Mutex<HashMap<String, WorkspaceSession>>`  
  (one live `codex app-server` session per connected workspace)
- `terminal_sessions: Mutex<HashMap<String, TerminalSession>>`
- `data_dir: PathBuf`
- `event_sink: DaemonEventSink` → broadcasts `DaemonEvent` to clients

Persistent files:
- `<data-dir>/workspaces.json`
- `<data-dir>/settings.json`

Runtime-only (in-memory):
- workspace `sessions`
- terminal sessions

### Client connections: `handle_client`

Each client connection:

- Reads newline-delimited JSON messages (`BufRead::lines()`).
- If auth is enabled:
  - requires the first successful method to be `auth`
  - otherwise returns `{"error":{"message":"unauthorized"}}`
- After auth, it subscribes to the daemon event broadcast and starts forwarding:
  - `DaemonEvent::AppServerEvent` → notification `app-server-event`
  - `DaemonEvent::TerminalOutput` → notification `terminal-output`

---

## Module structure (shared + daemon-only)

The daemon binary includes these shared modules:

| Module | Source file | Purpose |
|---|---|---|
| `backend` | `src-tauri/src/backend/mod.rs` | Shared backend building blocks |
| `codex_config` | `src-tauri/src/codex_config.rs` | Read/write `$CODEX_HOME/config.toml` feature flags |
| `codex_home` | `src-tauri/src/codex_home.rs` | Resolve CODEX_HOME / legacy `.codexmonitor` |
| `git_utils` | `src-tauri/src/git_utils.rs` | Helper utilities (find repo root, etc.) |
| `local_usage_core` | `src-tauri/src/local_usage_core.rs` | Parse Codex session JSONL logs for usage stats |
| `rules` | `src-tauri/src/rules.rs` | Read/write `$CODEX_HOME/rules/default.rules` |
| `storage` | `src-tauri/src/storage.rs` | Read/write JSON state files |
| `types` | `src-tauri/src/types.rs` | Shared serializable structs/enums |
| `utils` | `src-tauri/src/utils.rs` | Misc helpers |

And it defines significant daemon-only code inside the binary file itself:
- TCP server
- RPC dispatcher (`handle_rpc_request`)
- Git operations (libgit2 + some CLI)
- Workspace management (clone/worktree management, cleanup)
- Terminal session management glue

---

## Codex CLI integration

### Process model: one `codex app-server` per workspace

The core integration is implemented in:

- `src-tauri/src/backend/app_server.rs`

Key concepts:
- A `WorkspaceSession` represents a running app-server process tied to:
  - a `cwd` (workspace root)
  - a `codex_bin` path (optional override)

Spawn behavior:
- The backend spawns a process roughly like:

```sh
codex app-server
```

with:
- `stdin` used for requests/responses
- `stdout` emitting newline-delimited JSON notifications/events
- `stderr` piped for logging/debug

### Sending requests

The app-server protocol is treated as:
- request: `{ "id": <number>, "method": "<string>", "params": <object> }`
- response: `{ "id": <same>, "result": <any> }` or `{ "id": <same>, "error": ... }`
- notification: `{ "method": "<string>", "params": <any> }`

In the daemon:
- the daemon RPC methods translate into app-server requests (e.g. `turn/start`).
- the initial app-server response is returned to the client call.
- subsequent updates arrive as notifications and are broadcast.

### Broadcasting events to clients

The shared backend uses an `EventSink` abstraction (see `src-tauri/src/backend/events.rs`):

- In the **desktop app**, the sink emits Tauri events.
- In the **daemon**, the sink broadcasts to TCP clients.

In the daemon binary:
- `DaemonEventSink` implements `EventSink`.
- It pushes `DaemonEvent::AppServerEvent { workspace_id, message }` into a `tokio::sync::broadcast::Sender`.

---

## Event system details

### What is an AppServerEvent?

`AppServerEvent` is a thin envelope:

- `workspace_id: String`
- `message: serde_json::Value` (opaque)

Daemon clients receive it as notification:
- method: `"app-server-event"`
- params: `AppServerEvent`

The `message` payload is the raw Codex event (examples):
- `item/agentMessage/delta`
- `item/completed`
- `turn/plan/updated`
- `thread/tokenUsage/updated`
- `account/rateLimits/updated`
- `requestApproval/*`

### Client-side parsing responsibility

Neither the daemon nor the Rust backend tries to interpret event semantics.
Interpretation is done in the clients:
- Desktop: reducers/hooks under `src/features/threads/`
- iOS: `CodexStore` + `ConversationHelpers`

---

## Terminal multiplexing

### PTY implementation

Terminal sessions use `portable_pty` (see `src-tauri/src/terminal.rs` in desktop backend).

On the daemon:
- `terminal_open` spawns a PTY shell process:
  - cwd = workspace path
  - shell = `$SHELL` if set, otherwise a default
- Output is read asynchronously and broadcast as `terminal-output` notifications.

### Session identity

The *client* chooses `terminalId` in `terminal_open`.
The daemon stores sessions in:

- `terminal_sessions: HashMap<String, TerminalSession>`

All subsequent calls (`terminal_write`, `terminal_resize`, `terminal_close`) use this `terminalId`.

### Output flooding risk

Terminal output is streamed as unbounded strings. A noisy command can:
- generate a high volume of events
- pressure memory / CPU in clients

Mitigations currently rely on:
- client-side trimming of buffers
- user discipline (no explicit server-side backpressure)

---

## Git operations

### Libraries used

The daemon uses:
- `git2` (libgit2 bindings) for many operations:
  - status
  - log traversal
  - diffs
  - committing
  - branch enumeration
- Some operations may invoke git CLI depending on implementation (worktree creation, clone).

### Workspace root vs git root

Most methods take a `workspaceId`. The daemon resolves:
- workspace root directory
- git root (by walking up from workspace root)

Some workspaces can contain multiple nested repos (monorepos); see:
- RPC: `list_git_roots`

### Security / safety

Git methods operate directly on repositories on the daemon host.
Combined with token auth, they are effectively “remote git control”.

---

## Settings and configuration

### Persistent settings (`settings.json`)

The daemon persists a full `AppSettings` blob:
- backendMode (local/remote)
- codex binary override
- UI preferences (theme, scale, etc.)
- experimental feature toggles (collab/steer/unified_exec)
- workspace grouping preferences

### Codex feature flags (`$CODEX_HOME/config.toml`)

`get_app_settings` overlays feature flags from Codex config:
- `features.collab`
- `features.steer`
- `features.unified_exec`

`update_app_settings` writes them back.

### Approval rules (`$CODEX_HOME/rules/default.rules`)

When a user accepts an approval “remember” action:
- daemon `remember_approval_rule` calls `rules::append_prefix_rule(...)`

This changes what Codex can execute without prompting and is security-sensitive.

---

## Security and risk notes

- **Plain TCP; no TLS**  
  The daemon assumes you protect the port using localhost binding + Tailscale.

- **Token == full remote control**  
  With a valid token, a client can:
  - spawn shells (terminal)
  - run git operations
  - read workspace files
  - instruct Codex to run commands depending on access mode and approval rules

- **No per-client authorization**  
  There is only one token for all clients; no user identities.

- **`--insecure-no-auth` caveat**  
  In no-auth mode, `auth` is not implemented. Desktop remote mode can omit auth if token is empty, but the iOS client always performs auth and will fail to connect.

---

## File inventory (Rust backend)

Top-level Tauri crate (`src-tauri/src/*.rs`):

- `lib.rs` — command registration + app bootstrap
- `main.rs` — desktop binary entry
- `state.rs` — `AppState` + shared state orchestration
- `storage.rs` — JSON file persistence helpers
- `types.rs` — shared serializable models
- `remote_backend.rs` — desktop remote backend TCP client
- `codex.rs` — high-level Codex request builders
- `terminal.rs` — PTY sessions + event emission
- `workspaces.rs` — workspace CRUD, file listing/reading
- `git.rs`, `git_utils.rs` — git operations + helpers
- `github.rs` — GitHub operations via `gh` CLI
- `prompts.rs` — prompt library CRUD
- `rules.rs` — approval rules file handling
- `local_usage_core.rs` — usage scanning
- `dictation/*` — desktop dictation support (not used by daemon)

Backend folder (`src-tauri/src/backend/*.rs`):
- `app_server.rs` — spawn/manage Codex app-server process
- `events.rs` — event types and sink abstraction
