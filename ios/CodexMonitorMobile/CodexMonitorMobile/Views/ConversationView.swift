import SwiftUI
import CodexMonitorRendering
import CodexMonitorModels

struct ConversationView: View {
    @EnvironmentObject private var store: CodexStore
    var threadId: String?

    var body: some View {
        ScrollViewReader { proxy in
            ScrollView {
                LazyVStack(alignment: .leading, spacing: 12) {
                    if let workspaceId = store.activeWorkspaceId {
                        ForEach(store.approvals.filter { $0.workspaceId == workspaceId }) { approval in
                            ApprovalCard(approval: approval)
                                .id("approval-\(approval.id)")
                        }
                    }

                    ForEach(items) { item in
                        ConversationItemView(item: item)
                            .id(item.id)
                    }
                }
                .padding()
            }
            .onChange(of: items.count) { _, _ in
                if let last = items.last {
                    withAnimation {
                        proxy.scrollTo(last.id, anchor: .bottom)
                    }
                }
            }
        }
    }

    private var items: [ConversationItem] {
        guard let threadId else { return [] }
        return store.itemsByThread[threadId] ?? []
    }
}

private struct ApprovalCard: View {
    @EnvironmentObject private var store: CodexStore
    let approval: ApprovalRequest

    var body: some View {
        GlassCard {
            VStack(alignment: .leading, spacing: 8) {
                Text("Approval Required")
                    .font(.headline)
                Text(approval.method)
                    .font(.caption)
                    .foregroundStyle(.secondary)
                HStack {
                    Button("Approve") {
                        Task { await store.respondToApproval(approval, decision: .accept) }
                    }
                    .buttonStyle(.borderedProminent)

                    Button("Deny") {
                        Task { await store.respondToApproval(approval, decision: .decline) }
                    }
                    .buttonStyle(.bordered)
                }
            }
        }
    }
}

private struct ConversationItemView: View {
    let item: ConversationItem

    var body: some View {
        switch item.kind {
        case .message:
            messageView
        case .reasoning:
            reasoningView
        case .diff:
            diffView
        case .review:
            reviewView
        case .tool:
            toolView
        }
    }

    private var messageView: some View {
        HStack(alignment: .top, spacing: 12) {
            Circle()
                .fill(item.role == .assistant ? Color.blue : Color.green)
                .frame(width: 8, height: 8)
                .padding(.top, 10)

            GlassMessageBubble(isAssistant: item.role == .assistant) {
                Text(CodexMonitorRendering.markdown(item.text ?? ""))
                    .frame(maxWidth: .infinity, alignment: .leading)
            }
        }
    }

    private var reasoningView: some View {
        GlassCard {
            VStack(alignment: .leading, spacing: 6) {
                Text("Reasoning")
                    .font(.caption.weight(.semibold))
                if let summary = item.summary, !summary.isEmpty {
                    Text(AttributedString(summary))
                        .font(.caption)
                }
                if let content = item.content, !content.isEmpty {
                    Text(CodexMonitorRendering.monospaced(content))
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
        }
    }

    private var diffView: some View {
        GlassCard {
            VStack(alignment: .leading, spacing: 6) {
                Text(item.title ?? "Diff")
                    .font(.headline)
                if let diff = item.diff {
                    Text(CodexMonitorRendering.monospaced(diff))
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
        }
    }

    private var reviewView: some View {
        GlassCard {
            VStack(alignment: .leading, spacing: 6) {
                Text(item.state == .started ? "Review Started" : "Review Completed")
                    .font(.headline)
                if let text = item.text {
                    Text(text)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
        }
    }

    private var toolView: some View {
        GlassCard {
            VStack(alignment: .leading, spacing: 6) {
                Text(item.title ?? "Tool Output")
                    .font(.headline)
                if let detail = item.detail, !detail.isEmpty {
                    Text(detail)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                if let output = item.output, !output.isEmpty {
                    Text(CodexMonitorRendering.monospaced(output))
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }
        }
    }
}

// MARK: - Glass Message Bubble
private struct GlassMessageBubble<Content: View>: View {
    let isAssistant: Bool
    @ViewBuilder var content: Content

    var body: some View {
        if #available(iOS 26.0, *) {
            content
                .padding(12)
                .glassEffect(
                    isAssistant ? .regular.tint(.blue.opacity(0.3)) : .regular.tint(.green.opacity(0.3)),
                    in: .rect(cornerRadius: 14)
                )
                .overlay(
                    RoundedRectangle(cornerRadius: 14)
                        .strokeBorder(
                            isAssistant ? Color.blue.opacity(0.2) : Color.green.opacity(0.2),
                            lineWidth: 0.5
                        )
                )
        } else {
            content
                .padding(12)
                .background(
                    .ultraThinMaterial,
                    in: RoundedRectangle(cornerRadius: 14)
                )
                .overlay(
                    RoundedRectangle(cornerRadius: 14)
                        .strokeBorder(
                            isAssistant ? Color.blue.opacity(0.2) : Color.green.opacity(0.2),
                            lineWidth: 0.5
                        )
                )
        }
    }
}
