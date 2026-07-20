use tauri::AppHandle;
use tracing::{error, info};

use crate::error::{AppError, AppResult};

#[tauri::command]
pub fn greet(app: AppHandle, name: &str) -> AppResult<String> {
    info!("greet called with name: {name}");

    let _ = app;

    let greeting = format!(
        "Hello, {name}! You've been greeted from Rust (taura v{})",
        env!("CARGO_PKG_VERSION")
    );

    if name.trim().is_empty() {
        error!("greet called with empty name");
        return Err(AppError::Message("Name cannot be empty".into()));
    }

    Ok(greeting)
}
