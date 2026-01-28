import SwiftUI
import CodexMonitorModels

struct MediaDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                HStack(spacing: 12) {
                    StatCardView(title: "Backlog", value: "\(store.mediaDashboard?.stats.backlogCount ?? 0)")
                    StatCardView(title: "In Progress", value: "\(store.mediaDashboard?.stats.inProgressCount ?? 0)")
                }

                HStack(spacing: 12) {
                    StatCardView(title: "Completed", value: "\(store.mediaDashboard?.stats.completedCount ?? 0)")
                    StatCardView(title: "Avg Rating", value: ratingLabel())
                }

                TimeRangePicker(selection: $timeRange)

                if store.dashboardLoading {
                    ProgressView("Loading…")
                }

                if let error = store.dashboardError {
                    Text(error)
                        .foregroundStyle(.red)
                        .font(.caption)
                }

                if let byType = store.mediaDashboard?.byType, !byType.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("By Type")
                            .font(.headline)
                        ForEach(byType.sorted(by: { $0.key < $1.key }), id: \.key) { key, value in
                            HStack {
                                Text(key.capitalized)
                                Spacer()
                                Text("\(value)")
                            }
                            .font(.caption)
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let recent = store.mediaDashboard?.recentlyActive, !recent.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Recently Active")
                            .font(.headline)
                        ForEach(recent) { item in
                            VStack(alignment: .leading, spacing: 4) {
                                Text(item.title)
                                    .font(.subheadline)
                                Text("\(item.mediaType.rawValue.capitalized) · \(item.status.rawValue)")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                                if let rating = item.rating {
                                    Text("⭐ \(rating, specifier: "%.1f")")
                                        .font(.caption2)
                                }
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }
            }
            .padding()
        }
        .task(id: timeRange) {
            await store.fetchMediaDashboard(range: timeRange)
        }
    }

    private func ratingLabel() -> String {
        guard let rating = store.mediaDashboard?.stats.avgRating else { return "--" }
        return String(format: "%.1f", rating)
    }
}
