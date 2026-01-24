import SwiftUI

struct ConversationTabView: View {
    @EnvironmentObject private var store: CodexStore
    var selectedThreadId: String? = nil

    var body: some View {
        VStack(spacing: 0) {
            if let threadId = currentThreadId, let workspaceId = store.activeWorkspaceId {
                ConversationView(threadId: threadId)
                Divider()
                ComposerView(workspaceId: workspaceId, threadId: threadId)
                    .padding()
            } else {
                ContentUnavailableView("No thread selected", systemImage: "bubble.left.and.text.bubble.right", description: Text("Pick a workspace and thread to start chatting."))
            }
        }
        .navigationTitle("Codex")
    }

    private var currentThreadId: String? {
        if let selectedThreadId {
            return selectedThreadId
        }
        if let workspaceId = store.activeWorkspaceId {
            return store.activeThreadIdByWorkspace[workspaceId] ?? nil
        }
        return nil
    }
}
