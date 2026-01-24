import SwiftUI
import CodexMonitorModels

struct ProjectsView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var selectedWorkspace: WorkspaceInfo?

    var body: some View {
        List {
            if let snapshot = store.usageSnapshot {
                Section(header: Text("Usage")) {
                    GlassCard {
                        VStack(alignment: .leading, spacing: 6) {
                            Text("Last 7 days: \(snapshot.totals.last7DaysTokens) tokens")
                                .font(.subheadline)
                            Text("Last 30 days: \(snapshot.totals.last30DaysTokens) tokens")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                    }
                }
            }
            Section(header: Text("Workspaces")) {
                ForEach(store.workspaces) { workspace in
                    NavigationLink(destination: ThreadsForWorkspaceView(workspace: workspace)) {
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text(workspace.name)
                                    .font(.headline)
                                Text(workspace.path)
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                                    .lineLimit(1)
                            }
                            Spacer()
                            Circle()
                                .fill(workspace.connected ? Color.green : Color.orange)
                                .frame(width: 10, height: 10)
                        }
                    }
                }
            }
        }
        .navigationTitle("Projects")
        .toolbar {
            ToolbarItem(placement: .topBarLeading) {
                Button(action: { Task { await store.refreshWorkspaces() } }) {
                    Image(systemName: "arrow.clockwise")
                }
            }
        }
        .overlay(alignment: .bottom) {
            ConnectionStatusView()
                .padding()
        }
        .task {
            await store.refreshWorkspaces()
        }
    }
}

private struct ThreadsForWorkspaceView: View {
    @EnvironmentObject private var store: CodexStore
    let workspace: WorkspaceInfo
    @State private var selectedThreadId: String?

    var body: some View {
        ThreadsListView(selectedWorkspace: .constant(workspace), selectedThreadId: $selectedThreadId)
            .navigationTitle(workspace.name)
            .onAppear {
                store.activeWorkspaceId = workspace.id
            }
            .navigationDestination(isPresented: Binding(
                get: { selectedThreadId != nil },
                set: { if !$0 { selectedThreadId = nil } }
            )) {
                ConversationTabView(selectedThreadId: selectedThreadId)
            }
            .toolbar {
                ToolbarItem(placement: .topBarTrailing) {
                    NavigationLink(destination: FilesView()) {
                        Image(systemName: "doc.text.magnifyingglass")
                    }
                }
                ToolbarItem(placement: .topBarTrailing) {
                    NavigationLink(destination: PromptsView()) {
                        Image(systemName: "text.badge.plus")
                    }
                }
            }
    }
}
