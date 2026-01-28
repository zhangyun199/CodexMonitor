import type { LifeDomain } from "../types";
import { LIFE_DOMAINS } from "../types";

type DomainSelectorProps = {
  activeDomain: LifeDomain | null;
  onSelect: (domain: LifeDomain | null) => void;
};

export function DomainSelector({ activeDomain, onSelect }: DomainSelectorProps) {
  return (
    <div className="life-domain-selector">
      <div className="life-domain-selector-header">
        <div className="life-domain-selector-title">Life Domains</div>
        {activeDomain && (
          <button
            type="button"
            className="ghost life-domain-clear"
            onClick={() => onSelect(null)}
          >
            Back to Chat
          </button>
        )}
      </div>
      <div className="life-domain-selector-list">
        {LIFE_DOMAINS.map((domain) => {
          const isActive = activeDomain === domain.id;
          return (
            <button
              key={domain.id}
              type="button"
              className={`life-domain-button${isActive ? " is-active" : ""}`}
              onClick={() => onSelect(domain.id)}
            >
              <span className="life-domain-icon" aria-hidden>
                {domain.icon}
              </span>
              <span className="life-domain-label">{domain.label}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}
