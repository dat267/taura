<script lang="ts">
  import { getAppSettings, setTheme } from "$lib/stores/settings.svelte";
  import type { Theme } from "$lib/types";

  const { settings } = getAppSettings();

  const themes: { value: Theme; label: string; icon: string }[] = [
    { value: "light", label: "Light", icon: "☀" },
    { value: "dark", label: "Dark", icon: "☾" },
    { value: "system", label: "System", icon: "⚙" },
  ];

  function cycleTheme() {
    const current = settings.theme;
    const idx = themes.findIndex((t) => t.value === current);
    const next = themes[(idx + 1) % themes.length]!;
    setTheme(next.value);
  }
</script>

<button class="theme-toggle" onclick={cycleTheme} title="Toggle theme (current: {settings.theme})">
  {#each themes as { value, icon }}
    {#if value === settings.theme}
      <span class="icon">{icon}</span>
    {/if}
  {/each}
</button>

<style>
  .theme-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: var(--border-radius);
    transition: background var(--transition-fast);
  }

  .theme-toggle:hover {
    background: var(--bg-tertiary);
  }

  .icon {
    font-size: 16px;
    line-height: 1;
  }
</style>
