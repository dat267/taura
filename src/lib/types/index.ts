export type Theme = "light" | "dark" | "system";

export interface AppSettings {
  theme: Theme;
  sidebarCollapsed: boolean;
  sidebarWidth: number;
}

export type NavigatePayload = string;

export interface TauriEventMap {
  navigate: NavigatePayload;
  "toggle-sidebar": void;
}
