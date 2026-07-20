import { listen, emit } from "@tauri-apps/api/event";
import type { TauriEventMap } from "$lib/types";

type Listener<T> = (payload: T) => void;

const listeners = new Map<string, Set<(...args: unknown[]) => void>>();
const unlisteners = new Map<string, () => void>();

export function listenForEvent<K extends keyof TauriEventMap>(
  event: K,
  handler: Listener<TauriEventMap[K]>,
): () => void {
  const key = event as string;

  if (!listeners.has(key)) {
    listeners.set(key, new Set());
  }

  const handlerSet = listeners.get(key)!;
  handlerSet.add(handler as (...args: unknown[]) => void);

  if (!unlisteners.has(key)) {
    const unlisten = listen<unknown>(key, (event) => {
      const handlers = listeners.get(key);
      if (handlers) {
        for (const h of handlers) {
          h(event.payload);
        }
      }
    });
    unlisteners.set(
      key,
      () => {
        unlisten.then((fn) => fn());
      },
    );
  }

  return () => {
    handlerSet.delete(handler as (...args: unknown[]) => void);
    if (handlerSet.size === 0) {
      unlisteners.get(key)?.();
      unlisteners.delete(key);
      listeners.delete(key);
    }
  };
}

export function emitEvent<K extends keyof TauriEventMap>(
  event: K,
  payload: TauriEventMap[K],
): void {
  emit(event as string, payload);
}
