<script lang="ts">
  import "../app.css";
  import { browser } from "$app/environment";
  import { loadSettings, getAppSettings, resolveTheme, toggleSidebar } from "$lib/stores/settings.svelte";
  import { initAppListeners } from "$lib/stores/app.svelte";
  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import ThemeToggle from "$lib/components/ThemeToggle.svelte";
  import { onMount } from "svelte";

  let { children } = $props();

  const { settings, loaded } = getAppSettings();

  let themeResolved = $derived(resolveTheme(settings.theme));

  $effect(() => {
    if (browser) {
      document.documentElement.setAttribute("data-theme", themeResolved);
    }
  });

  let sidebarExpanded = $derived(!settings.sidebarCollapsed);

  onMount(() => {
    loadSettings();
    const cleanup = initAppListeners();
    return cleanup;
  });
</script>

{#if !loaded}
  <div class="loading-screen">
    <div class="spinner"></div>
    <p>Loading taura…</p>
  </div>
{:else}
  <div class="layout" class:sidebar-open={sidebarExpanded}>
    <AppSidebar />

    <div class="main-area">
      <header class="top-bar">
        <div class="top-bar-left">
          {#if sidebarExpanded}
            <button class="menu-btn" onclick={toggleSidebar} title="Toggle sidebar">
              ☰
            </button>
          {/if}
        </div>
        <div class="top-bar-right">
          <ThemeToggle />
        </div>
      </header>

      <main class="content">
        {@render children()}
      </main>
    </div>
  </div>
{/if}

<style>
  .loading-screen {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    gap: 16px;
    color: var(--text-secondary);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border-color);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .layout {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .main-area {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  .top-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: var(--header-height);
    padding: 0 16px;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-primary);
    flex-shrink: 0;
  }

  .top-bar-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .top-bar-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .menu-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: var(--border-radius);
    font-size: 18px;
    transition: background var(--transition-fast);
  }

  .menu-btn:hover {
    background: var(--bg-tertiary);
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
  }
</style>
