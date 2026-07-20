<script lang="ts">
  import { getAppSettings, setTheme, saveSettings } from "$lib/stores/settings.svelte";
  import type { Theme } from "$lib/types";

  const { settings } = getAppSettings();

  const themeOptions: { value: Theme; label: string }[] = [
    { value: "light", label: "Light" },
    { value: "dark", label: "Dark" },
    { value: "system", label: "Follow system" },
  ];
</script>

<div class="page">
  <h1>Settings</h1>
  <p class="subtitle">Application preferences are persisted to disk automatically.</p>

  <section class="section">
    <h2>Appearance</h2>

    <div class="setting">
      <label for="theme-select">Theme</label>
      <div class="theme-options" role="radiogroup" aria-label="Theme selection">
        {#each themeOptions as opt}
          <button
            class="theme-btn"
            class:active={settings.theme === opt.value}
            onclick={() => setTheme(opt.value)}
            role="radio"
            aria-checked={settings.theme === opt.value}
          >
            {opt.label}
          </button>
        {/each}
      </div>
    </div>
  </section>

  <section class="section">
    <h2>Layout</h2>

    <div class="setting">
      <label for="sidebar-width">Sidebar width (px)</label>
      <div class="input-row">
        <input
          id="sidebar-width"
          type="range"
          min={160}
          max={400}
          step={8}
          bind:value={settings.sidebarWidth}
          oninput={() => saveSettings({ sidebarWidth: settings.sidebarWidth })}
        />
        <span class="value">{Math.round(settings.sidebarWidth)}</span>
      </div>
    </div>

    <div class="setting">
      <label class="checkbox-label">
        <input
          type="checkbox"
          checked={settings.sidebarCollapsed}
          onchange={() => saveSettings({ sidebarCollapsed: !settings.sidebarCollapsed })}
        />
        Collapse sidebar by default
      </label>
    </div>
  </section>
</div>

<style>
  .page {
    max-width: 640px;
    margin: 0 auto;
  }

  h1 {
    font-size: 28px;
    font-weight: 700;
    margin-bottom: 4px;
  }

  .subtitle {
    color: var(--text-secondary);
    margin-bottom: 32px;
    font-size: 14px;
  }

  .section {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    padding: 24px;
    margin-bottom: 16px;
  }

  .section h2 {
    font-size: 16px;
    font-weight: 600;
    margin-bottom: 20px;
    padding-bottom: 12px;
    border-bottom: 1px solid var(--border-color);
  }

  .setting {
    margin-bottom: 20px;
  }

  .setting:last-child {
    margin-bottom: 0;
  }

  .setting > label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    margin-bottom: 8px;
  }

  .theme-options {
    display: flex;
    gap: 8px;
  }

  .theme-btn {
    padding: 8px 20px;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    font-size: 13px;
    transition: all var(--transition-fast);
  }

  .theme-btn:hover {
    border-color: var(--accent);
  }

  .theme-btn.active {
    background: var(--accent);
    color: white;
    border-color: var(--accent);
  }

  .input-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .input-row input[type="range"] {
    flex: 1;
    padding: 0;
    border: none;
    background: none;
    box-shadow: none;
    accent-color: var(--accent);
  }

  .value {
    font-family: var(--font-mono);
    font-size: 13px;
    color: var(--text-secondary);
    min-width: 32px;
    text-align: right;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    color: var(--text-primary) !important;
    font-size: 14px !important;
  }

  .checkbox-label input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: var(--accent);
  }
</style>
