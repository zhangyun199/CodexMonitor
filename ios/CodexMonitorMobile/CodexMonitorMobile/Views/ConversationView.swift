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
            .onChange(of: lastItemHash) { _, _ in
                if let last = items.last {
                    withAnimation {
                        proxy.scrollTo(last.id, anchor: .bottom)
                    }
                }
            }
        }
    }

    private var lastItemHash: Int {
        guard let last = items.last else { return 0 }
        return "\(last.id)-\(last.text?.count ?? 0)-\(last.output?.count ?? 0)".hashValue
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
    @Environment(\.horizontalSizeClass) private var horizontalSizeClass

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
        let isAssistant = item.role == .assistant
        let screenWidth = UIScreen.main.bounds.width
        let maxBubbleWidth: CGFloat = min(
            (horizontalSizeClass == .compact) ? 260 : 420,
            screenWidth * (horizontalSizeClass == .compact ? 0.68 : 0.6)
        )

        return HStack(alignment: .top, spacing: 12) {
            if isAssistant {
                statusDot(isAssistant: true)
                bubble(maxWidth: maxBubbleWidth, alignment: .leading)
                Spacer(minLength: 0)
            } else {
                Spacer(minLength: 0)
                bubble(maxWidth: maxBubbleWidth, alignment: .trailing)
                statusDot(isAssistant: false)
            }
        }
        .frame(maxWidth: .infinity, alignment: isAssistant ? .leading : .trailing)
    }

    private func statusDot(isAssistant: Bool) -> some View {
        Circle()
            .fill(isAssistant ? Color(hex: "3DAAFF") : Color(hex: "43E38A"))
            .frame(width: 8, height: 8)
            .padding(.top, 10)
    }

    private func bubble(maxWidth: CGFloat, alignment: Alignment) -> some View {
        GlassMessageBubble(isAssistant: item.role == .assistant) {
            Text(CodexMonitorRendering.markdown(item.text ?? ""))
                .fixedSize(horizontal: false, vertical: true)
        }
        .frame(maxWidth: maxWidth, alignment: alignment)
        .fixedSize(horizontal: false, vertical: true)
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

    private var assistantColor: Color {
        Color(hex: "3DAAFF")
    }

    private var userColor: Color {
        Color(hex: "43E38A")
    }

    private var bubbleColor: Color {
        isAssistant ? assistantColor : userColor
    }

    private var bubbleBoost: Double {
        // Keep assistant vibrant, soften user haze a touch
        isAssistant ? 0.10 : 0.08
    }

    var body: some View {
        content
            .padding(12)
            .raisedGlassStyle(
                cornerRadius: 14,
                tint: bubbleColor,
                colorBoost: bubbleBoost, // slightly less haze
                borderOpacity: 0.6,
                interactive: true,
                lift: 5
            )
    }
}
