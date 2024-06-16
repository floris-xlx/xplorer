// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod disk;
pub mod files;
pub mod images;

use crate::disk::DisksInfo;
use crate::disk::list_files::{ list_files_on_drive, list_files_at_root };
use crate::files::formats::avif::convert_avif_to_webp;
use crate::files::delete::delete_files;
use crate::images::remove_background::remove_background;
use crate::images::resize::resize_images;
use crate::files::rename::rename_files;
use std::fs::OpenOptions;
use serde_json::{ json, Value };
use std::path::Path;
use std::io::Write;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::process::{ Command, Stdio };
use std::process::Child;
use std::io::Error;

fn main() {
    println!("Starting Tauri application...");
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
                resize_images
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("Tauri application started successfully.");
    log_append_to_file("Tauri application started successfully.");
}

#[tauri::command]
fn list_drives() -> Value {
    println!("Listing drives...");
    log_append_to_file("Listing drives...");

    let drives: Vec<DisksInfo> = DisksInfo::list_drives();

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

    let response = json!({ 
        "drives": drives_json,
        "length": drives_json.len()
    });

    println!("Drives listed: {:?}", response);
    log_append_to_file(&format!("Drives listed: {:?}", response));

    response
}

#[tauri::command(rename_all = "snake_case")]
fn list_files(path: &str) -> Value {
    // handle path error
    if path.is_empty() || path == "null" {
        let error_message = "Error: Path is null";
        println!("{}", error_message);
        log_append_to_file(error_message);
        return json!({ "error": "Path is null" });
    }
    println!("Listing files from path: {}", path);
    log_append_to_file(&format!("Listing files from path: {}", path));

    let start_time_in_ms: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("Start time in ms: {}", start_time_in_ms);
    log_append_to_file(&format!("Start time in ms: {}", start_time_in_ms));

    let result: Vec<Value> = list_files_on_drive(path);
    println!("Result from list_files_on_drive: {:?}", result);
    log_append_to_file(&format!("Result from list_files_on_drive: {:?}", result));

    let mut files: Vec<Value> = Vec::new();
    let mut dirs: Vec<Value> = Vec::new();

    for item in result {
        if item.get("file").is_some() {
            println!("File found: {:?}", item);
            log_append_to_file(&format!("File found: {:?}", item));
            files.push(item);
        } else if item.get("directory").is_some() {
            println!("Directory found: {:?}", item);
            log_append_to_file(&format!("Directory found: {:?}", item));
            dirs.push(item);
        }
    }

    let end_time_in_ms: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("End time in ms: {}", end_time_in_ms);
    log_append_to_file(&format!("End time in ms: {}", end_time_in_ms));

    let total_amount_files: usize = files.len();
    println!("Total amount of files: {}", total_amount_files);
    log_append_to_file(&format!("Total amount of files: {}", total_amount_files));

    let total_amount_dirs: usize = dirs.len();
    println!("Total amount of directories: {}", total_amount_dirs);
    log_append_to_file(&format!("Total amount of directories: {}", total_amount_dirs));

    let total_time: u128 = end_time_in_ms - start_time_in_ms;
    println!("Total time taken: {} ms", total_time);
    log_append_to_file(&format!("Total time taken: {} ms", total_time));

    let total_log: String = format!(
        "{} {} {} {}",
        total_amount_files,
        total_amount_dirs,
        total_time,
        path
    );
    println!("Total log: {}", total_log);
    log_append_to_file(&total_log);

    let response =
        json!({ 
        "files": files, 
        "dirs": dirs, 
        "loading_time": total_time, 
        "total_amount_files": total_amount_files, 
        "total_amount_dirs": total_amount_dirs
    });
    println!("Response: {:?}", response);
    log_append_to_file(&format!("Response: {:?}", response));

    response
}

#[tauri::command(rename_all = "snake_case")]
fn list_files_from_root(path: &str) -> Value {
    println!("Listing files from root path: {}", path);
    log_append_to_file(&format!("Listing files from root path: {}", path));

    let start_time_in_ms: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("Start time in ms: {}", start_time_in_ms);
    log_append_to_file(&format!("Start time in ms: {}", start_time_in_ms));

    let filepath: &Path = Path::new(path);

    let result: Vec<Value> = list_files_at_root(filepath).unwrap();
    println!("Result from list_files_at_root: {:?}", result);
    log_append_to_file(&format!("Result from list_files_at_root: {:?}", result));

    let mut files: Vec<Value> = Vec::new();
    let mut dirs: Vec<Value> = Vec::new();

    for item in result {
        if item.get("file").is_some() {
            println!("File found: {:?}", item);
            log_append_to_file(&format!("File found: {:?}", item));
            files.push(item);
        } else if item.get("directory").is_some() {
            println!("Directory found: {:?}", item);
            log_append_to_file(&format!("Directory found: {:?}", item));
            dirs.push(item);
        }
    }

    let total_amount_files: usize = files.len();
    println!("Total amount of files: {}", total_amount_files);
    log_append_to_file(&format!("Total amount of files: {}", total_amount_files));

    let total_amount_dirs: usize = dirs.len();
    println!("Total amount of directories: {}", total_amount_dirs);
    log_append_to_file(&format!("Total amount of directories: {}", total_amount_dirs));

    let end_time_in_ms: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let total_time: u128 = end_time_in_ms - start_time_in_ms;
    println!("Total time taken: {} ms", total_time);
    log_append_to_file(&format!("Total time taken: {} ms", total_time));

    let total_log: String = format!(
        "Total files:{} Total dirs:{} TIME: {}ms PATH: {}",
        total_amount_files,
        total_amount_dirs,
        total_time,
        path
    );
    println!("Total log: {}", total_log);
    log_append_to_file(&total_log);

    let response =
        json!({ 
        "files": files, 
        "dirs": dirs, 
        "loading_time": total_time, 
        "total_amount_files": total_amount_files, 
        "total_amount_dirs": total_amount_dirs
    });
    println!("Response: {:?}", response);
    log_append_to_file(&format!("Response: {:?}", response));

    response
}

fn log_append_to_file(log: &str) {
    let mut file: std::fs::File = OpenOptions::new()
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
    println!("Opening file from path: {}", path);
    log_append_to_file(&format!("Opening file from path: {}", path));

    // Normalize path to use forward slashes
    let path: String = path.replace("\\", "/");

    let result: Result<Child, Error> = Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(&path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match result {
        Ok(_) => {
            let success_message: &str = "File opened successfully.";
            println!("{}", success_message);
            log_append_to_file(success_message);
            Ok(())
        }
        Err(e) => {
            let error_message: String = format!("Failed to open file: {}", e);
            println!("{}", error_message);
            log_append_to_file(&error_message);
            Err(error_message)
        }
    }
}
