use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    clear_binaries();
    build_backend();
    tauri_build::build();
}

fn build_backend() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let backend_dir = manifest_dir.join("..").join("..").join("backend");
    let target = env::var("TARGET").unwrap();
    let binary_name = if target.contains("windows") {
        "backend.exe"
    } else {
        "backend"
    };

    let status = Command::new("bun")
        .current_dir(&backend_dir)
        .args([
            "build",
            "src/index.ts",
            "--compile",
            "--minify",
            "--outfile",
            binary_name,
        ])
        .status()
        .expect("Failed to execute bun");
    assert!(status.success(), "bun build failed");

    let target_name = if target.contains("windows") {
        "backend-x86_64-pc-windows-msvc.exe"
    } else if target.contains("apple-darwin") {
        if target.starts_with("aarch64") {
            "backend-aarch64-apple-darwin"
        } else {
            "backend-x86_64-apple-darwin"
        }
    } else {
        panic!("Unsupported target: {}", target);
    };

    let binaries_dir = manifest_dir.join("binaries");
    fs::create_dir_all(&binaries_dir).unwrap();

    fs::copy(
        backend_dir.join(binary_name),
        binaries_dir.join(target_name),
    )
    .unwrap();
}

fn clear_binaries() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let binaries_dir = manifest_dir.join("binaries");
    if !binaries_dir.exists() {
        return;
    }

    for entry in fs::read_dir(&binaries_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            fs::remove_file(path).unwrap();
        } else if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        }
    }
}
