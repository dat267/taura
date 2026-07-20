# taura

Robust baseline template for Tauri v2 + Svelte 5 desktop applications.

## Stack

- **Backend**: Rust + Tauri v2 with typed IPC
- **Frontend**: Svelte 5 + TypeScript (SvelteKit SPA mode)
- **Build**: Vite + Vitest
- **Plugins**: opener, dialog, notification, updater, clipboard-manager, process

## Features

- **Persistent settings** — theme, sidebar state saved to disk via JSON
- **Theming** — Light / Dark / System with CSS variables and auto-switching
- **Native menus** — File, Edit, View, Help with keyboard accelerators
- **Tray-ready** — `tray-icon` feature available in Cargo.toml
- **Typed IPC** — Strongly-typed invoke wrappers for all Rust commands
- **Error handling** — `AppError` enum, `AppResult<T>`, structured logging
- **Layout** — Sidebar navigation with collapse, top bar, content area
- **Pages** — Home, Settings, About
- **Testing** — Vitest (frontend) + `cargo test` (backend)
- **CI** — GitHub Actions: lint, typecheck, test, clippy

## Getting started

```bash
npm install
npm run tauri dev
```

## Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Start Vite dev server |
| `npm run build` | Build frontend |
| `npm test` | Run Vitest |
| `npm run check` | Svelte type-check |
| `npm run lint` | Prettier check |
| `npm run format` | Prettier write |
| `npm run tauri dev` | Launch Tauri dev |
| `npm run tauri build` | Build release bundle |
| `cargo test` | Run Rust tests |
| `cargo clippy` | Rust lint |

## Project structure

```
src/              Frontend (TypeScript + Svelte 5)
├── lib/
│   ├── components/   Reusable Svelte components
│   ├── stores/       Svelte 5 rune-based stores (.svelte.ts)
│   ├── tauri/        Typed invoke wrappers + event helpers
│   ├── types/        Shared TypeScript types
│   └── __tests__/    Vitest tests
├── routes/           SvelteKit pages (Home, Settings, About)
├── app.css           Global CSS variables and resets
├── app.d.ts          Ambient type declarations
└── app.html          HTML shell

src-tauri/        Backend (Rust)
├── src/
│   ├── lib.rs        App setup, plugins, menus
│   ├── main.rs       Entry point
│   ├── commands/     Tauri command handlers
│   ├── error.rs      AppError type
│   ├── logging.rs    Tracing subscriber init
│   └── state.rs      App state + settings persistence
├── capabilities/     Tauri v2 permission capabilities
└── tauri.conf.json   Tauri configuration
```

## Architecture patterns

### Adding a new Tauri command

**Rust** (`src-tauri/src/commands/`):

```rust
#[tauri::command]
pub fn my_command(app: AppHandle, arg: String) -> AppResult<String> {
    info!("my_command called with {arg}");
    Ok(format!("Processed: {arg}"))
}
```

Register in `lib.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    // ...
    commands::my_command::my_command,
])
```

**Frontend** (`src/lib/tauri/commands.ts`):

```typescript
export async function myCommand(arg: string): Promise<string> {
  return invoke<string>("my_command", { arg });
}
```

### Adding a new page

1. Create `src/routes/my-page/+page.svelte`
2. Add nav link in `AppSidebar.svelte`
3. Menu items in `lib.rs` if desired
