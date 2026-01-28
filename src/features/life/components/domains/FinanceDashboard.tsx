import type { Bill, LifeTimeRange } from "../../types";
import { useFinanceDashboard } from "../../hooks/useFinanceDashboard";
import { StatCard } from "../shared/StatCard";
import { TimeRangeSelector } from "../shared/TimeRangeSelector";

type FinanceDashboardProps = {
  workspaceId: string | null;
  range: LifeTimeRange;
  onRangeChange: (range: LifeTimeRange) => void;
};

export function FinanceDashboard({
  workspaceId,
  range,
  onRangeChange,
}: FinanceDashboardProps) {
  const { dashboard, loading, error, refresh } = useFinanceDashboard(
    workspaceId,
    range,
  );

  const stats = dashboard?.stats;
  const bills = dashboard?.bills ?? [];
  const categories = dashboard?.byCategory ?? {};
  const statusMessage = dashboard?.statusMessage;

  return (
    <div className="life-dashboard life-finance-dashboard">
      <div className="life-dashboard-header">
        <div>
          <div className="life-dashboard-title">💸 Finance Dashboard</div>
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
        <div className="life-dashboard-status">Loading finance data…</div>
      )}

      {dashboard ? (
        <>
          <div className="life-stat-grid">
            <StatCard
              label="Monthly Total"
              value={stats ? formatCurrency(stats.monthlyTotal) : "--"}
            />
            <StatCard
              label="Due Soon"
              value={stats ? String(stats.dueSoonCount) : "--"}
            />
            <StatCard
              label="Auto-Pay"
              value={stats ? String(stats.autoPayCount) : "--"}
            />
          </div>

          {statusMessage ? (
            <div className="life-dashboard-status life-status-warning">
              {statusMessage}
            </div>
          ) : null}

          <section className="life-section">
            <div className="life-section-title">Upcoming Bills</div>
            {bills.length ? (
              <div className="life-list">
                {bills.map((bill) => (
                  <BillRow key={bill.id} bill={bill} />
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No bills found.</div>
            )}
          </section>

          <section className="life-section">
            <div className="life-section-title">By Category</div>
            {Object.keys(categories).length ? (
              <div className="life-trend-list">
                {Object.entries(categories).map(([category, amount]) => (
                  <div key={category} className="life-trend-row">
                    <span>{category}</span>
                    <span>{formatCurrency(amount)}</span>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No category totals yet.</div>
            )}
          </section>
        </>
      ) : null}
    </div>
  );
}

function BillRow({ bill }: { bill: Bill }) {
  const dueDate = formatShortDate(bill.nextDueDate);
  return (
    <div className="life-list-item">
      <div className="life-list-title">
        {dueDate} {bill.autoPay ? "🔄" : "•"} {bill.name}
      </div>
      <div className="life-list-meta">
        {formatCurrency(bill.amount)} · {bill.frequency}
      </div>
    </div>
  );
}

function formatCurrency(value: number) {
  return value.toLocaleString(undefined, {
    style: "currency",
    currency: "USD",
    maximumFractionDigits: 0,
  });
}

function formatShortDate(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;
  return date.toLocaleDateString(undefined, { month: "short", day: "numeric" });
}
