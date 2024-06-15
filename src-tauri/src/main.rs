// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod disk;

use crate::disk::DisksInfo;
use crate::disk::list_files::list_files_on_drive;

use serde_json::{json, Value};

fn main() {
    tauri::Builder 
        ::default()
        .invoke_handler(tauri::generate_handler![
          greet, 
          list_drives,
          list_files
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
fn list_files(drive_letter: &str) -> Value {

  let result: Vec<Value> = list_files_on_drive(drive_letter);

  let mut files: Vec<Value> = Vec::new();
  let mut dirs: Vec<Value> = Vec::new();

  for item in result {
    if item.get("file").is_some() {
      files.push(item);
    } else if item.get("directory").is_some() {
      dirs.push(item);
    }
  }

  json!({ "files": files, "dirs": dirs })
}