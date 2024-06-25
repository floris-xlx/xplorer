// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod disk;
pub mod files;
pub mod images;
pub mod directories;
pub mod v2;

use crate::disk::DisksInfo;
use crate::disk::list_files::{ list_files_on_drive, list_files_at_root };
use crate::files::formats::avif::convert_avif_to_webp;
use crate::files::delete::delete_files;
use crate::images::remove_background::remove_background;
use crate::images::resize::resize_images;
use crate::directories::dir_size::calculate_dir_size_in_gb;
use crate::files::rename::rename_files;
use crate::files::search::search_by_name::search_keyword_in_files;
use std::fs::OpenOptions;
use serde_json::{ json, Value };
use std::path::Path;
use std::io::Write;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::process::{ Command, Stdio };
use std::process::Child;
use std::io::Error;

//v2 
use crate::v2::list_directory::v2_list_files;

fn main() {
    log_append_to_file("Starting Tauri application...");

    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                list_drives,
                list_files,
                list_files_from_root,
                open_file_from_path,
                convert_avif_to_webp,
                delete_files,
                rename_files,
                remove_background,
                resize_images,
                calculate_dir_size_in_gb,
                search_keyword_in_files,
                v2_list_files
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    log_append_to_file("Tauri application started successfully.");
}


#[tauri::command]
fn list_drives() -> Value {
    let drives = DisksInfo::list_drives();
    json!({
        "drives": drives.iter().map(|drive| json!({
            "drive_name": drive.name,
            "drive_available_space": drive.available_space,
            "drive_total_space": drive.total_space,
            "drive_percentage_full": drive.percentage_full,
            "drive_letter": drive.letter
        })).collect::<Vec<Value>>(),
        "length": drives.len()
    })
}


#[tauri::command(rename_all = "snake_case")]
fn list_files(path: &str) -> Value {
    if path.is_empty() || path == "null" {
        return json!({ "error": "Path is null" });
    }

    let start_time = SystemTime::now();
    let result = list_files_on_drive(path);

    let files: Vec<Value> = result.iter().filter(|item| item.get("file").is_some()).cloned().collect();
    let dirs: Vec<Value> = result.iter().filter(|item| item.get("directory").is_some()).cloned().collect();

    let total_time = SystemTime::now().duration_since(start_time).expect("Time went backwards").as_millis();

    json!({
        "files": files,
        "dirs": dirs,
        "loading_time": total_time,
        "total_amount_files": files.len(),
        "total_amount_dirs": dirs.len()
    })
}

#[tauri::command(rename_all = "snake_case")]
fn list_files_from_root(path: &str) -> Value {
    let start_time = SystemTime::now();
    let filepath = Path::new(path);

    let result = list_files_at_root(filepath).unwrap_or_default();

    let files: Vec<Value> = result.iter().filter(|item| item.get("file").is_some()).cloned().collect();
    let dirs: Vec<Value> = result.iter().filter(|item| item.get("directory").is_some()).cloned().collect();

    let total_time = SystemTime::now().duration_since(start_time).expect("Time went backwards").as_millis();

    json!({
        "files": files,
        "dirs": dirs,
        "loading_time": total_time,
        "total_amount_files": files.len(),
        "total_amount_dirs": dirs.len()
    })
}

fn log_append_to_file(log: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("../log.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", log) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

#[tauri::command(rename_all = "snake_case")]
fn open_file_from_path(path: &str) -> Result<(), String> {
    let path = path.replace("\\", "/");
    let result = Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(&path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open file: {}", e)),
    }
}