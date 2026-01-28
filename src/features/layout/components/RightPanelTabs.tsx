import type { ReactNode } from "react";
import { useLayoutEffect, useMemo, useRef, useState } from "react";
import GitBranch from "lucide-react/dist/esm/icons/git-branch";
import Brain from "lucide-react/dist/esm/icons/brain";
import Globe from "lucide-react/dist/esm/icons/globe";
import Sparkles from "lucide-react/dist/esm/icons/sparkles";
import LayoutDashboard from "lucide-react/dist/esm/icons/layout-dashboard";

export type RightPanelTabId = "git" | "memory" | "browser" | "skills" | "domain";

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
  { id: "domain", label: "Domains", icon: <LayoutDashboard aria-hidden /> },
  { id: "browser", label: "Browser", icon: <Globe aria-hidden /> },
  { id: "skills", label: "Skills", icon: <Sparkles aria-hidden /> },
];

export function RightPanelTabs({ active, onSelect }: RightPanelTabsProps) {
  const wrapperRef = useRef<HTMLDivElement | null>(null);
  const measureRef = useRef<HTMLDivElement | null>(null);
  const [useDropdown, setUseDropdown] = useState(false);
  const activeTab = useMemo(
    () => tabs.find((tab) => tab.id === active) ?? tabs[0],
    [active],
  );

  useLayoutEffect(() => {
    const wrapper = wrapperRef.current;
    const measure = measureRef.current;
    if (!wrapper || !measure) {
      return;
    }
    const update = () => {
      const available = wrapper.clientWidth;
      const required = measure.scrollWidth;
      const shouldUseDropdown = required > available + 4;
      setUseDropdown((prev) =>
        prev === shouldUseDropdown ? prev : shouldUseDropdown,
      );
    };
    update();
    const observer = new ResizeObserver(update);
    observer.observe(wrapper);
    observer.observe(measure);
    return () => observer.disconnect();
  }, [active]);

  const renderTabs = (extraClassName = "") => (
    <div
      className={`panel-tabs${extraClassName ? ` ${extraClassName}` : ""}`}
      role="tablist"
      aria-label="Right panel"
    >
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

  return (
    <div className="right-panel-tabs" ref={wrapperRef}>
      {useDropdown ? (
        <div className="composer-select-wrap right-panel-select-wrap">
          <span className="composer-icon" aria-hidden>
            {activeTab.icon}
          </span>
          <select
            className="composer-select right-panel-select"
            value={activeTab.id}
            onChange={(event) => onSelect(event.target.value as RightPanelTabId)}
            aria-label="Right panel"
          >
            {tabs.map((tab) => (
              <option key={tab.id} value={tab.id}>
                {tab.label}
              </option>
            ))}
          </select>
        </div>
      ) : (
        renderTabs()
      )}
      <div className="panel-tabs-measure" ref={measureRef} aria-hidden>
        {renderTabs("panel-tabs-measure-inner")}
      </div>
    </div>
  );
}
