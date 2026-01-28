# CodexMonitor Domains Feature Completion Plan

## Executive Summary

The domains feature is ~60% complete. The core backend infrastructure works, but **thread-level system prompt injection is missing**, and **iOS lacks CRUD UI**. This plan details exactly what to build.

---

## Current State

### What's Working
| Component | Status | Notes |
|-----------|--------|-------|
| Domain CRUD (Rust) | ✅ | `domains.rs` - create, read, update, delete |
| Domain persistence | ✅ | `storage.rs` - saves to `domains.json` |
| Tauri commands | ✅ | All registered in `lib.rs` |
| React hooks | ✅ | `useDomains.ts`, `useDomainDashboard.ts` |
| Desktop settings UI | ✅ | Domain cards with edit forms |
| Workspace→Domain assignment | ✅ | Dropdown in settings |
| DomainPanel (trends) | ✅ | Shows cards/lists for 7d/30d/lifetime |
| Obsidian trends parsing | ✅ | Computes stats from vault |
| iOS API layer | ✅ | `CodexMonitorAPI.swift` has all methods |
| iOS DomainDashboardView | ✅ | Read-only trends display |

### What's Broken/Missing
| Component | Status | Impact |
|-----------|--------|--------|
| `thread/start` injection | ❌ | Domain prompt NOT applied to new threads |
| iOS CodexStore wrappers | ❌ | Can't call CRUD from iOS views |
| iOS DomainListView | ❌ | No way to manage domains on iOS |
| iOS CreateDomainSheet | ❌ | No way to create domains on iOS |
| iOS domain picker | ❌ | Can't assign domain to workspace on iOS |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER FLOW                               │
├─────────────────────────────────────────────────────────────────┤
│  1. User creates Domain (name, systemPrompt, theme, defaults)   │
│  2. User assigns Domain to Workspace via settings               │
│  3. User starts new Thread in that Workspace                    │
│  4. Thread/start injects domain.systemPrompt ← MISSING!         │
│  5. All messages in thread have domain context                  │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow
```
Domain (stored in domains.json)
    │
    ▼
Workspace.settings.domainId (links workspace to domain)
    │
    ▼
thread/start OR turn/start (should inject systemPrompt)
    │
    ▼
Codex API receives instructionInjection parameter
    │
    ▼
Claude sees domain context in system message
```

---

## Phase 1: Critical Fix - Thread Start Injection

**Priority: P0 - This is why domains don't work**

### Problem
When a new thread starts, the domain's `systemPrompt` is NOT passed to Codex. Currently:
- `turn/start` (per-message) DOES inject via `instructionInjection` ✅
- `thread/start` (new thread) does NOT inject ❌

### Files to Modify

#### 1. `src-tauri/src/bin/codex_monitor_daemon.rs`

**Location:** Lines 1022-1029 (`start_thread` function)

**Current code:**
```rust
async fn start_thread(&self, workspace_id: String) -> Result<Value, String> {
    let session = self.get_session(&workspace_id).await?;
    let params = json!({
        "cwd": session.entry.path,
        "approvalPolicy": "on-request"
    });
    session.send_request("thread/start", params).await
}
```

**Change to:**
```rust
async fn start_thread(&self, workspace_id: String) -> Result<Value, String> {
    let session = self.get_session(&workspace_id).await?;

    // Resolve domain from workspace settings
    let domain_instructions = {
        let domains = self.domains.lock().await;
        let apply = session.entry.settings.apply_domain_instructions.unwrap_or(true);

        if apply {
            session.entry.settings.domain_id.as_ref()
                .and_then(|domain_id| domains.iter().find(|d| &d.id == domain_id))
                .map(|domain| domain.system_prompt.clone())
                .filter(|prompt| !prompt.is_empty())
        } else {
            None
        }
    };

    // Build params with domain context
    let mut params = json!({
        "cwd": session.entry.path,
    });

    // Inject domain system prompt if available
    if let Some(instructions) = domain_instructions {
        params["instructionInjection"] = json!(instructions);
    }

    // Apply domain defaults or fallback
    let approval_policy = {
        let domains = self.domains.lock().await;
        session.entry.settings.domain_id.as_ref()
            .and_then(|id| domains.iter().find(|d| &d.id == id))
            .and_then(|d| d.default_approval_policy.clone())
            .unwrap_or_else(|| "on-request".to_string())
    };
    params["approvalPolicy"] = json!(approval_policy);

    session.send_request("thread/start", params).await
}
```

