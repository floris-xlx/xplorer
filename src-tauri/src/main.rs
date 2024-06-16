// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod disk;

use crate::disk::DisksInfo;
use crate::disk::list_files::{list_files_on_drive, list_files_at_root};

use serde_json::{json, Value};

fn main() {
    tauri::Builder 
        ::default()
        .invoke_handler(tauri::generate_handler![
          greet, 
          list_drives,
          list_files,
          list_files_from_root
        ])
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
    let drive_json: Value = json!({ 
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
  let start_time_in_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();

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

  let end_time_in_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
  let total_time = end_time_in_ms - start_time_in_ms;

  json!({ "files": files, "dirs": dirs, "loading_time": total_time})
}


#[tauri::command(rename_all = "snake_case")]
fn list_files_from_root(path: &str) -> Value {
  let start_time_in_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
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

  let end_time_in_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
  let total_time = end_time_in_ms - start_time_in_ms;


  json!({ "files": files, "dirs": dirs, "loading_time": total_time})
}