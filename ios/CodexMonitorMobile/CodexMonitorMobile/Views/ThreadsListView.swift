import SwiftUI
import CodexMonitorModels

struct ThreadsListView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var selectedWorkspace: WorkspaceInfo?
    @Binding var selectedThreadId: String?

    // MARK: - Helper Functions

    /// Smart timestamp conversion - handles both seconds and milliseconds
    private func formatDate(_ timestamp: Double) -> Date {
        // If timestamp > year 2100 in seconds (~4.1 billion), it's milliseconds
        if timestamp > 4_102_444_800 {
            return Date(timeIntervalSince1970: timestamp / 1000)
        } else {
            return Date(timeIntervalSince1970: timestamp)
        }
    }

    /// Display name for thread - handles UUID names gracefully
    private func displayName(for thread: ThreadSummary) -> String {
        // Check if name looks like UUID (contains dashes and is 36+ chars)
        if thread.name.count >= 36 && thread.name.contains("-") {
            return "Thread " + String(thread.name.prefix(8)) + "..."
        }
        return thread.name
    }

    /// Check if thread is currently active
    private func isActive(_ thread: ThreadSummary) -> Bool {
        store.threadStatusById[thread.id]?.isProcessing == true
    }

    var body: some View {
        let workspace = selectedWorkspace ?? store.workspaces.first
        ScrollView {
            LazyVStack(spacing: 8) {
                if let workspace {
                    ForEach(store.threadsByWorkspace[workspace.id] ?? []) { thread in
                        let isSelected = selectedThreadId == thread.id

                        GlassThreadRow(
                            thread: thread,
                            displayName: displayName(for: thread),
                            formattedDate: formatDate(thread.updatedAt),
                            isSelected: isSelected,
                            isActive: isActive(thread),
                            hasUnread: store.threadStatusById[thread.id]?.hasUnread == true
                        )
                        .onTapGesture {
                            selectedThreadId = thread.id
                            store.activeWorkspaceId = workspace.id
                            store.activeThreadIdByWorkspace[workspace.id] = thread.id
                            Task { await store.resumeThread(workspaceId: workspace.id, threadId: thread.id) }
                        }
                    }
                } else {
                    Text("No workspace selected.")
                        .foregroundStyle(.secondary)
                        .padding()
                }
            }
            .padding(.horizontal)
            .padding(.top, 8)
        }
        .background {
            GradientBackground()
        }
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                Button(action: {
                    guard let workspace else { return }
                    Task {
                        let threadId = await store.startThread(in: workspace.id)
                        selectedThreadId = threadId
                    }
                }) {
                    Image(systemName: "plus.bubble")
                }
            }
            ToolbarItem(placement: .topBarLeading) {
                Button(action: {
                    guard let workspace else { return }
                    Task { await store.refreshThreads(for: workspace.id) }
                }) {
                    Image(systemName: "arrow.clockwise")
                }
            }
        }
        .onAppear {
            if let workspace {
                Task { await store.refreshThreads(for: workspace.id) }
            }
        }
        .onChange(of: selectedWorkspace) { _, newValue in
            if let workspace = newValue ?? store.workspaces.first {
                Task { await store.refreshThreads(for: workspace.id) }
            }
        }
        .onChange(of: store.workspaces) { _, newValue in
            // When workspaces load and no selection, fetch threads for first workspace
            if selectedWorkspace == nil, let workspace = newValue.first {
                Task { await store.refreshThreads(for: workspace.id) }
            }
        }
    }
}

// MARK: - Glass Thread Row
private struct GlassThreadRow: View {
    let thread: ThreadSummary
    let displayName: String
    let formattedDate: Date
    let isSelected: Bool
    let isActive: Bool
    let hasUnread: Bool

    var body: some View {
        HStack(spacing: 12) {
            // Status indicator
            Circle()
                .fill(isActive ? Color.green : Color.clear)
                .frame(width: 8, height: 8)

            VStack(alignment: .leading, spacing: 4) {
                Text(displayName)
                    .font(.headline)
                    .fontWeight(.semibold)
                Text(formattedDate, style: .relative)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }

            Spacer()

            if isActive {
                ProgressView()
                    .scaleEffect(0.8)
            }

            if hasUnread {
                Circle()
                    .fill(Color.blue)
                    .frame(width: 8, height: 8)
            }
        }
        .padding(.horizontal, 14)
        .padding(.vertical, 12)
        .contentShape(Rectangle())
        .glassListRow(isSelected: isSelected)
    }
}
