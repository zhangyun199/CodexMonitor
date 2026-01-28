import type { ReactNode } from "react";
import type { LifeDomain, LifeTimeRange } from "../types";
import { DeliveryDashboard } from "./domains/DeliveryDashboard";
import { MediaDashboard } from "./domains/MediaDashboard";

type LifeWorkspaceViewProps = {
  workspaceId: string | null;
  activeDomain: LifeDomain | null;
  timeRange: LifeTimeRange;
  onTimeRangeChange: (range: LifeTimeRange) => void;
  onBackToChat: () => void;
  messagesNode: ReactNode;
};

export function LifeWorkspaceView({
  workspaceId,
  activeDomain,
  timeRange,
  onTimeRangeChange,
  onBackToChat,
  messagesNode,
}: LifeWorkspaceViewProps) {
  if (!activeDomain) {
    return <div className="life-chat-layer">{messagesNode}</div>;
  }

  return (
    <div className="life-workspace">
      <div className="life-dashboard-toolbar">
        <button type="button" className="ghost" onClick={onBackToChat}>
          ← Back to Chat
        </button>
      </div>
      {activeDomain === "delivery" ? (
        <DeliveryDashboard
          workspaceId={workspaceId}
          range={timeRange}
          onRangeChange={onTimeRangeChange}
        />
      ) : activeDomain === "media" ? (
        <MediaDashboard
          workspaceId={workspaceId}
          range={timeRange}
          onRangeChange={onTimeRangeChange}
        />
      ) : (
        <div className="life-dashboard-status">Dashboard coming soon.</div>
      )}
    </div>
  );
}
