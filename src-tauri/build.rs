use std::{fs, process::Command};

fn main() {
    build_backend();
    tauri_build::build();
}

fn build_backend() {
    #[cfg(target_os = "windows")]
    let backend_bin = "../backend/target/release/backend.exe";

    #[cfg(not(target_os = "windows"))]
    let backend_bin = "../backend/target/release/backend";

    if std::path::Path::new(backend_bin).exists() {
        fs::remove_file(backend_bin).expect("Failed to remove old backend binary");
    }

    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--manifest-path",
            "../backend/Cargo.toml",
        ])
        .status()
        .unwrap();

    assert!(status.success());

    #[cfg(target_os = "windows")]
    let src = "../backend/target/release/backend.exe";

    #[cfg(not(target_os = "windows"))]
    let src = "../backend/target/release/backend";

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let dst = "binaries/backend-aarch64-apple-darwin";

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    let dst = "binaries/backend-x86_64-apple-darwin";

    #[cfg(target_os = "windows")]
    let dst = "binaries/backend-x86_64-pc-windows-msvc.exe";

    fs::create_dir_all("binaries").unwrap();
    fs::copy(src, dst).unwrap();
}
