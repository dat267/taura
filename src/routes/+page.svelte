<script lang="ts">
  import { greet } from "$lib/tauri/commands";

  let name = $state("");
  let greetMsg = $state("");
  let error = $state("");
  let loading = $state(false);

  async function handleGreet(e: Event) {
    e.preventDefault();
    if (!name.trim()) return;
    loading = true;
    error = "";
    greetMsg = "";
    try {
      greetMsg = await greet(name);
    } catch (e) {
      error = String(e);
      console.error("greet failed:", e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="page">
  <h1>Welcome to taura</h1>
  <p class="subtitle">Robust Tauri + Svelte 5 baseline template</p>

  <div class="card">
    <h2>Try it out</h2>
    <form onsubmit={handleGreet}>
      <div class="input-row">
        <input
          id="greet-input"
          type="text"
          placeholder="Enter your name…"
          bind:value={name}
          aria-label="Your name"
        />
        <button type="submit" disabled={loading || !name.trim()}>
          {loading ? "Greeting…" : "Greet"}
        </button>
      </div>
    </form>

    {#if greetMsg}
      <div class="result success">
        {greetMsg}
      </div>
    {/if}

    {#if error}
      <div class="result error">
        {error}
      </div>
    {/if}
  </div>

  <div class="features">
    <div class="feature">
      <strong>⌨ Tauri v2</strong>
      <span>Native Rust backend with typed IPC</span>
    </div>
    <div class="feature">
      <strong>⚡ Svelte 5</strong>
      <span>Runes-based reactivity, $state / $derived</span>
    </div>
    <div class="feature">
      <strong>🎨 Theming</strong>
      <span>Light / Dark / System with CSS variables</span>
    </div>
    <div class="feature">
      <strong>🔧 Robust</strong>
      <span>Error handling, tracing, persistent settings</span>
    </div>
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
    margin-bottom: 4px;
  }

  .subtitle {
    color: var(--text-secondary);
    margin-bottom: 32px;
    font-size: 15px;
  }

  .card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    padding: 24px;
    margin-bottom: 24px;
  }

  .card h2 {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 16px;
  }

  .input-row {
    display: flex;
    gap: 8px;
  }

  .input-row input {
    flex: 1;
  }

  .input-row button {
    padding: 0.5em 1.5em;
    background: var(--accent);
    color: white;
    border-radius: var(--border-radius);
    font-weight: 500;
    transition: background var(--transition-fast);
  }

  .input-row button:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .input-row button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .result {
    margin-top: 16px;
    padding: 12px;
    border-radius: var(--border-radius);
    font-size: 14px;
  }

  .result.success {
    background: var(--success-bg);
    color: var(--success);
    border: 1px solid var(--success);
  }

  .result.error {
    background: var(--danger-bg);
    color: var(--danger);
    border: 1px solid var(--danger);
  }

  .features {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: 12px;
  }

  .feature {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 16px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
  }

  .feature strong {
    font-size: 14px;
  }

  .feature span {
    font-size: 12px;
    color: var(--text-secondary);
  }
</style>