#### 2. `src-tauri/src/codex.rs`

**Location:** Lines 157-184 (`start_thread` function - desktop variant)

Apply same pattern as daemon. Look up workspace → domain → inject `instructionInjection`.

### Verification
After fix:
1. Create a domain with system prompt: "You are a delivery optimization assistant"
2. Assign domain to workspace
3. Start new thread
4. First message should show Claude acknowledging the context

---

## Phase 2: iOS CRUD Implementation

**Priority: P1 - Enables mobile domain management**

### 2.1 CodexStore Additions

**File:** `ios/CodexMonitorMobile/CodexMonitorMobile/CodexStore.swift`

**Add after line 227:**
```swift
// MARK: - Domain CRUD

func createDomain(_ domain: Domain) async {
    do {
        let created = try await api.domainsCreate(domain)
        domains.append(created)
    } catch {
        lastError = error.localizedDescription
    }
}

func updateDomain(_ domain: Domain) async {
    do {
        let updated = try await api.domainsUpdate(domain)
        if let index = domains.firstIndex(where: { $0.id == updated.id }) {
            domains[index] = updated
        }
    } catch {
        lastError = error.localizedDescription
    }
}

func deleteDomain(_ domainId: String) async {
    do {
        try await api.domainsDelete(domainId)
        domains.removeAll { $0.id == domainId }
    } catch {
        lastError = error.localizedDescription
    }
}

func assignDomainToWorkspace(_ workspaceId: String, domainId: String?) async {
    guard var workspace = workspaces.first(where: { $0.id == workspaceId }) else { return }
    workspace.settings.domainId = domainId
    do {
        let updated = try await api.updateWorkspaceSettings(workspaceId, workspace.settings)
        if let index = workspaces.firstIndex(where: { $0.id == workspaceId }) {
            workspaces[index].settings = updated
        }
    } catch {
        lastError = error.localizedDescription
    }
}
```

### 2.2 DomainListView.swift (New File)

**File:** `ios/CodexMonitorMobile/CodexMonitorMobile/Views/DomainListView.swift`

```swift
import SwiftUI

struct DomainListView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var showCreateSheet = false
    @State private var editingDomain: Domain?

    var body: some View {
        List {
            ForEach(store.domains) { domain in
                DomainRow(domain: domain)
                    .contentShape(Rectangle())
                    .onTapGesture {
                        editingDomain = domain
                    }
                    .contextMenu {
                        Button("Edit") { editingDomain = domain }
                        Button("Delete", role: .destructive) {
                            Task { await store.deleteDomain(domain.id) }
                        }
                    }
            }
        }
        .navigationTitle("Domains")
        .toolbar {
            ToolbarItem(placement: .primaryAction) {
                Button {
                    showCreateSheet = true
                } label: {
                    Image(systemName: "plus")
                }
            }
            ToolbarItem(placement: .navigationBarLeading) {
                Button {
                    Task { await store.refreshDomains() }
                } label: {
                    Image(systemName: "arrow.clockwise")
                }
            }
        }
        .sheet(isPresented: $showCreateSheet) {
            DomainFormSheet(mode: .create)
        }
        .sheet(item: $editingDomain) { domain in
            DomainFormSheet(mode: .edit(domain))
        }
        .task {
            await store.refreshDomains()
        }
    }
}

struct DomainRow: View {
    let domain: Domain

    var body: some View {
        HStack(spacing: 12) {
            Text(domain.theme.icon)
                .font(.title2)

            VStack(alignment: .leading, spacing: 2) {
                Text(domain.name)
                    .font(.headline)
                if let desc = domain.description, !desc.isEmpty {
                    Text(desc)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                        .lineLimit(1)
                }
            }

            Spacer()

            Circle()
                .fill(Color(hex: domain.theme.color) ?? .purple)
                .frame(width: 12, height: 12)
        }
        .padding(.vertical, 4)
    }
}
```

### 2.3 DomainFormSheet.swift (New File)

**File:** `ios/CodexMonitorMobile/CodexMonitorMobile/Views/DomainFormSheet.swift`

