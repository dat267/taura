mod commands;
mod error;
mod logging;
mod state;

use state::AppState;
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder};
use tauri::{Emitter, Manager, Wry};

fn build_menu(app: &tauri::AppHandle) -> Result<tauri::menu::Menu<Wry>, tauri::Error> {
    let settings = MenuItemBuilder::with_id("settings", "Settings\u{2026}").build(app)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit")
        .accelerator("CmdOrCtrl+Q")
        .build(app)?;

    let file = SubmenuBuilder::new(app, "File")
        .item(&settings)
        .item(&separator)
        .item(&quit)
        .build()?;

    let undo = MenuItemBuilder::with_id("undo", "Undo")
        .accelerator("CmdOrCtrl+Z")
        .build(app)?;
    let redo = MenuItemBuilder::with_id("redo", "Redo")
        .accelerator("CmdOrCtrl+Shift+Z")
        .build(app)?;
    let edit_sep = PredefinedMenuItem::separator(app)?;
    let cut = MenuItemBuilder::with_id("cut", "Cut")
        .accelerator("CmdOrCtrl+X")
        .build(app)?;
    let copy = MenuItemBuilder::with_id("copy", "Copy")
        .accelerator("CmdOrCtrl+C")
        .build(app)?;
    let paste = MenuItemBuilder::with_id("paste", "Paste")
        .accelerator("CmdOrCtrl+V")
        .build(app)?;

    let edit = SubmenuBuilder::new(app, "Edit")
        .item(&undo)
        .item(&redo)
        .item(&edit_sep)
        .item(&cut)
        .item(&copy)
        .item(&paste)
        .build()?;

    let toggle_sidebar = MenuItemBuilder::with_id("toggle_sidebar", "Toggle Sidebar")
        .accelerator("CmdOrCtrl+B")
        .build(app)?;

    let view = SubmenuBuilder::new(app, "View")
        .item(&toggle_sidebar)
        .build()?;

    let about = MenuItemBuilder::with_id("about", "About taura").build(app)?;

    let help = SubmenuBuilder::new(app, "Help")
        .item(&about)
        .build()?;

    let menu = MenuBuilder::new(app)
        .item(&file)
        .item(&edit)
        .item(&view)
        .item(&help)
        .build()?;

    Ok(menu)
}

fn handle_menu_event(app: &tauri::AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "settings" => {
            let _ = app.emit("navigate", "/settings");
        }
        "about" => {
            let _ = app.emit("navigate", "/about");
        }
        "toggle_sidebar" => {
            let _ = app.emit("toggle-sidebar", ());
        }
        "quit" => {
            app.exit(0);
        }
        id => {
            tracing::debug!("Unhandled menu event: {id}");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logging::init();

    tracing::info!("Starting taura v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let app_state = AppState::new(app)?;
            app.manage(app_state);
            tracing::info!("App initialized");

            let menu = build_menu(app.handle())?;
            app.set_menu(menu)?;
            app.on_menu_event(handle_menu_event);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet::greet,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::reset_settings,
        ])
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                tracing::info!("Window close requested");
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
