import SwiftUI
import CodexMonitorModels

struct DeliveryDashboardView: View {
    @EnvironmentObject private var store: CodexStore
    @Binding var timeRange: LifeTimeRange
    private let goal: Double = 150

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                header
                heroCard
                compactStats

                HStack {
                    TimeRangePicker(selection: $timeRange)
                    Button {
                        Task { await store.fetchDeliveryDashboard(range: timeRange) }
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

                if let merchants = store.deliveryDashboard?.topMerchants, !merchants.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Top Merchants")
                            .font(.headline)
                        ForEach(merchants, id: \.merchantName) { merchant in
                            GlassCardView {
                                VStack(alignment: .leading, spacing: 4) {
                                    Text(merchant.merchantName)
                                        .font(.subheadline)
                                    Text("\(merchant.orderCount) orders · $\(merchant.totalEarnings, specifier: "%.2f")")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }
                            }
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                }

                if let orders = store.deliveryDashboard?.orders, !orders.isEmpty {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Recent Orders")
                            .font(.headline)
                        ForEach(orders.prefix(6)) { order in
                            GlassCardView {
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
                            }
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

    private func arLabel() -> String {
        guard let start = store.deliveryDashboard?.stats.startingAr,
              let end = store.deliveryDashboard?.stats.endingAr else {
            return "--"
        }
        return "\(Int(start))% → \(Int(end))%"
    }

    private func whalesLabel() -> String {
        if let whales = store.deliveryDashboard?.stats.whaleCatches {
            return "\(whales)"
        }
        return "--"
    }

    private var header: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text("🚗 Delivery Dashboard")
                .font(.headline)
            if let meta = store.deliveryDashboard?.meta {
                Text("\(meta.periodStart) → \(meta.periodEnd)")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
    }

    private var heroCard: some View {
        let earnings = store.deliveryDashboard?.stats.totalEarnings ?? 0
        let progress = min(1, earnings / goal)
        return GlassCardView {
            VStack(alignment: .leading, spacing: 12) {
                Text(currency(store.deliveryDashboard?.stats.totalEarnings))
                    .font(.system(size: 44, weight: .bold, design: .rounded))
                    .foregroundStyle(LifeColors.earnings)
                ProgressBarView(progress: progress, color: LifeColors.earnings, height: 8)
                Text("\(Int(progress * 100))% of $\(Int(goal)) goal")
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
    }

    private var compactStats: some View {
        let columns = [GridItem(.adaptive(minimum: 140), spacing: 12)]
        let miles = store.deliveryDashboard?.stats.totalMiles
            ?? store.deliveryDashboard?.orders.reduce(0, { $0 + ($1.miles ?? 0) }) ?? 0

        return LazyVGrid(columns: columns, spacing: 12) {
            GlassCardView {
                StatLine(label: "$/hr", value: number(store.deliveryDashboard?.stats.hourlyRate), color: LifeColors.time)
            }
            GlassCardView {
                StatLine(label: "Orders", value: "\(store.deliveryDashboard?.stats.orderCount ?? 0)", color: LifeColors.count)
            }
            GlassCardView {
                StatLine(label: "Miles", value: miles > 0 ? String(format: "%.1f mi", miles) : "--", color: .white)
            }
            GlassCardView {
                StatLine(label: "$/mi", value: number(store.deliveryDashboard?.stats.perMileRate), color: .white)
            }
            GlassCardView {
                StatLine(label: "AR", value: arLabel(), color: .white)
            }
            GlassCardView {
                StatLine(label: "Whales", value: whalesLabel(), color: .white)
            }
        }
    }
}

private struct StatLine: View {
    let label: String
    let value: String
    let color: Color

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(label)
                .font(.caption)
                .foregroundStyle(.secondary)
            Text(value)
                .font(.headline)
                .foregroundStyle(color)
        }
    }
}
