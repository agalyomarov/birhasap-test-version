use std::{
    net::TcpListener,
    path::PathBuf,
    process::{Child, Command},
    sync::Mutex,
};

use tauri::{AppHandle, Manager};

struct BackendState {
    child: Option<Child>,
    port: Option<u16>,
    data_dir: Option<PathBuf>,
}

pub struct BackendManager {
    state: Mutex<BackendState>,
}

impl BackendManager {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(BackendState {
                child: None,
                port: None,
                data_dir: None,
            }),
        }
    }

    fn find_free_port() -> Result<u16, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port: u16 = listener.local_addr()?.port();
        drop(listener);

        Ok(port)
    }

    pub fn start(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_running() {
            return Ok(());
        }

        let exe = std::env::current_exe()?;
        let exe_dir = exe
            .parent()
            .ok_or("Cannot determine executable directory")?;

        #[cfg(target_os = "windows")]
        let backend_path = exe_dir.join("backend.exe");

        #[cfg(not(target_os = "windows"))]
        let backend_path = exe_dir.join("backend");

        let data_dir = match app.path().app_local_data_dir() {
            Ok(path) => path,
            Err(err) => {
                eprintln!("app_local_data_dir: {err}");
                return Err(err.into());
            }
        };
        std::fs::create_dir_all(&data_dir)?;

        let port = Self::find_free_port()?;

        println!("Selected port: {}", port);

        println!("Backend  : {}", backend_path.display());
        println!("Data dir : {}", data_dir.display());
        println!("Port     : {}", port);

        let child = Command::new(&backend_path)
            .arg("serve")
            .env("APP_DATA_DIR", &data_dir)
            .env("APP_PORT", port.to_string())
            .spawn()?;

        let mut state = self.state.lock().unwrap();

        state.child = Some(child);
        state.port = Some(port);
        state.data_dir = Some(data_dir);

        Ok(())
    }

    pub fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut state = self.state.lock().unwrap();
        if let Some(mut child) = state.child.take() {
            match child.try_wait()? {
                Some(_) => {}

                None => {
                    child.kill()?;
                    child.wait()?;
                }
            }
        }
        state.port = None;
        state.data_dir = None;

        Ok(())
    }

    pub fn restart(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        self.stop()?;
        self.start(app)
    }

    pub fn is_running(&self) -> bool {
        let mut state = self.state.lock().unwrap();

        if let Some(child) = state.child.as_mut() {
            match child.try_wait() {
                Ok(Some(_)) => {
                    state.child = None;
                    state.port = None;
                    state.data_dir = None;
                    false
                }

                Ok(None) => true,

                Err(_) => {
                    state.child = None;
                    state.port = None;
                    state.data_dir = None;
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn port(&self) -> Option<u16> {
        self.state.lock().unwrap().port
    }

    pub fn data_dir(&self) -> Option<PathBuf> {
        self.state.lock().unwrap().data_dir.clone()
    }
}
