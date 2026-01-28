import SwiftUI
import CodexMonitorModels

struct FinanceDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                HStack(spacing: 12) {
                    StatCardView(title: "Monthly Total", value: currency(store.financeDashboard?.stats.monthlyTotal))
                    StatCardView(title: "Due Soon", value: "\(store.financeDashboard?.stats.dueSoonCount ?? 0)")
                }

                HStack(spacing: 12) {
                    StatCardView(title: "Auto-Pay", value: "\(store.financeDashboard?.stats.autoPayCount ?? 0)")
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

                if let status = store.financeDashboard?.statusMessage {
                    Text(status)
                        .foregroundStyle(.orange)
                        .font(.caption)
                        .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let bills = store.financeDashboard?.bills, !bills.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Upcoming Bills")
                            .font(.headline)
                        ForEach(bills) { bill in
                            VStack(alignment: .leading, spacing: 4) {
                                Text("\(shortDate(bill.nextDueDate)) \(bill.autoPay ? "🔄" : "•") \(bill.name)")
                                    .font(.subheadline)
                                Text("\(currency(bill.amount)) · \(bill.frequency)")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let categories = store.financeDashboard?.byCategory, !categories.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("By Category")
                            .font(.headline)
                        ForEach(categories.sorted(by: { $0.key < $1.key }), id: \.key) { key, value in
                            HStack {
                                Text(key)
                                Spacer()
                                Text(currency(value))
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
            await store.fetchFinanceDashboard(range: timeRange)
        }
    }

    private func currency(_ value: Double?) -> String {
        guard let value else { return "--" }
        return value.formatted(.currency(code: "USD"))
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
}
