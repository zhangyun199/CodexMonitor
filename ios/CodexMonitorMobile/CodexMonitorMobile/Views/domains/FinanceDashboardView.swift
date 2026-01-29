import SwiftUI
import CodexMonitorModels

struct FinanceDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                header

                HStack(spacing: 12) {
                    StatCardView(title: "Monthly Total", value: currency(store.financeDashboard?.stats.monthlyTotal))
                    StatCardView(title: "Due Soon", value: "\(store.financeDashboard?.stats.dueSoonCount ?? 0)")
                }

                HStack(spacing: 12) {
                    StatCardView(title: "Auto-Pay", value: "\(store.financeDashboard?.stats.autoPayCount ?? 0)")
                }

                HStack {
                    TimeRangePicker(selection: $timeRange)
                    Button {
                        Task { await store.fetchFinanceDashboard(range: timeRange) }
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
                } else {
                    Text("No bills found.")
                        .foregroundStyle(.secondary)
                }

                if let categories = store.financeDashboard?.byCategory, !categories.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("By Category")
                            .font(.headline)
                        ForEach(categoryRows(categories), id: \.name) { row in
                            HStack {
                                Text(row.name)
                                    .frame(width: 120, alignment: .leading)
                                GeometryReader { proxy in
                                    Capsule()
                                        .fill(Color.orange.opacity(0.6))
                                        .frame(width: proxy.size.width * row.percent, height: 6)
                                        .frame(maxWidth: .infinity, alignment: .leading)
                                }
                                .frame(height: 8)
                                Text(currency(row.amount))
                                    .frame(width: 80, alignment: .trailing)
                            }
                            .font(.caption)
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                } else {
                    Text("No category totals yet.")
                        .foregroundStyle(.secondary)
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

    private func categoryRows(_ categories: [String: Double]) -> [CategoryRow] {
        let rows = categories.sorted { $0.key < $1.key }
        let maxValue = max(1, rows.map { $0.value }.max() ?? 1)
        return rows.map { (name, amount) in
            CategoryRow(name: name, amount: amount, percent: amount / maxValue)
        }
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("💸 Finance Dashboard")
                .font(.headline)
            if let meta = store.financeDashboard?.meta {
                Text("\(meta.periodStart) → \(meta.periodEnd)")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }
}

private struct CategoryRow: Identifiable {
    let id = UUID()
    let name: String
    let amount: Double
    let percent: Double
}
