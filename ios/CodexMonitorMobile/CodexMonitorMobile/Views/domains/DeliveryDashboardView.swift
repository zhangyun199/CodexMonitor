import SwiftUI
import CodexMonitorModels

struct DeliveryDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                HStack(spacing: 12) {
                    StatCardView(title: "Earnings", value: currency(store.deliveryDashboard?.stats.totalEarnings))
                    StatCardView(title: "Orders", value: "\(store.deliveryDashboard?.stats.orderCount ?? 0)")
                    StatCardView(title: "$/hr", value: number(store.deliveryDashboard?.stats.hourlyRate))
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

                if let merchants = store.deliveryDashboard?.topMerchants, !merchants.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Top Merchants")
                            .font(.headline)
                        ForEach(merchants, id: \.merchantName) { merchant in
                            VStack(alignment: .leading, spacing: 4) {
                                Text(merchant.merchantName)
                                    .font(.subheadline)
                                Text("\(merchant.orderCount) orders · $\(merchant.totalEarnings, specifier: "%.2f")")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Divider()
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let orders = store.deliveryDashboard?.orders, !orders.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Orders")
                            .font(.headline)
                        ForEach(orders) { order in
                            VStack(alignment: .leading, spacing: 4) {
                                Text(order.merchantName)
                                    .font(.subheadline)
                                Text(order.startedAt)
                                    .font(.caption2)
                                    .foregroundStyle(.secondary)
                                HStack {
                                    Text("$\(order.payout, specifier: "%.2f")")
                                    if let miles = order.miles {
                                        Text("· \(miles, specifier: "%.1f") mi")
                                    }
                                }
                                .font(.caption)
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
            await store.fetchDeliveryDashboard(range: timeRange)
        }
    }

    private func currency(_ value: Double?) -> String {
        guard let value else { return "--" }
        return value.formatted(.currency(code: "USD"))
    }

    private func number(_ value: Double?) -> String {
        guard let value else { return "--" }
        return String(format: "$%.2f", value)
    }
}
