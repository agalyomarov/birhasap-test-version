mod bootstrap;
mod commands;

use bootstrap::backend::BackendManager;
use tauri::{Manager, RunEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(BackendManager::new())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::backend::backend_start,
            commands::backend::backend_stop,
            commands::backend::backend_restart,
            commands::backend::backend_status,
            commands::backend::backend_port,
            commands::backend::backend_data_dir,
        ])
        .setup(|app: &mut tauri::App| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            } else {
                // Первый запуск backend
                let backend = app.state::<BackendManager>();
                backend.start(app.handle())?;
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            RunEvent::ExitRequested { .. } | RunEvent::Exit => {
                let backend = app_handle.state::<BackendManager>();

                if let Err(e) = backend.stop() {
                    eprintln!("Backend stop error: {}", e);
                }
            }

            _ => {}
        })
}
