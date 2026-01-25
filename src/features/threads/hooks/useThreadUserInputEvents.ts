import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import type { RequestUserInputRequest } from "../../../types";

export function useThreadUserInputEvents(
  onRequest: (request: RequestUserInputRequest) => void,
) {
  useEffect(() => {
    const unlisten = listen<RequestUserInputRequest>(
      "item/tool/requestUserInput",
      (event) => {
        onRequest(event.payload);
      },
    );

    return () => {
      unlisten.then((handler) => handler());
    };
  }, [onRequest]);
}
