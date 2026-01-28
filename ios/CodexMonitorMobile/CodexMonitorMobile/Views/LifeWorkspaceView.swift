import SwiftUI
import CodexMonitorModels

struct LifeWorkspaceView: View {
    @EnvironmentObject private var store: CodexStore

    var body: some View {
        VStack(spacing: 0) {
            DomainTabBar(selection: Binding(
                get: { store.lifeActiveDomain },
                set: { store.lifeActiveDomain = $0 }
            ))

            if let domain = store.lifeActiveDomain {
                switch domain {
                case .delivery:
                    DeliveryDashboardView(timeRange: Binding(
                        get: { store.lifeTimeRange },
                        set: { store.lifeTimeRange = $0 }
                    ))
                case .media:
                    MediaDashboardView()
                case .youtube:
                    YouTubeDashboardView()
                default:
                    Text("Dashboard coming soon.")
                        .foregroundStyle(.secondary)
                        .padding()
                }
            } else {
                conversationLayer
            }
        }
        .background {
            GradientBackground()
        }
    }

    private var conversationLayer: some View {
        Group {
            if let threadId = currentThreadId, let workspaceId = store.activeWorkspaceId {
                ConversationView(threadId: threadId)
                Divider()
                ComposerView(workspaceId: workspaceId, threadId: threadId)
                    .padding()
            } else {
                ContentUnavailableView(
                    "No thread selected",
                    systemImage: "bubble.left.and.text.bubble.right",
                    description: Text("Pick a workspace and thread to start chatting.")
                )
            }
        }
    }

    private var currentThreadId: String? {
        if let workspaceId = store.activeWorkspaceId {
            return store.activeThreadIdByWorkspace[workspaceId] ?? nil
        }
        return nil
    }
}
