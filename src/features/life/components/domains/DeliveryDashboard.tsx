import { useMemo } from "react";
import type { LifeTimeRange } from "../../types";
import { useDeliveryDashboard } from "../../hooks/useDeliveryDashboard";
import { StatCard } from "../shared/StatCard";
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

  const earnings = useMemo(
    () => (stats ? `$${stats.totalEarnings.toFixed(2)}` : "--"),
    [stats],
  );
  const ordersCount = stats ? String(stats.orderCount) : "--";
  const hourlyRate = stats ? `$${stats.hourlyRate.toFixed(2)}` : "--";
  const perMile = stats ? `$${stats.perMileRate.toFixed(2)}` : "--";
  const arLabel =
    stats?.startingAr !== undefined && stats?.endingAr !== undefined
      ? `${stats.startingAr}% → ${stats.endingAr}%`
      : "--";
  const whales =
    stats?.whaleCatches !== undefined ? String(stats.whaleCatches) : "--";

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
          <div className="life-stat-grid">
            <StatCard label="Earnings" value={earnings} />
            <StatCard label="Orders" value={ordersCount} />
            <StatCard label="$/hr" value={hourlyRate} />
            <StatCard label="$/mi" value={perMile} />
            <StatCard label="AR" value={arLabel} />
            <StatCard label="Whales" value={whales} />
          </div>

          <section className="life-section">
            <div className="life-section-title">Top Merchants</div>
            {topMerchants.length ? (
              <div className="life-merchant-grid">
                {topMerchants.map((merchant) => (
                  <div key={merchant.merchantName} className="life-merchant-card">
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
            <div className="life-section-title">Orders</div>
            {orders.length ? (
              <div className="life-table">
                <div className="life-table-row life-table-header">
                  <div>Time</div>
                  <div>Merchant</div>
                  <div>Payout</div>
                  <div>Miles</div>
                  <div>App</div>
                  <div>Notes</div>
                </div>
                {orders.map((order) => (
                  <div key={order.id} className="life-table-row">
                    <div>{order.startedAt.slice(11, 16)}</div>
                    <div>{order.merchantName}</div>
                    <div>${order.payout.toFixed(2)}</div>
                    <div>{order.miles ? order.miles.toFixed(1) : "--"}</div>
                    <div>{order.platform ?? "--"}</div>
                    <div>{order.notes ?? ""}</div>
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
