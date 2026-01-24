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
        List {
            if let workspace {
                Section(header: Text("Threads")) {
                    ForEach(store.threadsByWorkspace[workspace.id] ?? []) { thread in
                        let isSelected = selectedThreadId == thread.id

                        HStack(spacing: 12) {
                            // Green dot for active threads
                            if isActive(thread) {
                                Circle()
                                    .fill(Color.green)
                                    .frame(width: 8, height: 8)
                            } else {
                                Circle()
                                    .fill(Color.clear)
                                    .frame(width: 8, height: 8)
                            }

                            VStack(alignment: .leading, spacing: 4) {
                                Text(displayName(for: thread))
                                    .font(.headline)
                                    .fontWeight(.semibold)
                                Text(formatDate(thread.updatedAt), style: .relative)
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }

                            Spacer()

                            if isActive(thread) {
                                ProgressView()
                                    .scaleEffect(0.8)
                            }

                            if store.threadStatusById[thread.id]?.hasUnread == true {
                                Circle()
                                    .fill(Color.blue)
                                    .frame(width: 8, height: 8)
                            }
                        }
                        .padding(.vertical, 4)
                        .contentShape(Rectangle())
                        .onTapGesture {
                            selectedThreadId = thread.id
                            store.activeWorkspaceId = workspace.id
                            store.activeThreadIdByWorkspace[workspace.id] = thread.id
                            Task { await store.resumeThread(workspaceId: workspace.id, threadId: thread.id) }
                        }
                        .listRowBackground(
                            RoundedRectangle(cornerRadius: 8)
                                .fill(isSelected ? Color.accentColor.opacity(0.15) : Color.clear)
                                .padding(.horizontal, 4)
                        )
                    }
                }
            } else {
                Text("No workspace selected.")
                    .foregroundStyle(.secondary)
            }
        }
        .scrollContentBackground(.hidden)
        .background(.clear)
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
