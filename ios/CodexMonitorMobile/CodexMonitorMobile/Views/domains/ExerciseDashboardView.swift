import SwiftUI
import CodexMonitorModels

struct ExerciseDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                header

                activityCard

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
                        Text(activityTitle)
                            .font(.headline)
                        ForEach(entries) { entry in
                            GlassCardView {
                                VStack(alignment: .leading, spacing: 4) {
                                    Text("\(emoji(for: entry.type)) \(entry.description)")
                                        .font(.subheadline)
                                    Text(entryMeta(entry))
                                        .font(.caption2)
                                        .foregroundStyle(.secondary)
                                }
                            }
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                } else {
                    Text("No exercise logged yet.")
                        .foregroundStyle(.secondary)
                }

                GlassCardView {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("This Week")
                            .font(.headline)
                        HStack(spacing: 6) {
                            ForEach(activityDots) { dot in
                                Circle()
                                    .fill(dot.isActive ? LifeColors.exercise : Color.white.opacity(0.2))
                                    .frame(width: 10, height: 10)
                            }
                        }
                    }
                }
            }
            .padding()
        }
        .task(id: timeRange) {
            await store.fetchExerciseDashboard(range: timeRange)
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

    private var activityCard: some View {
        let totals = computeTotals()
        let streak = store.exerciseDashboard?.stats.currentStreak ?? 0
        return GlassCardView {
            VStack(alignment: .leading, spacing: 12) {
                HStack {
                    Text("Activity")
                        .font(.headline)
                    Spacer()
                    Text("🔥 \(streak) day streak")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }

                ActivityRow(label: "Move", value: "\(Int(totals.move)) / 500 cal", progress: totals.move / 500, color: LifeColors.move)
                ActivityRow(label: "Exercise", value: "\(Int(totals.minutes)) / 30 min", progress: totals.minutes / 30, color: LifeColors.exercise)
                ActivityRow(label: "Miles", value: String(format: "%.1f / 4.0 mi", totals.miles), progress: totals.miles / 4, color: LifeColors.stand)
            }
        }
    }

    private func computeTotals() -> (move: Double, minutes: Double, miles: Double) {
        let entries = store.exerciseDashboard?.entries ?? []
        let minutes = entries.reduce(0) { $0 + ($1.duration ?? 0) }
        let miles = store.exerciseDashboard?.stats.walkingMiles ?? 0
        let move = entries.reduce(0) { $0 + ($1.miles ?? 0) * 100 } + Double(store.exerciseDashboard?.stats.workoutCount ?? 0) * 150
        let normalizedMinutes = minutes > 0 ? minutes : Double(store.exerciseDashboard?.stats.workoutCount ?? 0) * 30
        return (move: move, minutes: normalizedMinutes, miles: miles)
    }

    private var activityDots: [ActivityDot] {
        let entries = store.exerciseDashboard?.entries ?? []
        let active = Set(entries.map { String($0.timestamp.prefix(10)) })
        let today = Date()
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd"
        return (0..<7).map { index in
            let day = Calendar.current.date(byAdding: .day, value: index - 6, to: today) ?? today
            let key = formatter.string(from: day)
            return ActivityDot(id: UUID(), isActive: active.contains(key))
        }
    }

    private var activityTitle: String {
        timeRange == .today ? "Today's Activity" : "Activity"
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

private struct ActivityRow: View {
    let label: String
    let value: String
    let progress: Double
    let color: Color

    var body: some View {
        HStack {
            Text(label)
                .frame(width: 80, alignment: .leading)
                .font(.caption)
                .foregroundStyle(.secondary)
            ProgressBarView(progress: progress, color: color, height: 8)
            Text(value)
                .frame(width: 110, alignment: .trailing)
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }
}

private struct ActivityDot: Identifiable {
    let id: UUID
    let isActive: Bool
}
