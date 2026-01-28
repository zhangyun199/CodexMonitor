import SwiftUI
import CodexMonitorModels

struct DomainDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @State private var range: String = "7d"

    var body: some View {
        let activeWorkspace = store.workspaces.first { $0.id == store.activeWorkspaceId }
        let domainId = activeWorkspace?.settings.domainId
        let domain = store.domains.first { $0.id == domainId }
        let snapshot = activeWorkspace.flatMap { store.domainTrendsByWorkspace[$0.id] }

        VStack(alignment: .leading, spacing: 16) {
            HStack {
                Text(domain?.theme.icon ?? "🧭")
                Text(domain?.name ?? "Domain Dashboard")
                    .font(.headline)
                Spacer()
                Button {
                    if let workspaceId = activeWorkspace?.id, let domainId = domainId {
                        Task {
                            await store.refreshDomainTrends(workspaceId: workspaceId, range: range)
                        }
                    }
                } label: {
                    Image(systemName: "arrow.clockwise")
                }
                .buttonStyle(.bordered)
                Picker("Range", selection: $range) {
                    Text("7d").tag("7d")
                    Text("30d").tag("30d")
                    Text("Life").tag("lifetime")
                }
                .pickerStyle(.segmented)
                .frame(width: 180)
            }

            if let snapshot {
                ScrollView {
                    VStack(alignment: .leading, spacing: 16) {
                        LazyVGrid(columns: [GridItem(.adaptive(minimum: 120))], spacing: 12) {
                            ForEach(snapshot.cards) { card in
                                VStack(alignment: .leading, spacing: 6) {
                                    Text(card.label)
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                    Text(card.value)
                                        .font(.headline)
                                    if let sub = card.subLabel {
                                        Text(sub)
                                            .font(.caption2)
                                            .foregroundStyle(.secondary)
                                    }
                                }
                                .padding(12)
                                .background(.thinMaterial)
                                .cornerRadius(12)
                            }
                        }

                        ForEach(snapshot.lists) { list in
                            VStack(alignment: .leading, spacing: 8) {
                                Text(list.title)
                                    .font(.subheadline)
                                    .foregroundStyle(.secondary)
                                ForEach(Array(list.items.enumerated()), id: \.offset) { _, item in
                                    VStack(alignment: .leading, spacing: 2) {
                                        HStack {
                                            Text(item.label)
                                            Spacer()
                                            Text(item.value)
                                        }
                                        .font(.callout)
                                        if let sub = item.subLabel {
                                            Text(sub)
                                                .font(.caption2)
                                                .foregroundStyle(.secondary)
                                        }
                                    }
                                    Divider()
                                }
                            }
                        }
                    }
                }
            } else {
                Text("No dashboard data yet.")
                    .foregroundStyle(.secondary)
            }
        }
        .padding()
        .task(id: range) {
            if let workspaceId = activeWorkspace?.id, let domainId = domainId {
                await store.refreshDomainTrends(workspaceId: workspaceId, range: range)
            }
        }
        .onAppear {
            Task {
                await store.refreshDomains()
                if let workspaceId = activeWorkspace?.id, let domainId = domainId {
                    await store.refreshDomainTrends(workspaceId: workspaceId, range: range)
                }
            }
        }
    }
}
