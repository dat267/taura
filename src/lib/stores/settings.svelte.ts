import { browser } from "$app/environment";
import { getSettings, updateSettings } from "$lib/tauri/commands";
import type { AppSettings, Theme } from "$lib/types";

let settings = $state<AppSettings>({
  theme: "system",
  sidebarCollapsed: false,
  sidebarWidth: 240,
});

let loaded = $state(false);

export function getAppSettings() {
  return {
    get settings() {
      return settings;
    },
    get loaded() {
      return loaded;
    },
  };
}

export async function loadSettings(): Promise<void> {
  if (!browser) return;
  try {
    const s = await getSettings();
    settings = s;
  } catch (e) {
    console.warn("Failed to load settings, using defaults:", e);
  } finally {
    loaded = true;
  }
}

export async function saveSettings(partial: Partial<AppSettings>): Promise<void> {
  settings = { ...settings, ...partial };
  if (!browser) return;
  try {
    await updateSettings(settings);
  } catch (e) {
    console.error("Failed to save settings:", e);
  }
}

export function setTheme(theme: Theme): void {
  saveSettings({ theme });

  if (!browser) return;

  const resolved = resolveTheme(theme);
  document.documentElement.setAttribute("data-theme", resolved);
}

export function resolveTheme(theme: Theme): "light" | "dark" {
  if (theme === "system") {
    if (!browser) return "dark";
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  return theme;
}

export function toggleSidebar(): void {
  saveSettings({ sidebarCollapsed: !settings.sidebarCollapsed });
}

if (browser) {
  loadSettings();

  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
    if (settings.theme === "system") {
      document.documentElement.setAttribute(
        "data-theme",
        resolveTheme("system"),
      );
    }
  });
}
