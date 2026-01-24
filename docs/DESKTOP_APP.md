# Desktop App (Tauri + React)

This document describes the **desktop CodexMonitor application**.

The desktop app is split into:
- **React frontend** (`src/`)
- **Tauri Rust backend** (`src-tauri/src/`)

The Rust backend can run in two modes:
- **Local backend mode** (default): desktop app spawns `codex app-server` locally.
- **Remote backend mode**: desktop app proxies requests to a remote daemon over TCP (`src-tauri/src/remote_backend.rs`).

Key paths:
- React entry: `src/main.tsx`
- React root: `src/App.tsx`
- Tauri backend entry: `src-tauri/src/main.rs` → `codex_monitor_lib::run()`
- Tauri command registry: `src-tauri/src/lib.rs`

---

## React component tree (high level)

Entry:
- `src/main.tsx` mounts `<App />`.

Top level:
- `src/App.tsx`
  - Bootstraps app settings + backend mode
  - Wires together feature hooks (workspaces, threads, git, prompts, terminal, dictation)
  - Registers event listeners (app-server events, terminal output, dictation events)
  - Renders `AppLayout`

Layout:
- `src/features/app/components/AppLayout.tsx`
  - Chooses layout based on viewport/device profile:
    - `DesktopLayout`
    - `TabletLayout`
    - `PhoneLayout`

Conversation surface (core user journey):
- Workspace selection (sidebar / project list)
- Thread selection (threads list)
- Conversation view (items + plan)
- Composer (text + options + attachments)
- Optional side panels (Git, Prompts, Terminal)

The exact composition varies per layout, but those are the core conceptual nodes.

---

## Frontend module structure (`src/features/*`)

The frontend is organized by feature folders under `src/features/`:

| Feature | Folder | Responsibilities |
|---|---|---|
| app | `src/features/app/` | Layout shell, app-level event routing (`useAppServerEvents`) |
| workspaces | `src/features/workspaces/` | Workspace list, connect/add/remove, workspace settings |
| threads | `src/features/threads/` | Thread list, thread lifecycle, items store, approvals |
| composer | `src/features/composer/` | Message composer UI, model/effort/accessMode selection |
| git | `src/features/git/` | Status/diffs/log/branches, staging/commit/push/pull |
| github | `src/features/github/` | PR/issue panels via daemon/backend gh integration |
| prompts | `src/features/prompts/` | Prompt library UI, CRUD + move between scopes |
| terminal | `src/features/terminal/` | Terminal tabs, PTY output rendering |
| dictation | `src/features/dictation/` | Desktop-only audio dictation + transcription |
| settings | `src/features/settings/` | Settings UI (backend mode, paths, UI scale) |
| history | `src/features/history/` | Local history and last-activity caches |
| timeline | `src/features/timeline/` | Timeline visualization of turns/items |
| toasts | `src/features/toasts/` | Toast notification UI |
| debug | `src/features/debug/` | Debug panels/logging |
| navigation | `src/features/navigation/` | Routing + navigation helpers |

---

## State management approach

The app does not use a single monolithic global store; it composes state from:
- feature-level hooks (`useWorkspaces`, `useThreads`, etc.)
- reducers for complex state (threads/items)
- local `useState` for UI state
- `localStorage` for “sticky” UX settings (pinned threads, panel sizes)

Common persistence keys:
- `codexmonitor.threadLastUserActivity`
- `codexmonitor.pinnedThreads`

See:
- `src/features/history/utils/localStorage.ts`

---

## Key hooks (agent-focused)

### `useWorkspaces`

Path: `src/features/workspaces/hooks/useWorkspaces.ts`

Responsibilities:
- Fetch list of workspaces from backend (`list_workspaces`)
- Add/remove workspaces (`add_workspace`, `remove_workspace`)
- Connect workspace sessions (`connect_workspace`)
- Worktree operations (`add_worktree`, `remove_worktree`, `rename_worktree`)
- Update workspace settings (`update_workspace_settings`)
- Track “active workspace” selection for the UI

### `useThreads`

Path: `src/features/threads/hooks/useThreads.ts`

Responsibilities:
- Maintain the threads list for the active workspace
- Start/resume/archive threads
- Maintain a local cache of conversation items per thread
- Integrate with app-server streaming events:
  - append deltas (`item/*Delta`)
  - mark completion (`item/completed`, `turn/completed`)
  - update plans (`turn/plan/updated`)
  - handle approvals (`*requestApproval*` methods)
