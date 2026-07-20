pub mod greet;
pub mod settings;

use crate::error::AppError;
use tauri::{AppHandle, Manager};

pub(crate) fn with_state<T>(
    app: &AppHandle,
    f: impl FnOnce(&crate::state::AppState) -> Result<T, AppError>,
) -> Result<T, AppError> {
    let state = app.state::<crate::state::AppState>();
    f(state.inner())
}
