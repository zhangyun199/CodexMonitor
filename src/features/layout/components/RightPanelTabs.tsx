import type { ReactNode } from "react";
import GitBranch from "lucide-react/dist/esm/icons/git-branch";
import Brain from "lucide-react/dist/esm/icons/brain";
import Globe from "lucide-react/dist/esm/icons/globe";
import Sparkles from "lucide-react/dist/esm/icons/sparkles";

export type RightPanelTabId = "git" | "memory" | "browser" | "skills";

type RightPanelTab = {
  id: RightPanelTabId;
  label: string;
  icon: ReactNode;
};

type RightPanelTabsProps = {
  active: RightPanelTabId;
  onSelect: (id: RightPanelTabId) => void;
};

const tabs: RightPanelTab[] = [
  { id: "git", label: "Git", icon: <GitBranch aria-hidden /> },
  { id: "memory", label: "Memory", icon: <Brain aria-hidden /> },
  { id: "browser", label: "Browser", icon: <Globe aria-hidden /> },
  { id: "skills", label: "Skills", icon: <Sparkles aria-hidden /> },
];

export function RightPanelTabs({ active, onSelect }: RightPanelTabsProps) {
  return (
    <div className="panel-tabs" role="tablist" aria-label="Right panel">
      {tabs.map((tab) => {
        const isActive = tab.id === active;
        return (
          <button
            key={tab.id}
            type="button"
            className={`panel-tab${isActive ? " is-active" : ""}`}
            onClick={() => onSelect(tab.id)}
            aria-current={isActive ? "page" : undefined}
            title={tab.label}
          >
            <span className="panel-tab-icon" aria-hidden>
              {tab.icon}
            </span>
            <span className="panel-tab-label">{tab.label}</span>
          </button>
        );
      })}
    </div>
  );
}