```swift
import SwiftUI

enum DomainFormMode: Identifiable {
    case create
    case edit(Domain)

    var id: String {
        switch self {
        case .create: return "create"
        case .edit(let d): return d.id
        }
    }
}

struct DomainFormSheet: View {
    @EnvironmentObject private var store: CodexStore
    @Environment(\.dismiss) private var dismiss

    let mode: DomainFormMode

    @State private var name: String = ""
    @State private var description: String = ""
    @State private var systemPrompt: String = ""
    @State private var icon: String = "✨"
    @State private var color: String = "#7c3aed"
    @State private var viewType: String = "dashboard"

    private var isEditing: Bool {
        if case .edit = mode { return true }
        return false
    }

    private var title: String {
        isEditing ? "Edit Domain" : "New Domain"
    }

    var body: some View {
        NavigationStack {
            Form {
                Section("Basic Info") {
                    TextField("Name", text: $name)
                    TextField("Description", text: $description)
                    TextField("Icon (emoji)", text: $icon)
                    TextField("Color (hex)", text: $color)
                }

                Section("System Prompt") {
                    TextEditor(text: $systemPrompt)
                        .frame(minHeight: 150)
                }

                Section("View Type") {
                    Picker("Type", selection: $viewType) {
                        Text("Dashboard").tag("dashboard")
                        Text("Chat").tag("chat")
                    }
                    .pickerStyle(.segmented)
                }
            }
            .navigationTitle(title)
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Save") {
                        Task { await save() }
                    }
                    .disabled(name.isEmpty)
                }
            }
            .onAppear {
                if case .edit(let domain) = mode {
                    name = domain.name
                    description = domain.description ?? ""
                    systemPrompt = domain.systemPrompt
                    icon = domain.theme.icon
                    color = domain.theme.color
                    viewType = domain.viewType
                }
            }
        }
    }

    private func save() async {
        let theme = DomainTheme(
            icon: icon,
            color: color,
            accent: color,
            background: nil
        )

        switch mode {
        case .create:
            let domain = Domain(
                id: "",  // Backend assigns ID
                name: name,
                description: description.isEmpty ? nil : description,
                systemPrompt: systemPrompt,
                viewType: viewType,
                theme: theme,
                defaultModel: nil,
                defaultAccessMode: nil,
                defaultReasoningEffort: nil,
                defaultApprovalPolicy: nil
            )
            await store.createDomain(domain)

        case .edit(let existing):
            var updated = existing
            updated.name = name
            updated.description = description.isEmpty ? nil : description
            updated.systemPrompt = systemPrompt
            updated.viewType = viewType
            updated.theme = theme
            await store.updateDomain(updated)
        }

        dismiss()
    }
}
```

### 2.4 Add Domain Picker to Workspace Settings

**File:** `ios/CodexMonitorMobile/CodexMonitorMobile/Views/SettingsView.swift`

Add a domain picker section for each workspace:

```swift
// In workspace settings section, add:
Section("Domain") {
    Picker("Assigned Domain", selection: domainBinding(for: workspace)) {
        Text("None").tag(String?.none)
        ForEach(store.domains) { domain in
            HStack {
                Text(domain.theme.icon)
                Text(domain.name)
            }
            .tag(Optional(domain.id))
        }
    }
}

// Helper binding:
private func domainBinding(for workspace: Workspace) -> Binding<String?> {
    Binding(
        get: { workspace.settings.domainId },
        set: { newId in
            Task {
                await store.assignDomainToWorkspace(workspace.id, domainId: newId)
            }
        }
    )
}
```

### 2.5 Add to Navigation

**File:** `ios/CodexMonitorMobile/CodexMonitorMobile/Views/RootView.swift`

Add DomainListView to settings or as a separate tab on iPad.

---

## Phase 3: Polish & Defaults

**Priority: P2 - Improve UX**

### 3.1 Seed Default Domains on First Launch

**File:** `src-tauri/src/storage.rs`

The seeding logic exists but relies on files at `/Users/jmwillis/Desktop/workspace-*.md`.

**Change to hardcoded defaults if files don't exist:**

