import { useMemo } from "react";
import type { LifeTimeRange } from "../../types";
import { useDeliveryDashboard } from "../../hooks/useDeliveryDashboard";
import { TimeRangeSelector } from "../shared/TimeRangeSelector";

type DeliveryDashboardProps = {
  workspaceId: string | null;
  range: LifeTimeRange;
  onRangeChange: (range: LifeTimeRange) => void;
};

export function DeliveryDashboard({
  workspaceId,
  range,
  onRangeChange,
}: DeliveryDashboardProps) {
  const { dashboard, loading, error, refresh } = useDeliveryDashboard(
    workspaceId,
    range,
  );

  const stats = dashboard?.stats;
  const orders = dashboard?.orders ?? [];
  const topMerchants = dashboard?.topMerchants ?? [];

  const earningsValue = stats?.totalEarnings ?? 0;
  const earnings = stats ? `$${stats.totalEarnings.toFixed(2)}` : "--";
  const ordersCount = stats ? String(stats.orderCount) : "--";
  const hourlyRate = stats ? `$${stats.hourlyRate.toFixed(2)}` : "--";
  const perMile = stats ? `$${stats.perMileRate.toFixed(2)}` : "--";
  const arLabel =
    stats?.startingAr !== undefined && stats?.endingAr !== undefined
      ? `${stats.startingAr}% → ${stats.endingAr}%`
      : "--";
  const whales =
    stats?.whaleCatches !== undefined ? String(stats.whaleCatches) : "--";
  const totalMiles = useMemo(() => {
    if (stats?.totalMiles !== undefined) {
      return stats.totalMiles;
    }
    return orders.reduce((sum, order) => sum + (order.miles ?? 0), 0);
  }, [orders, stats]);
  const goal = 150;
  const progress = Math.min(100, (earningsValue / goal) * 100);
  const recentOrders = useMemo(
    () => orders.slice(-6).reverse(),
    [orders],
  );

  return (
    <div className="life-dashboard life-delivery-dashboard">
      <div className="life-dashboard-header">
        <div>
          <div className="life-dashboard-title">🚗 Delivery Dashboard</div>
          <div className="life-dashboard-subtitle">
            {dashboard?.meta ? `${dashboard.meta.periodStart} → ${dashboard.meta.periodEnd}` : ""}
          </div>
        </div>
        <div className="life-dashboard-actions">
          <TimeRangeSelector value={range} onChange={onRangeChange} />
          <button
            type="button"
            className="ghost life-refresh-button"
            onClick={() => void refresh()}
            disabled={loading}
          >
            Refresh
          </button>
        </div>
      </div>

      {error && <div className="life-dashboard-error">{error}</div>}
      {loading && !dashboard && (
        <div className="life-dashboard-status">Loading delivery data…</div>
      )}

      {dashboard ? (
        <>
          <section className="life-card delivery-hero">
            <div className="delivery-hero-earnings">{earnings}</div>
            <div className="delivery-goal-progress">
              <div
                className="delivery-goal-progress-fill"
                style={{ width: `${progress}%` }}
              />
            </div>
            <div className="delivery-goal-meta">
              {progress.toFixed(0)}% of ${goal} goal
            </div>
          </section>

          <div className="delivery-compact-stats">
            <div className="life-card">
              <div className="delivery-stat-label">$/hr</div>
              <div className="delivery-stat-value delivery-stat-value--time">
                {hourlyRate}
              </div>
            </div>
            <div className="life-card">
              <div className="delivery-stat-label">Orders</div>
              <div className="delivery-stat-value delivery-stat-value--count">
                {ordersCount}
              </div>
            </div>
            <div className="life-card">
              <div className="delivery-stat-label">Miles</div>
              <div className="delivery-stat-value delivery-stat-value--neutral">
                {totalMiles ? `${totalMiles.toFixed(1)} mi` : "--"}
              </div>
            </div>
            <div className="life-card">
              <div className="delivery-stat-label">$/mi</div>
              <div className="delivery-stat-value delivery-stat-value--neutral">
                {perMile}
              </div>
            </div>
            <div className="life-card">
              <div className="delivery-stat-label">AR</div>
              <div className="delivery-stat-value delivery-stat-value--neutral">
                {arLabel}
              </div>
            </div>
            <div className="life-card">
              <div className="delivery-stat-label">Whales</div>
              <div className="delivery-stat-value delivery-stat-value--neutral">
                {whales}
              </div>
            </div>
          </div>

          <section className="life-section">
            <div className="life-section-title">Top Merchants</div>
            {topMerchants.length ? (
              <div className="life-merchant-grid">
                {topMerchants.map((merchant) => (
                  <div key={merchant.merchantName} className="life-card">
                    <div className="life-merchant-name">{merchant.merchantName}</div>
                    <div className="life-merchant-meta">
                      {merchant.orderCount} orders · ${merchant.totalEarnings.toFixed(2)}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No merchants yet.</div>
            )}
          </section>

          <section className="life-section">
            <div className="life-section-title">Recent Orders</div>
            {recentOrders.length ? (
              <div className="life-list">
                {recentOrders.map((order) => (
                  <div key={order.id} className="life-card">
                    <div className="life-list-title">{order.merchantName}</div>
                    <div className="life-list-meta">
                      {order.startedAt.slice(11, 16)} · ${order.payout.toFixed(2)}
                      {order.miles ? ` · ${order.miles.toFixed(1)} mi` : ""}
                      {order.platform ? ` · ${order.platform}` : ""}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No orders in this range.</div>
            )}
          </section>
        </>
      ) : null}
    </div>
  );
}
