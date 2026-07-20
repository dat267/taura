<script lang="ts">
  import { page } from "$app/stores";
  import { toggleSidebar, getAppSettings } from "$lib/stores/settings.svelte";

  const { settings } = getAppSettings();

  type NavItem = {
    href: string;
    label: string;
    icon: string;
  };

  const navItems: NavItem[] = [
    { href: "/", label: "Home", icon: "⌂" },
    { href: "/settings", label: "Settings", icon: "⚙" },
    { href: "/about", label: "About", icon: "ℹ" },
  ];

  function isActive(href: string): boolean {
    return $page.url.pathname === href;
  }

</script>

<aside
  class="sidebar"
  class:collapsed={settings.sidebarCollapsed}
  style="width: {settings.sidebarCollapsed ? '0px' : settings.sidebarWidth + 'px'}"
  role="navigation"
  aria-label="Main navigation"
>
  <div class="sidebar-inner">
    <div class="sidebar-header">
      <span class="app-name">taura</span>
      <button class="collapse-btn" onclick={toggleSidebar} title="Collapse sidebar">
        ◀
      </button>
    </div>

    <nav class="nav-list">
      {#each navItems as item}
        <a
          href={item.href}
          class="nav-item"
          class:active={isActive(item.href)}
          data-sveltekit-preload-data="off"
        >
          <span class="nav-icon">{item.icon}</span>
          <span class="nav-label">{item.label}</span>
        </a>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <span class="version">v{__APP_VERSION__}</span>
    </div>
  </div>
</aside>

{#if settings.sidebarCollapsed}
  <button class="sidebar-reveal" onclick={toggleSidebar} title="Show sidebar">
    ▶
  </button>
{/if}

<style>
  .sidebar {
    position: relative;
    height: 100%;
    background: var(--bg-sidebar);
    border-right: 1px solid var(--border-color);
    overflow: hidden;
    transition: width var(--transition-normal);
    flex-shrink: 0;
  }

  .sidebar-inner {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 240px;
    padding: 8px;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 4px;
    margin-bottom: 16px;
  }

  .app-name {
    font-weight: 700;
    font-size: 16px;
    letter-spacing: 0.02em;
  }

  .collapse-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: var(--border-radius);
    font-size: 12px;
    transition: background var(--transition-fast);
  }

  .collapse-btn:hover {
    background: var(--bg-tertiary);
  }

  .nav-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    border-radius: var(--border-radius);
    color: var(--text-primary);
    text-decoration: none;
    transition: background var(--transition-fast);
  }

  .nav-item:hover {
    background: var(--bg-tertiary);
  }

  .nav-item.active {
    background: var(--accent-bg);
    color: var(--accent);
    font-weight: 500;
  }

  .nav-icon {
    font-size: 16px;
    width: 20px;
    text-align: center;
  }

  .nav-label {
    font-size: 14px;
  }

  .sidebar-footer {
    padding: 8px 4px;
    border-top: 1px solid var(--border-color);
    margin-top: 8px;
  }

  .version {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .sidebar-reveal {
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    width: 24px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-sidebar);
    border: 1px solid var(--border-color);
    border-left: none;
    border-radius: 0 var(--border-radius) var(--border-radius) 0;
    font-size: 12px;
    z-index: 10;
    transition: background var(--transition-fast);
  }

  .sidebar-reveal:hover {
    background: var(--bg-tertiary);
  }
</style>