```rust
pub fn seed_default_domains() -> Vec<Domain> {
    vec![
        Domain {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Delivery & Finance".to_string(),
            description: Some("Track earnings, orders, and financial goals".to_string()),
            system_prompt: "You are a delivery optimization and finance tracking assistant. Help analyze earnings, suggest efficient routes, and track financial progress.".to_string(),
            view_type: "dashboard".to_string(),
            theme: DomainTheme {
                icon: "🚗".to_string(),
                color: "#22c55e".to_string(),
                accent: "#22c55e".to_string(),
                background: None,
            },
            ..Default::default()
        },
        Domain {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Nutrition & Fitness".to_string(),
            description: Some("Track meals, workouts, and health goals".to_string()),
            system_prompt: "You are a nutrition and fitness coach. Help track meals, suggest healthy options, and monitor workout progress toward weight goals.".to_string(),
            view_type: "dashboard".to_string(),
            theme: DomainTheme {
                icon: "🏋️".to_string(),
                color: "#f97316".to_string(),
                accent: "#f97316".to_string(),
                background: None,
            },
            ..Default::default()
        },
        Domain {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Media".to_string(),
            description: Some("Track movies, shows, games, and books".to_string()),
            system_prompt: "You are a media tracking assistant. Help log and rate movies, TV shows, games, and books. Provide recommendations based on preferences.".to_string(),
            view_type: "dashboard".to_string(),
            theme: DomainTheme {
                icon: "🎬".to_string(),
                color: "#8b5cf6".to_string(),
                accent: "#8b5cf6".to_string(),
                background: None,
            },
            ..Default::default()
        },
        Domain {
            id: uuid::Uuid::new_v4().to_string(),
            name: "YouTube".to_string(),
            description: Some("Manage video ideas and content pipeline".to_string()),
            system_prompt: "You are a YouTube content strategist. Help brainstorm video ideas, develop outlines, write scripts, and track content through the production pipeline.".to_string(),
            view_type: "dashboard".to_string(),
            theme: DomainTheme {
                icon: "🎥".to_string(),
                color: "#ef4444".to_string(),
                accent: "#ef4444".to_string(),
                background: None,
            },
            ..Default::default()
        },
    ]
}
```

### 3.2 Show Domain Indicator in Thread Header

When a thread has a domain assigned, show the domain icon/name in the thread header so user knows context is active.

**File:** `src/features/messages/components/Messages.tsx`

Add domain badge near thread title.

---

## Phase 4: Future Enhancements (P3)

These are nice-to-haves for later:

1. **Domain defaults enforcement** - Apply `defaultModel`, `defaultAccessMode`, etc. when starting threads
2. **Chart rendering** - `TrendSeries` data exists but isn't rendered
3. **Chat view type** - Only "dashboard" works, implement "chat" variant
4. **Domain switching mid-thread** - Allow changing domain context
5. **Domain templates** - Pre-built prompts for common use cases

---

## File Reference

### Key Files to Modify

| File | Changes |
|------|---------|
| `src-tauri/src/bin/codex_monitor_daemon.rs:1022-1029` | Inject domain in `start_thread` |
| `src-tauri/src/codex.rs:157-184` | Same for desktop variant |
| `ios/.../CodexStore.swift` | Add CRUD wrappers |
| `ios/.../Views/DomainListView.swift` | NEW - List domains |
| `ios/.../Views/DomainFormSheet.swift` | NEW - Create/edit form |
| `ios/.../Views/SettingsView.swift` | Add domain picker |
| `src-tauri/src/storage.rs` | Improve default seeding |

### Type Definitions (Keep in Sync)

| Platform | File | Lines |
|----------|------|-------|
| Rust | `src-tauri/src/types.rs` | 256-276 |
| TypeScript | `src/types.ts` | 154-172 |
| Swift | `ios/.../Models.swift` | 154-197 |

---

## Testing Checklist

### Phase 1 Verification
- [ ] Create domain with system prompt
- [ ] Assign domain to workspace
- [ ] Start NEW thread in that workspace
- [ ] Verify first message shows domain context (not just second message)

### Phase 2 Verification (iOS)
- [ ] View domain list on iOS
- [ ] Create new domain from iOS
- [ ] Edit existing domain
- [ ] Delete domain
- [ ] Assign domain to workspace via picker
- [ ] View domain dashboard with trends

### Phase 3 Verification
- [ ] Fresh install shows default domains
- [ ] Domain indicator visible in thread header

---

## Summary

| Phase | Priority | Effort | Impact |
|-------|----------|--------|--------|
| 1. Thread injection fix | P0 | Small | Critical - makes domains work |
| 2. iOS CRUD | P1 | Medium | Enables mobile management |
| 3. Polish & defaults | P2 | Small | Better UX |
| 4. Future enhancements | P3 | Large | Nice-to-have |

**Start with Phase 1** - it's the smallest change with the biggest impact. Without it, domains are fundamentally broken.
