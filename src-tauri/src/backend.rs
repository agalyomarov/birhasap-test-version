use std::sync::Mutex;

use tauri::{AppHandle, Manager};
use tauri_plugin_shell::{process::CommandChild, ShellExt};

pub struct BackendState {
    pub child: Mutex<Option<CommandChild>>,
}

impl Default for BackendState {
    fn default() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }
}

pub fn start(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let command = app.shell().sidecar("backend")?;
    let (_rx, child) = command.spawn()?;
    let state = app.state::<BackendState>();
    *state.child.lock().unwrap() = Some(child);
    Ok(())
}

pub fn stop(app: &AppHandle) {
    let state = app.state::<BackendState>();
    let child = {
        let mut guard = state.child.lock().unwrap();
        guard.take()
    };
    if let Some(child) = child {
        let _ = child.kill();
    }
}