- Persist per-thread “last activity” timestamps

### `useAppServerEvents`

Path: `src/features/app/hooks/useAppServerEvents.ts`

Responsibilities:
- Listen to backend-emitted `app-server-event` payloads
- Normalize the raw message into:
  - `AppServerMessage`: `{ method, params }` from Codex
- Dispatch to registered callbacks by method string

This hook is the main bridge between:
- raw Codex app-server protocol
- the rest of the React state machine

### Git hooks

- `src/features/git/hooks/useGitStatus.ts` — status polling, staged/unstaged lists
- `src/features/git/hooks/useGitDiffs.ts` — file diff list + selection
- `src/features/git/hooks/useGitLog.ts` — commit history view
- `src/features/git/hooks/useGitBranches.ts` — branch picker/create/checkout

Each wraps a thin Tauri command in a React state machine with loading/error state.

### Terminal hooks

- `src/features/terminal/hooks/useTerminalSessions.ts` — opens/closes sessions and binds output events
- `src/features/terminal/hooks/useTerminalOutput.ts` — accumulation & trimming of output buffers

---

## Tauri IPC

### How React calls Rust

React uses wrapper functions in:

- `src/services/tauri.ts`

Example pattern:

```ts
import { invoke } from "@tauri-apps/api/core";

export async function listWorkspaces() {
  return invoke("list_workspaces");
}
```

### How Rust exposes commands

Rust commands are registered in:

- `src-tauri/src/lib.rs` (`tauri::generate_handler![ ... ]`)

Each command is implemented in a feature module (settings/workspaces/codex/git/etc.).

---

## All Tauri commands (desktop)

These are the commands registered in `src-tauri/src/lib.rs`.

> Not all of these are available when running against a remote daemon; see “Remote backend mode” below.

| Tauri command (JS invoke) | Rust handler | Notes |
|---|---|---|
| `get_app_settings` | `settings::get_app_settings` | |
| `update_app_settings` | `settings::update_app_settings` | |
| `menu_set_accelerators` | `menu::menu_set_accelerators` | |
| `codex_doctor` | `codex::codex_doctor` | |
| `list_workspaces` | `workspaces::list_workspaces` | |
| `is_workspace_path_dir` | `workspaces::is_workspace_path_dir` | |
| `add_workspace` | `workspaces::add_workspace` | |
| `add_clone` | `workspaces::add_clone` | |
| `add_worktree` | `workspaces::add_worktree` | |
| `remove_workspace` | `workspaces::remove_workspace` | |
| `remove_worktree` | `workspaces::remove_worktree` | |
| `rename_worktree` | `workspaces::rename_worktree` | |
| `rename_worktree_upstream` | `workspaces::rename_worktree_upstream` | |
| `apply_worktree_changes` | `workspaces::apply_worktree_changes` | |
| `update_workspace_settings` | `workspaces::update_workspace_settings` | |
| `update_workspace_codex_bin` | `workspaces::update_workspace_codex_bin` | |
| `start_thread` | `codex::start_thread` | |
| `send_user_message` | `codex::send_user_message` | |
| `turn_interrupt` | `codex::turn_interrupt` | |
| `start_review` | `codex::start_review` | |
| `respond_to_server_request` | `codex::respond_to_server_request` | |
| `remember_approval_rule` | `codex::remember_approval_rule` | |
| `get_commit_message_prompt` | `codex::get_commit_message_prompt` | |
| `generate_commit_message` | `codex::generate_commit_message` | |
| `resume_thread` | `codex::resume_thread` | |
| `list_threads` | `codex::list_threads` | |
| `archive_thread` | `codex::archive_thread` | |
| `collaboration_mode_list` | `codex::collaboration_mode_list` | |
| `connect_workspace` | `workspaces::connect_workspace` | |
| `get_git_status` | `git::get_git_status` | |
| `list_git_roots` | `git::list_git_roots` | |
| `get_git_diffs` | `git::get_git_diffs` | |
| `get_git_log` | `git::get_git_log` | |
| `get_git_commit_diff` | `git::get_git_commit_diff` | |
| `get_git_remote` | `git::get_git_remote` | |
| `stage_git_file` | `git::stage_git_file` | |
| `stage_git_all` | `git::stage_git_all` | |
| `unstage_git_file` | `git::unstage_git_file` | |
| `revert_git_file` | `git::revert_git_file` | |
| `revert_git_all` | `git::revert_git_all` | |
| `commit_git` | `git::commit_git` | |
| `push_git` | `git::push_git` | |
| `pull_git` | `git::pull_git` | |
| `sync_git` | `git::sync_git` | |
| `get_github_issues` | `git::get_github_issues` | |
| `get_github_pull_requests` | `git::get_github_pull_requests` | |
| `get_github_pull_request_diff` | `git::get_github_pull_request_diff` | |
| `get_github_pull_request_comments` | `git::get_github_pull_request_comments` | |
| `list_workspace_files` | `workspaces::list_workspace_files` | |
| `read_workspace_file` | `workspaces::read_workspace_file` | |
| `open_workspace_in` | `workspaces::open_workspace_in` | |
| `list_git_branches` | `git::list_git_branches` | |
| `checkout_git_branch` | `git::checkout_git_branch` | |
| `create_git_branch` | `git::create_git_branch` | |
| `model_list` | `codex::model_list` | |
| `account_rate_limits` | `codex::account_rate_limits` | |
| `skills_list` | `codex::skills_list` | |
| `prompts_list` | `prompts::prompts_list` | |
| `prompts_create` | `prompts::prompts_create` | |
| `prompts_update` | `prompts::prompts_update` | |
| `prompts_delete` | `prompts::prompts_delete` | |
| `prompts_move` | `prompts::prompts_move` | |
| `prompts_workspace_dir` | `prompts::prompts_workspace_dir` | |
| `prompts_global_dir` | `prompts::prompts_global_dir` | |
| `terminal_open` | `terminal::terminal_open` | |
| `terminal_write` | `terminal::terminal_write` | |
| `terminal_resize` | `terminal::terminal_resize` | |
| `terminal_close` | `terminal::terminal_close` | |
| `dictation_model_status` | `dictation::dictation_model_status` | |
| `dictation_download_model` | `dictation::dictation_download_model` | |
| `dictation_cancel_download` | `dictation::dictation_cancel_download` | |
| `dictation_remove_model` | `dictation::dictation_remove_model` | |
| `dictation_start` | `dictation::dictation_start` | |
| `dictation_stop` | `dictation::dictation_stop` | |
| `dictation_cancel` | `dictation::dictation_cancel` | |
| `local_usage_snapshot` | `local_usage::local_usage_snapshot` | |

