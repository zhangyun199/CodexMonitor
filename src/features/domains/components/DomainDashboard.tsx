import type { ReactNode } from "react";
import type { DomainTrendSnapshot } from "../../../types";

type StatCardProps = {
  label: string;
  value: string;
  helper?: string | null;
};

function StatCard({ label, value, helper }: StatCardProps) {
  return (
    <div className="domain-card">
      <div className="domain-card-label">{label}</div>
      <div className="domain-card-value">{value}</div>
      {helper && <div className="domain-card-helper">{helper}</div>}
    </div>
  );
}

type DomainSectionProps = {
  title: string;
  children: ReactNode;
};

function DomainSection({ title, children }: DomainSectionProps) {
  return (
    <section className="domain-section">
      <div className="domain-section-title">{title}</div>
      {children}
    </section>
  );
}

export type DomainDashboardProps = {
  snapshot: DomainTrendSnapshot;
};

export function DomainDashboard({ snapshot }: DomainDashboardProps) {
  return (
    <div className="domain-dashboard">
      <DomainSection title="Highlights">
        {snapshot.cards.length > 0 ? (
          <div className="domain-card-grid">
            {snapshot.cards.map((card) => (
              <StatCard
                key={card.id}
                label={card.label}
                value={card.value}
                helper={card.subLabel ?? null}
              />
            ))}
          </div>
        ) : (
          <div className="domain-empty">No summary available.</div>
        )}
      </DomainSection>

      {snapshot.lists.map((list) => (
        <DomainSection key={list.id} title={list.title}>
          {list.items.length > 0 ? (
            <div className="domain-list">
              {list.items.map((item, index) => (
                <div key={`${list.id}-${index}`} className="domain-list-row">
                  <div className="domain-list-label">{item.label}</div>
                  <div className="domain-list-value">{item.value}</div>
                  {item.subLabel && (
                    <div className="domain-list-sub">{item.subLabel}</div>
                  )}
                </div>
              ))}
            </div>
          ) : (
            <div className="domain-empty">No data.</div>
          )}
        </DomainSection>
      ))}
    </div>
  );
}
