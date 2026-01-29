import SwiftUI
import CodexMonitorModels

struct NutritionDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange
    private let goals = NutritionGoals()

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                header

                heroCard

                macroCard

                HStack {
                    TimeRangePicker(selection: $timeRange)
                    Button {
                        Task { await store.fetchNutritionDashboard(range: timeRange) }
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

                if let meals = store.nutritionDashboard?.meals, !meals.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text(mealTitle)
                            .font(.headline)
                        ForEach(meals) { meal in
                            GlassCardView {
                                VStack(alignment: .leading, spacing: 4) {
                                    Text("\(emoji(for: meal.mealType)) \(meal.description)")
                                        .font(.subheadline)
                                    Text(mealMeta(meal))
                                        .font(.caption2)
                                        .foregroundStyle(.secondary)
                                }
                            }
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                } else {
                    Text("No meals logged yet.")
                        .foregroundStyle(.secondary)
                }

                if let trend = store.nutritionDashboard?.weeklyTrend, !trend.isEmpty {
                    GlassCardView {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Weekly Trends")
                                .font(.headline)
                            ForEach(trendRows(trend), id: \.date) { row in
                                HStack {
                                    Text(shortDate(row.date))
                                        .frame(width: 50, alignment: .leading)
                                    ProgressBarView(progress: row.percent, color: LifeColors.earnings, height: 6)
                                    Text("\(Int(row.value)) cal")
                                        .frame(width: 70, alignment: .trailing)
                                }
                                .font(.caption)
                            }
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                    }
                }
            }
            .padding()
        }
        .task(id: timeRange) {
            await store.fetchNutritionDashboard(range: timeRange)
        }
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("🍽️ Nutrition Dashboard")
                .font(.headline)
            if let meta = store.nutritionDashboard?.meta {
                Text("\(meta.periodStart) → \(meta.periodEnd)")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var heroCard: some View {
        let calories = store.nutritionDashboard?.stats.calories ?? 0
        let remaining = max(0, Double(goals.calories) - calories)
        return GlassCardView {
            VStack(alignment: .leading, spacing: 8) {
                Text("\(Int(calories))")
                    .font(.system(size: 40, weight: .bold, design: .rounded))
                Text("calories")
                    .font(.caption)
                    .foregroundStyle(.secondary)
                Text("\(Int(remaining)) remaining")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
        }
    }

    private var macroCard: some View {
        let stats = store.nutritionDashboard?.stats
        let rows: [MacroRow] = [
            MacroRow(label: "Protein", value: stats?.protein ?? 0, goal: Double(goals.protein), color: LifeColors.protein),
            MacroRow(label: "Carbs", value: stats?.carbs ?? 0, goal: Double(goals.carbs), color: LifeColors.carbs),
            MacroRow(label: "Fat", value: stats?.fat ?? 0, goal: Double(goals.fat), color: LifeColors.fat),
            MacroRow(label: "Fiber", value: stats?.fiber ?? 0, goal: Double(goals.fiber), color: LifeColors.fiber)
        ]
        return GlassCardView {
            VStack(alignment: .leading, spacing: 12) {
                Text("Macros")
                    .font(.headline)
                ForEach(rows) { row in
                    HStack {
                        Text(row.label)
                            .frame(width: 70, alignment: .leading)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        ProgressBarView(progress: row.progress, color: row.color, height: 8)
                        Text("\(Int(row.value))g / \(Int(row.goal))g")
                            .frame(width: 90, alignment: .trailing)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
    }

    private var mealTitle: String {
        timeRange == .today ? "Today's Meals" : "Meals"
    }

    private func mealMeta(_ meal: MealEntry) -> String {
        var parts: [String] = [formatTime(meal.timestamp)]
        if let calories = meal.estimatedCalories {
            parts.append("\(Int(calories)) cal")
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

    private func trendRows(_ trend: [String: Double]) -> [TrendRow] {
        let rows = trend.sorted { $0.key < $1.key }
        let maxValue = max(Double(goals.calories), rows.map { $0.value }.max() ?? 0)
        return rows.map { (date, value) in
            TrendRow(date: date, value: value, percent: maxValue > 0 ? value / maxValue : 0)
        }
    }

    private func shortDate(_ value: String) -> String {
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd"
        if let date = formatter.date(from: value) {
            formatter.dateFormat = "MMM d"
            return formatter.string(from: date)
        }
        return value
    }

    private func emoji(for mealType: String) -> String {
        switch mealType.lowercased() {
        case "breakfast":
            return "🌅"
        case "lunch":
            return "🌞"
        case "dinner":
            return "🌙"
        default:
            return "🍪"
        }
    }
}

private struct NutritionGoals {
    let calories = 2000
    let protein = 180
    let carbs = 150
    let fat = 80
    let fiber = 35
}

private struct TrendRow: Identifiable {
    let id = UUID()
    let date: String
    let value: Double
    let percent: Double
}

private struct MacroRow: Identifiable {
    let id = UUID()
    let label: String
    let value: Double
    let goal: Double
    let color: Color

    var progress: Double {
        guard goal > 0 else { return 0 }
        return min(1, value / goal)
    }
}
