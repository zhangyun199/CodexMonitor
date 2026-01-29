import SwiftUI
import CodexMonitorModels

struct ExerciseDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                header

                HStack(spacing: 12) {
                    StatCardView(title: "Workouts", value: "\(store.exerciseDashboard?.stats.workoutCount ?? 0)")
                    StatCardView(title: "Walking", value: miles(store.exerciseDashboard?.stats.walkingMiles))
                }

                HStack(spacing: 12) {
                    StatCardView(title: "Active Days", value: "\(store.exerciseDashboard?.stats.activeDays ?? 0)")
                    StatCardView(title: "Streak", value: streak(store.exerciseDashboard?.stats.currentStreak))
                }

                HStack {
                    TimeRangePicker(selection: $timeRange)
                    Button {
                        Task { await store.fetchExerciseDashboard(range: timeRange) }
                    } label: {
                        Image(systemName: "arrow.clockwise")
                    }
                    .buttonStyle(.bordered)
                }

                if store.dashboardLoading {
                    ProgressView("Loading…")
                }

                if let error = store.dashboardError {
                    Text(error)
                        .foregroundStyle(.red)
                        .font(.caption)
                }

                if let entries = store.exerciseDashboard?.entries, !entries.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text(rangeTitle(timeRange))
                            .font(.headline)
                        ForEach(entries) { entry in
                            VStack(alignment: .leading, spacing: 4) {
                                Text("\(emoji(for: entry.type)) \(entry.description)")
                                    .font(.subheadline)
                                Text(entryMeta(entry))
                                    .font(.caption2)
                                    .foregroundStyle(.secondary)
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                } else {
                    Text("No exercise logged yet.")
                        .foregroundStyle(.secondary)
                }

                if let breakdown = store.exerciseDashboard?.byType, !breakdown.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Activity Breakdown")
                            .font(.headline)
                        ForEach(breakdownRows(breakdown), id: \.type) { row in
                            HStack {
                                Text("\(emoji(for: row.type)) \(row.type)")
                                    .frame(width: 100, alignment: .leading)
                                GeometryReader { proxy in
                                    Capsule()
                                        .fill(Color.blue.opacity(0.6))
                                        .frame(width: proxy.size.width * row.percent, height: 6)
                                        .frame(maxWidth: .infinity, alignment: .leading)
                                }
                                .frame(height: 8)
                                Text("\(row.count)")
                                    .frame(width: 40, alignment: .trailing)
                            }
                            .font(.caption)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                } else {
                    Text("No activity yet.")
                        .foregroundStyle(.secondary)
                }
            }
            .padding()
        }
        .task(id: timeRange) {
            await store.fetchExerciseDashboard(range: timeRange)
        }
    }

    private func miles(_ value: Double?) -> String {
        guard let value else { return "--" }
        return String(format: "%.1f mi", value)
    }

    private func streak(_ value: Int?) -> String {
        guard let value else { return "--" }
        return "\(value) 🔥"
    }

    private func emoji(for type: String) -> String {
        switch type.lowercased() {
        case "walk":
            return "🚶"
        case "strength":
            return "🏋️"
        case "cardio":
            return "🏃"
        default:
            return "✨"
        }
    }

    private func entryMeta(_ entry: ExerciseEntry) -> String {
        var parts: [String] = [formatTime(entry.timestamp)]
        if let miles = entry.miles {
            parts.append(String(format: "%.1f mi", miles))
        }
        if let duration = entry.duration {
            parts.append(String(format: "%.0f min", duration))
        }
        return parts.joined(separator: " • ")
    }

    private func formatTime(_ timestamp: String) -> String {
        if timestamp.count >= 16 {
            let start = timestamp.index(timestamp.startIndex, offsetBy: 11)
            let end = timestamp.index(timestamp.startIndex, offsetBy: 16)
            return String(timestamp[start..<end])
        }
        return timestamp
    }

    private func rangeTitle(_ range: LifeTimeRange) -> String {
        switch range {
        case .today:
            return "Today"
        case .week:
            return "This Week"
        case .month:
            return "This Month"
        case .lifetime:
            return "All Time"
        }
    }

    private func breakdownRows(_ breakdown: [String: Int]) -> [BreakdownRow] {
        let rows = breakdown.sorted { $0.key < $1.key }
        let maxValue = max(1, rows.map { $0.value }.max() ?? 1)
        return rows.map { (type, count) in
            BreakdownRow(type: type, count: count, percent: Double(count) / Double(maxValue))
        }
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("🏋️ Exercise Dashboard")
                .font(.headline)
            if let meta = store.exerciseDashboard?.meta {
                Text("\(meta.periodStart) → \(meta.periodEnd)")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }
}

private struct BreakdownRow: Identifiable {
    let id = UUID()
    let type: String
    let count: Int
    let percent: Double
}
