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

                HStack(spacing: 12) {
                    StatCardView(
                        title: "Calories",
                        value: goalValue(
                            store.nutritionDashboard?.stats.calories,
                            goals.calories,
                            suffix: ""
                        )
                    )
                    StatCardView(
                        title: "Protein",
                        value: goalValue(
                            store.nutritionDashboard?.stats.protein,
                            goals.protein,
                            suffix: "g"
                        )
                    )
                }

                HStack(spacing: 12) {
                    StatCardView(
                        title: "Carbs",
                        value: goalValue(
                            store.nutritionDashboard?.stats.carbs,
                            goals.carbs,
                            suffix: "g"
                        )
                    )
                    StatCardView(
                        title: "Fat",
                        value: goalValue(
                            store.nutritionDashboard?.stats.fat,
                            goals.fat,
                            suffix: "g"
                        )
                    )
                }

                HStack(spacing: 12) {
                    StatCardView(title: "Fiber", value: grams(store.nutritionDashboard?.stats.fiber))
                    StatCardView(title: "Meals", value: count(store.nutritionDashboard?.stats.mealCount))
                }

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
                        Text("Meals")
                            .font(.headline)
                        ForEach(meals) { meal in
                            VStack(alignment: .leading, spacing: 4) {
                                Text("\(emoji(for: meal.mealType)) \(meal.description)")
                                    .font(.subheadline)
                                Text(mealMeta(meal))
                                    .font(.caption2)
                                    .foregroundStyle(.secondary)
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                } else {
                    Text("No meals logged yet.")
                        .foregroundStyle(.secondary)
                }

                if let trend = store.nutritionDashboard?.weeklyTrend, !trend.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Weekly Trends")
                            .font(.headline)
                        ForEach(trendRows(trend), id: \.date) { row in
                            HStack {
                                Text(shortDate(row.date))
                                    .frame(width: 50, alignment: .leading)
                                GeometryReader { proxy in
                                    Capsule()
                                        .fill(Color.green.opacity(0.6))
                                        .frame(width: proxy.size.width * row.percent, height: 6)
                                        .frame(maxWidth: .infinity, alignment: .leading)
                                }
                                .frame(height: 8)
                                Text("\(Int(row.value)) cal")
                                    .frame(width: 70, alignment: .trailing)
                            }
                            .font(.caption)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                } else {
                    Text("No trends yet.")
                        .foregroundStyle(.secondary)
                }
            }
            .padding()
        }
        .task(id: timeRange) {
            await store.fetchNutritionDashboard(range: timeRange)
        }
    }

    private func grams(_ value: Double?) -> String {
        guard let value else { return "--" }
        return "\(Int(value))g"
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

    private func goalValue(_ value: Double?, _ goal: Int, suffix: String) -> String {
        guard let value else { return "--" }
        let current = Int(value)
        return "\(current)\(suffix) / \(goal)\(suffix)"
    }

    private func count(_ value: Int?) -> String {
        guard let value else { return "--" }
        return "\(value)"
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
}

private struct NutritionGoals {
    let calories = 2000
    let protein = 180
    let carbs = 150
    let fat = 80
}

private struct TrendRow: Identifiable {
    let id = UUID()
    let date: String
    let value: Double
    let percent: Double
}
