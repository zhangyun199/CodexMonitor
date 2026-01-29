import type { LifeTimeRange } from "../../types";

type TimeRangeSelectorProps = {
  value: LifeTimeRange;
  onChange: (range: LifeTimeRange) => void;
};

const ranges: Array<{ id: LifeTimeRange; label: string }> = [
  { id: "today", label: "Today" },
  { id: "week", label: "Week" },
  { id: "month", label: "Month" },
  { id: "lifetime", label: "Life" },
];

export function TimeRangeSelector({ value, onChange }: TimeRangeSelectorProps) {
  return (
    <div className="life-segment-control">
      {ranges.map((range) => (
        <button
          key={range.id}
          type="button"
          className={`life-segment-button${value === range.id ? " is-active" : ""}`}
          onClick={() => onChange(range.id)}
        >
          {range.label}
        </button>
      ))}
    </div>
  );
}
