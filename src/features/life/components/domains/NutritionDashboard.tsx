import { useMemo } from "react";
import type { LifeTimeRange, MealEntry } from "../../types";
import { NUTRITION_GOALS } from "../../types";
import { useNutritionDashboard } from "../../hooks/useNutritionDashboard";
import { StatCard } from "../shared/StatCard";
import { TimeRangeSelector } from "../shared/TimeRangeSelector";

const MEAL_EMOJI: Record<MealEntry["mealType"], string> = {
  breakfast: "🌅",
  lunch: "🌞",
  dinner: "🌙",
  snack: "🍪",
};

type NutritionDashboardProps = {
  workspaceId: string | null;
  range: LifeTimeRange;
  onRangeChange: (range: LifeTimeRange) => void;
};

export function NutritionDashboard({
  workspaceId,
  range,
  onRangeChange,
}: NutritionDashboardProps) {
  const { dashboard, loading, error, refresh } = useNutritionDashboard(
    workspaceId,
    range,
  );

  const stats = dashboard?.stats;
  const meals = dashboard?.meals ?? [];
  const trend = dashboard?.weeklyTrend;

  const calories = stats ? stats.calories.toFixed(0) : "--";
  const protein = stats ? `${stats.protein.toFixed(0)}g` : "--";
  const carbs = stats ? `${stats.carbs.toFixed(0)}g` : "--";
  const fat = stats ? `${stats.fat.toFixed(0)}g` : "--";
  const fiber = stats?.fiber ? `${stats.fiber.toFixed(0)}g` : "--";

  const trendRows = useMemo(() => {
    if (!trend) return [];
    const rows = Object.entries(trend).sort(([a], [b]) => a.localeCompare(b));
    const maxValue = Math.max(
      NUTRITION_GOALS.calories,
      ...rows.map(([, value]) => value ?? 0),
    );
    return rows.map(([date, value]) => ({
      date,
      value,
      percent: maxValue > 0 ? (value / maxValue) * 100 : 0,
    }));
  }, [trend]);

  return (
    <div className="life-dashboard life-nutrition-dashboard">
      <div className="life-dashboard-header">
        <div>
          <div className="life-dashboard-title">🍽️ Nutrition Dashboard</div>
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
        <div className="life-dashboard-status">Loading nutrition data…</div>
      )}

      {dashboard ? (
        <>
          <div className="life-stat-grid">
            <StatCard
              label="Calories"
              value={calories}
              subLabel={`goal ${NUTRITION_GOALS.calories}`}
            />
            <StatCard
              label="Protein"
              value={protein}
              subLabel={`goal ${NUTRITION_GOALS.protein}g`}
            />
            <StatCard
              label="Carbs"
              value={carbs}
              subLabel={`goal ${NUTRITION_GOALS.carbs}g`}
            />
            <StatCard
              label="Fat"
              value={fat}
              subLabel={`goal ${NUTRITION_GOALS.fat}g`}
            />
            <StatCard label="Fiber" value={fiber} />
          </div>

          <section className="life-section">
            <div className="life-section-title">Meals</div>
            {meals.length ? (
              <div className="life-list">
                {meals.map((meal) => (
                  <div key={meal.id} className="life-list-item">
                    <div className="life-list-title">
                      {MEAL_EMOJI[meal.mealType]} {meal.description}
                    </div>
                    <div className="life-list-meta">
                      {formatMealTime(meal.timestamp)}
                      {meal.estimatedCalories !== undefined
                        ? ` • ${meal.estimatedCalories.toFixed(0)} cal`
                        : ""}
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No meals logged in this range.</div>
            )}
          </section>

          <section className="life-section">
            <div className="life-section-title">Weekly Trends</div>
            {trendRows.length ? (
              <div className="life-bar-chart">
                {trendRows.map((row) => (
                  <div key={row.date} className="life-bar-row">
                    <div className="life-bar-label">{formatShortDate(row.date)}</div>
                    <div className="life-bar-track">
                      <div
                        className="life-bar-fill"
                        style={{ width: `${row.percent}%` }}
                      />
                    </div>
                    <div className="life-bar-value">{row.value.toFixed(0)} cal</div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="life-dashboard-status">No trends yet.</div>
            )}
          </section>
        </>
      ) : null}
    </div>
  );
}

function formatMealTime(timestamp: string) {
  if (timestamp.length >= 16) {
    return timestamp.slice(11, 16);
  }
  return timestamp;
}

function formatShortDate(value: string) {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return value;
  return date.toLocaleDateString(undefined, { month: "short", day: "numeric" });
}