---

## Backend-emitted events (desktop)

The Rust backend emits events to the React frontend:

| Event name | Payload | Source |
|---|---|---|
| `app-server-event` | `AppServerEvent` | forwarded from Codex app-server (`src-tauri/src/backend/events.rs`) |
| `terminal-output` | `TerminalOutput` | PTY output (local or remote) |
| `dictation-download` | progress/status | dictation model download (`src-tauri/src/dictation/`) |
| `dictation-event` | transcription chunks | dictation runtime |

React subscribes via:
- `src/services/events.ts`

---

## Remote backend mode (Desktop → Daemon)

When `AppSettings.backendMode == "remote"` (see `src/types.ts` and backend settings UI), the desktop backend routes almost all stateful operations through:

- `src-tauri/src/remote_backend.rs`

Mechanics:
1. Tauri backend opens a TCP connection to `remoteBackendHost`
2. If a token is set, it calls daemon `auth`
3. For each command, it sends `{id, method, params}`
4. It forwards daemon notifications to the UI as normal Tauri events:
   - daemon `app-server-event` → Tauri `app-server-event`
   - daemon `terminal-output` → Tauri `terminal-output`

### Desktop-only commands

Some commands are inherently local-only (they manipulate local OS UI, audio, menus):

- `open_workspace_in`
- `menu_set_accelerators`
- dictation commands (`dictation_*`)

In remote mode, these are either:
- still executed locally (if meaningful), or
- disabled / return errors (if they require local OS integration on the remote host)

---

## Common gotchas & landmines (desktop)

- **Codex-backed commands return nested envelopes**  
  Many `codex` commands return the *raw app-server response* (with its own `result`), so JS often needs to read `response.result` (see `useThreads`).

- **Remote vs local type drift**  
  The daemon API uses a subset/superset of desktop commands. If you add a new command, decide whether it must be supported remotely too.

- **Event ordering assumptions**  
  UI logic often assumes `item/started` happens before deltas and `item/completed` eventually arrives. Network retries can cause missed updates; ensure reducers are defensive.

- **Terminal output can be large**  
  Both desktop and daemon stream terminal output as raw strings. Without trimming, memory can grow.

