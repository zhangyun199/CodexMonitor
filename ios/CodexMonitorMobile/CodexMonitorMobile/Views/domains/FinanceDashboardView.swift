import SwiftUI
import CodexMonitorModels

struct FinanceDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                header

                summaryCard

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
                            GlassCardView {
                                HStack(alignment: .top, spacing: 10) {
                                    Circle()
                                        .fill(isDueSoon(bill.nextDueDate) ? LifeColors.negative : Color.white.opacity(0.25))
                                        .frame(width: 10, height: 10)
                                        .padding(.top, 4)
                                    VStack(alignment: .leading, spacing: 4) {
                                        Text("\(shortDate(bill.nextDueDate)) \(bill.autoPay ? "🔄" : "•") \(bill.name)")
                                            .font(.subheadline)
                                        Text("\(currency(bill.amount)) · \(bill.frequency) · \(daysUntil(bill.nextDueDate))d")
                                            .font(.caption)
                                            .foregroundStyle(.secondary)
                                    }
                                }
                            }
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                } else {
                    Text("No bills found.")
                        .foregroundStyle(.secondary)
                }

                if let categories = store.financeDashboard?.byCategory, !categories.isEmpty {
                    GlassCardView {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Spending by Category")
                                .font(.headline)
                            ForEach(categoryRows(categories), id: \.name) { row in
                                HStack {
                                    Text(row.name)
                                        .frame(width: 120, alignment: .leading)
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                    ProgressBarView(progress: row.percent, color: Color.white.opacity(0.7), height: 6)
                                    Text(currency(row.amount))
                                        .frame(width: 80, alignment: .trailing)
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }
                            }
                        }
                        .frame(maxWidth: .infinity, alignment: .leading)
                    }
                }
            }
            .padding()
        }
        .task(id: timeRange) {
            await store.fetchFinanceDashboard(range: timeRange)
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

    private var summaryCard: some View {
        let stats = store.financeDashboard?.stats
        return GlassCardView {
            VStack(alignment: .leading, spacing: 8) {
                Text("Monthly Summary")
                    .font(.headline)
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Total Due")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Text(currency(stats?.monthlyTotal))
                            .font(.headline)
                    }
                    Spacer()
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Due Soon")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Text("\(stats?.dueSoonCount ?? 0)")
                            .font(.headline)
                    }
                    Spacer()
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Auto‑Pay")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                        Text("\(stats?.autoPayCount ?? 0)")
                            .font(.headline)
                    }
                }
            }
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

    private func daysUntil(_ value: String) -> Int {
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd"
        guard let date = formatter.date(from: value) else { return 0 }
        let diff = Calendar.current.dateComponents([.day], from: Date(), to: date).day ?? 0
        return max(0, diff)
    }

    private func isDueSoon(_ value: String) -> Bool {
        daysUntil(value) <= 7
    }

    private func categoryRows(_ categories: [String: Double]) -> [CategoryRow] {
        let rows = categories.sorted { $0.key < $1.key }
        let maxValue = max(1, rows.map { $0.value }.max() ?? 1)
        return rows.map { (name, amount) in
            CategoryRow(name: name, amount: amount, percent: amount / maxValue)
        }
    }
}

private struct CategoryRow: Identifiable {
    let id = UUID()
    let name: String
    let amount: Double
    let percent: Double
}
