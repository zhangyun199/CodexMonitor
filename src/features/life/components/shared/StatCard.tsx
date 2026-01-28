type StatCardProps = {
  label: string;
  value: string;
  subLabel?: string;
};

export function StatCard({ label, value, subLabel }: StatCardProps) {
  return (
    <div className="life-stat-card">
      <div className="life-stat-label">{label}</div>
      <div className="life-stat-value">{value}</div>
      {subLabel ? <div className="life-stat-sub">{subLabel}</div> : null}
    </div>
  );
}
