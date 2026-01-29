import type { Bill, LifeTimeRange } from "../../types";
import { useFinanceDashboard } from "../../hooks/useFinanceDashboard";
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
  const dueSoonCount = bills.filter((bill) => isDueSoon(bill.nextDueDate)).length;

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
          <section className="life-card">
            <div className="life-section-title">Monthly Summary</div>
            <div className="finance-summary-grid">
              <div className="finance-summary-item">
                <div className="finance-summary-label">Total Due</div>
                <div className="finance-summary-value">
                  {stats ? formatCurrency(stats.monthlyTotal) : "--"}
                </div>
              </div>
              <div className="finance-summary-item">
                <div className="finance-summary-label">Due Soon</div>
                <div className="finance-summary-value">
                  {stats ? String(dueSoonCount) : "--"}
                </div>
              </div>
              <div className="finance-summary-item">
                <div className="finance-summary-label">Auto-Pay</div>
                <div className="finance-summary-value">
                  {stats ? String(stats.autoPayCount) : "--"}
                </div>
              </div>
            </div>
          </section>

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
            <div className="life-section-title">Spending by Category</div>
            {Object.keys(categories).length ? (
              <CategoryBars categories={categories} />
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
  const dueSoon = isDueSoon(bill.nextDueDate);
  const daysRemaining = daysUntil(bill.nextDueDate);
  return (
    <div className="finance-bill-row">
      <span className={`finance-bill-dot${dueSoon ? " is-due-soon" : ""}`} />
      <div>
        <div className="life-list-title">
          {dueDate} {bill.autoPay ? "🔄" : "•"} {bill.name}
        </div>
        <div className="finance-bill-meta">
          {formatCurrency(bill.amount)} · {bill.frequency}
          {daysRemaining !== null ? ` · ${daysRemaining}d` : ""}
        </div>
      </div>
    </div>
  );
}

function CategoryBars({ categories }: { categories: Record<string, number> }) {
  const entries = Object.entries(categories);
  const maxValue = Math.max(1, ...entries.map(([, value]) => value ?? 0));
  return (
    <div className="life-bar-chart">
      {entries.map(([category, amount]) => (
        <div key={category} className="life-bar-row">
          <div className="life-bar-label">{category}</div>
          <div className="life-bar-track">
            <div
              className="life-bar-fill"
              style={{ width: `${(amount / maxValue) * 100}%` }}
            />
          </div>
          <div className="life-bar-value">{formatCurrency(amount)}</div>
        </div>
      ))}
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

function isDueSoon(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return false;
  const now = new Date();
  const diff = date.getTime() - now.getTime();
  const days = diff / (1000 * 60 * 60 * 24);
  return days >= 0 && days <= 7;
}

function daysUntil(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return null;
  const now = new Date();
  const diff = date.getTime() - now.getTime();
  return Math.max(0, Math.ceil(diff / (1000 * 60 * 60 * 24)));
}
