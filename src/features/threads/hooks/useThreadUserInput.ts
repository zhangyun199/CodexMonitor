import { useCallback, useState } from "react";
import type { RequestUserInputRequest } from "../../../types";

export function useThreadUserInput() {
  const [pendingRequests, setPendingRequests] = useState<RequestUserInputRequest[]>([]);

  const addRequest = useCallback((request: RequestUserInputRequest) => {
    setPendingRequests((prev) => {
      const index = prev.findIndex(
        (entry) =>
          entry.request_id === request.request_id &&
          entry.workspace_id === request.workspace_id,
      );
      if (index === -1) {
        return [...prev, request];
      }
      const updated = [...prev];
      updated[index] = request;
      return updated;
    });
  }, []);

  const removeRequest = useCallback((requestId: number | string) => {
    setPendingRequests((prev) => prev.filter((entry) => entry.request_id !== requestId));
  }, []);

  return {
    pendingRequests,
    addRequest,
    removeRequest,
  };
}
