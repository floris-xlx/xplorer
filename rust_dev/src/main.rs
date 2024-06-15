// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{DiskExt, System, SystemExt};

fn main() {
    println!("Hello, world!");

    let drives = list_drives();

    for drive in drives {
        println!("{}", drive);
    
    
}


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