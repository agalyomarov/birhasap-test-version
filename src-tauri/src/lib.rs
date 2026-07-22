mod commands;
mod database;
mod entities;
mod responses;
mod state;
use tauri::Manager;

use commands::prelude::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![auth_login_command])
        .setup(|app: &mut tauri::App| {
            let app_data_dir = app
                .path()
                .resolve("", tauri::path::BaseDirectory::AppLocalData)?;
            let db_pool = tauri::async_runtime::block_on(database::init(app_data_dir))?;
            let state = state::AppState { db: db_pool };
            app.manage(state);

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
