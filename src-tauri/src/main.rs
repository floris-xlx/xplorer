// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod disk;
pub mod files;

use crate::disk::DisksInfo;
use crate::disk::list_files::{ list_files_on_drive, list_files_at_root };
use crate::files::formats::avif::convert_avif_to_webp;
use std::fs::OpenOptions;
use serde_json::{ json, Value };


fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![greet, list_drives, list_files, list_files_from_root, open_file_from_path, convert_avif_to_webp]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Hello greet, {}!", name);
    format!("Hello, {}!", name)
}

#[tauri::command]
fn list_drives() -> Value {
    let drives: Vec<DisksInfo> = DisksInfo::list_drives();

    println!("{drives:#?}");

    let mut drives_json: Vec<Value> = Vec::new();

    for drive in drives {
        let drive_json: Value =
            json!({ 
      "drive_name": drive.name,
      "drive_available_space": drive.available_space,
      "drive_total_space": drive.total_space, 
      "drive_percentage_full": drive.percentage_full,
      "drive_letter": drive.letter
    });

        drives_json.push(drive_json);
    }

    json!({ 
    "drives": drives_json,
    "length": drives_json.len()
  })
}

#[tauri::command(rename_all = "snake_case")]
fn list_files(path: &str) -> Value {
    let start_time_in_ms = std::time::SystemTime
        ::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let result: Vec<Value> = list_files_on_drive(path);

    let mut files: Vec<Value> = Vec::new();
    let mut dirs: Vec<Value> = Vec::new();

    for item in result {
        if item.get("file").is_some() {
            files.push(item);
        } else if item.get("directory").is_some() {
            dirs.push(item);
        }
    }

    let end_time_in_ms: u128 = std::time::SystemTime
        ::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let total_amount_files: usize = files.len();
    let total_amount_dirs: usize = dirs.len();
    let total_time: u128 = end_time_in_ms - start_time_in_ms;

    let total_log: String = format!("{} {} {} {}", total_amount_files, total_amount_dirs, total_time, path);
    log_append_to_file(&total_log);

    

    json!({ "files": files, "dirs": dirs, "loading_time": total_time, "total_amount_files": total_amount_files, "total_amount_dirs": total_amount_dirs})
}


#[tauri::command(rename_all = "snake_case")]
fn list_files_from_root(path: &str) -> Value {
    let start_time_in_ms: u128 = std::time::SystemTime
        ::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    // check if path is not a duplicate

    println!("list_files_from_root {:?}", path);

    let filepath: &std::path::Path = std::path::Path::new(path);

    let result: Vec<Value> = list_files_at_root(filepath).unwrap();

    let mut files: Vec<Value> = Vec::new();
    let mut dirs: Vec<Value> = Vec::new();

    for item in result {
        if item.get("file").is_some() {
            files.push(item);
        } else if item.get("directory").is_some() {
            dirs.push(item);
        }
    }

    let total_amount_files: usize = files.len();
    let total_amount_dirs: usize = dirs.len();

    let end_time_in_ms: u128 = std::time::SystemTime
        ::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let total_time: u128 = end_time_in_ms - start_time_in_ms;

    let total_log: String = format!("Total files:{} Total dirs:{} TIME: {}ms PATH: {}", total_amount_files, total_amount_dirs, total_time, path);
    log_append_to_file(&total_log);

    json!({ 
      "files": files, 
      "dirs": dirs, 
      "loading_time": total_time, 
      "total_amount_files": total_amount_files, 
      "total_amount_dirs": total_amount_dirs
    })
}


fn log_append_to_file(log: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file: std::fs::File = OpenOptions::new().create(true).append(true).open("../log.txt").unwrap();

    if let Err(e) = writeln!(file, "{}", log) {
        eprintln!("Couldn't write to file: {}", e);
    }
}


#[tauri::command(rename_all = "snake_case")]
fn open_file_from_path(path: &str) -> Result<(), String> {
    use std::process::Command;
    use std::process::Stdio;

    // Normalize path to use forward slashes
    let path: String = path.replace("\\", "/");
    println!("OPENING: open_file_from_path {:?}", path);

    let result = Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(&path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    println!("RESULT: {:?}", result);

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("failed to open file: {}", e)),
    }
}
