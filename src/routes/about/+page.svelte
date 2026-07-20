<script lang="ts">
  import { getAppSettings } from "$lib/stores/settings.svelte";

  const { settings } = getAppSettings();

  let currentTheme = $derived(settings.theme);
</script>

<div class="page">
  <h1>About</h1>

  <div class="card">
    <div class="info-grid">
      <div class="info-row">
        <span class="info-label">App</span>
        <span class="info-value">taura</span>
      </div>
      <div class="info-row">
        <span class="info-label">Version</span>
        <span class="info-value">0.1.0</span>
      </div>
      <div class="info-row">
        <span class="info-label">Framework</span>
        <span class="info-value">Tauri v2 + Svelte 5</span>
      </div>
      <div class="info-row">
        <span class="info-label">Theme</span>
        <span class="info-value">{currentTheme}</span>
      </div>
    </div>
  </div>

  <div class="card">
    <h2>Keyboard Shortcuts</h2>
    <div class="shortcuts">
      {#each [
        { keys: "Ctrl/Cmd + B", action: "Toggle sidebar" },
        { keys: "Ctrl/Cmd + Q", action: "Quit app" },
        { keys: "Ctrl/Cmd + ,", action: "Open settings" },
      ] as { keys, action }}
        <div class="shortcut-row">
          <kbd>{keys}</kbd>
          <span>{action}</span>
        </div>
      {/each}
    </div>
  </div>

  <div class="card">
    <h2>Tech Stack</h2>
    <ul class="stack-list">
      <li><strong>Backend:</strong> Rust + Tauri v2</li>
      <li><strong>Frontend:</strong> Svelte 5 + TypeScript</li>
      <li><strong>Build:</strong> Vite + Vitest</li>
      <li><strong>Plugins:</strong> opener, dialog, notification, updater, clipboard</li>
    </ul>
  </div>
</div>

<style>
  .page {
    max-width: 640px;
    margin: 0 auto;
  }

  h1 {
    font-size: 28px;
    font-weight: 700;
    margin-bottom: 24px;
  }

  .card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    padding: 24px;
    margin-bottom: 16px;
  }

  .card h2 {
    font-size: 16px;
    font-weight: 600;
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid var(--border-color);
  }

  .info-grid {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .info-label {
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 500;
  }

  .info-value {
    font-family: var(--font-mono);
    font-size: 13px;
  }

  .shortcuts {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  kbd {
    display: inline-block;
    padding: 2px 8px;
    font-family: var(--font-mono);
    font-size: 12px;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    min-width: 120px;
    text-align: center;
  }

  .stack-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .stack-list li {
    font-size: 14px;
    color: var(--text-secondary);
  }
</style>
