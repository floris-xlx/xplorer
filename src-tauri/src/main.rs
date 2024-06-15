// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{DiskExt, System, SystemExt};

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet, list_drives])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello greet, {}!", name);
    format!("Hello, {}!", name)
}

#[tauri::command]
fn list_drives() -> Vec<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut drives_info = Vec::new();
    for disk in sys.disks() {
        let drive_info = format!(
            "Drive: {} - Total Space: {} GB - Available Space: {} GB",
            disk.mount_point().display(),
            disk.total_space() / 1_000_000_000,
            disk.available_space() / 1_000_000_000
        );
        drives_info.push(drive_info);
    }
    drives_info
}