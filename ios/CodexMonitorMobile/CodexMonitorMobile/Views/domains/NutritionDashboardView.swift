import SwiftUI
import CodexMonitorModels

struct NutritionDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                HStack(spacing: 12) {
                    StatCardView(title: "Calories", value: number(store.nutritionDashboard?.stats.calories))
                    StatCardView(title: "Protein", value: grams(store.nutritionDashboard?.stats.protein))
                }

                HStack(spacing: 12) {
                    StatCardView(title: "Carbs", value: grams(store.nutritionDashboard?.stats.carbs))
                    StatCardView(title: "Fat", value: grams(store.nutritionDashboard?.stats.fat))
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

                if let meals = store.nutritionDashboard?.meals, !meals.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Meals")
                            .font(.headline)
                        ForEach(meals) { meal in
                            VStack(alignment: .leading, spacing: 4) {
                                Text("\(emoji(for: meal.mealType)) \(meal.description)")
                                    .font(.subheadline)
                                Text(formatTime(meal.timestamp))
                                    .font(.caption2)
                                    .foregroundStyle(.secondary)
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let trend = store.nutritionDashboard?.weeklyTrend, !trend.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Weekly Trends")
                            .font(.headline)
                        ForEach(trend.sorted(by: { $0.key < $1.key }), id: \.key) { key, value in
                            HStack {
                                Text(shortDate(key))
                                Spacer()
                                Text("\(Int(value)) cal")
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
            await store.fetchNutritionDashboard(range: timeRange)
        }
    }

    private func number(_ value: Double?) -> String {
        guard let value else { return "--" }
        return "\(Int(value))"
    }

    private func grams(_ value: Double?) -> String {
        guard let value else { return "--" }
        return "\(Int(value))g"
    }

    private func formatTime(_ timestamp: String) -> String {
        if timestamp.count >= 16 {
            let start = timestamp.index(timestamp.startIndex, offsetBy: 11)
            let end = timestamp.index(timestamp.startIndex, offsetBy: 16)
            return String(timestamp[start..<end])
        }
        return timestamp
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
