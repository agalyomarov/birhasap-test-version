use tauri::{AppHandle, State};

use crate::bootstrap::backend::BackendManager;

#[tauri::command]
pub fn backend_start(app: AppHandle, backend: State<'_, BackendManager>) -> Result<(), String> {
    backend.start(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn backend_stop(backend: State<'_, BackendManager>) -> Result<(), String> {
    backend.stop().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn backend_restart(app: AppHandle, backend: State<'_, BackendManager>) -> Result<(), String> {
    backend.restart(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn backend_status(backend: State<'_, BackendManager>) -> bool {
    backend.is_running()
}

#[tauri::command]
pub fn backend_port(backend: tauri::State<'_, BackendManager>) -> Result<u16, String> {
    if cfg!(debug_assertions) {
        return Ok(8000);
    }

    backend
        .port()
        .ok_or_else(|| "Backend is not running".to_string())
}

#[tauri::command]
pub fn backend_data_dir(backend: tauri::State<'_, BackendManager>) -> Option<String> {
    backend.data_dir().map(|p| p.to_string_lossy().to_string())
}
