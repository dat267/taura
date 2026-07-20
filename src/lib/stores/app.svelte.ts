import { browser } from "$app/environment";
import { listenForEvent } from "$lib/tauri/events";
import { goto } from "$app/navigation";

let currentRoute = $state("");

export function getAppState() {
  return {
    get currentRoute() {
      return currentRoute;
    },
    set currentRoute(value: string) {
      currentRoute = value;
    },
  };
}

export function initAppListeners(): () => void {
  if (!browser) return () => {};

  const unlisteners: (() => void)[] = [];

  unlisteners.push(
    listenForEvent("navigate", (path) => {
      goto(path);
    }),
  );

  unlisteners.push(
    listenForEvent("toggle-sidebar", () => {
    }),
  );

  return () => {
    unlisteners.forEach((fn) => fn());
  };
}
