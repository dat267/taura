use tauri::AppHandle;
use tracing::{error, info};

use crate::error::AppResult;
use crate::state::AppSettings;

use super::with_state;

#[tauri::command]
pub fn get_settings(app: AppHandle) -> AppResult<AppSettings> {
    info!("get_settings called");

    let settings = with_state(&app, |state| {
        state.settings.lock().map_err(|e| {
            error!("Failed to lock settings: {e}");
            crate::error::AppError::Message(format!("Lock error: {e}"))
        }).map(|s| s.clone())
    })?;

    Ok(settings)
}

#[tauri::command]
pub fn update_settings(app: AppHandle, settings: AppSettings) -> AppResult<()> {
    info!("update_settings called: {settings:?}");

    with_state(&app, |state| {
        let mut s = state.settings.lock().map_err(|e| {
            error!("Failed to lock settings: {e}");
            crate::error::AppError::Message(format!("Lock error: {e}"))
        })?;
        *s = settings;
        state.save_settings()?;
        info!("Settings updated");
        Ok(())
    })?;

    Ok(())
}

#[tauri::command]
pub fn reset_settings(app: AppHandle) -> AppResult<AppSettings> {
    info!("reset_settings called");

    let default = AppSettings::default();

    with_state(&app, |state| {
        let mut s = state.settings.lock().map_err(|e| {
            error!("Failed to lock settings: {e}");
            crate::error::AppError::Message(format!("Lock error: {e}"))
        })?;
        *s = default.clone();
        state.save_settings()?;
        info!("Settings reset to defaults");
        Ok(())
    })?;

    Ok(default)
}
