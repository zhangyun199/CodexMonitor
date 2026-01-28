import SwiftUI
import CodexMonitorModels

struct ExerciseDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                HStack(spacing: 12) {
                    StatCardView(title: "Workouts", value: "\(store.exerciseDashboard?.stats.workoutCount ?? 0)")
                    StatCardView(title: "Walking", value: miles(store.exerciseDashboard?.stats.walkingMiles))
                }

                HStack(spacing: 12) {
                    StatCardView(title: "Active Days", value: "\(store.exerciseDashboard?.stats.activeDays ?? 0)")
                    StatCardView(title: "Streak", value: streak(store.exerciseDashboard?.stats.currentStreak))
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

                if let entries = store.exerciseDashboard?.entries, !entries.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("This Week")
                            .font(.headline)
                        ForEach(entries) { entry in
                            VStack(alignment: .leading, spacing: 4) {
                                Text("\(emoji(for: entry.type)) \(entry.description)")
                                    .font(.subheadline)
                                Text(formatTime(entry.timestamp))
                                    .font(.caption2)
                                    .foregroundStyle(.secondary)
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let breakdown = store.exerciseDashboard?.byType, !breakdown.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Activity Breakdown")
                            .font(.headline)
                        ForEach(breakdown.sorted(by: { $0.key < $1.key }), id: \.key) { key, value in
                            HStack {
                                Text("\(emoji(for: key)) \(key)")
                                Spacer()
                                Text("\(value)")
                            }
                            .font(.caption)
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
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

    private func formatTime(_ timestamp: String) -> String {
        if timestamp.count >= 16 {
            let start = timestamp.index(timestamp.startIndex, offsetBy: 11)
            let end = timestamp.index(timestamp.startIndex, offsetBy: 16)
            return String(timestamp[start..<end])
        }
        return timestamp
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
}
