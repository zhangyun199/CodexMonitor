import SwiftUI
import CodexMonitorModels

struct ThreadsListView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var selectedWorkspace: WorkspaceInfo?
    @Binding var selectedThreadId: String?

    var body: some View {
        let workspace = selectedWorkspace ?? store.workspaces.first
        List {
            if let workspace {
                Section(header: Text("Threads")) {
                    ForEach(store.threadsByWorkspace[workspace.id] ?? []) { thread in
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text(thread.name)
                                    .font(.headline)
                                Text(Date(timeIntervalSince1970: thread.updatedAt / 1000), style: .relative)
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Spacer()
                            if store.threadStatusById[thread.id]?.isProcessing == true {
                                ProgressView()
                            }
                            if store.threadStatusById[thread.id]?.hasUnread == true {
                                Circle()
                                    .fill(Color.blue)
                                    .frame(width: 8, height: 8)
                            }
                        }
                        .contentShape(Rectangle())
                        .onTapGesture {
                            selectedThreadId = thread.id
                            store.activeWorkspaceId = workspace.id
                            store.activeThreadIdByWorkspace[workspace.id] = thread.id
                            Task { await store.resumeThread(workspaceId: workspace.id, threadId: thread.id) }
                        }
                    }
                }
            } else {
                Text("No workspace selected.")
                    .foregroundStyle(.secondary)
            }
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
    }
}
