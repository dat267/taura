import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "$lib/types";

export async function greet(name: string): Promise<string> {
  return invoke<string>("greet", { name });
}

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("get_settings");
}

export async function updateSettings(settings: AppSettings): Promise<void> {
  return invoke<void>("update_settings", { settings });
}

export async function resetSettings(): Promise<AppSettings> {
  return invoke<AppSettings>("reset_settings");
}

export async function getAppVersion(): Promise<string> {
  return invoke<string>("greet", { name: "" }).then(
    () => "",
    () => "",
  );
}
