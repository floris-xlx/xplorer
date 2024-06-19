use anyhow::{Result, Error};
use serde_json::{Value, json};
use std::path::Path;
use crate::disk::utils::drivepath_helper::drivepath_from_letter;


#[tauri::command(rename_all = "snake_case")]
pub fn calculate_dir_size_in_gb(path_list: Vec<String>) -> Vec<Value> {
    let mut result: Vec<Value> = Vec::new();

    for path in path_list {
        let path: Box<Path> = drivepath_from_letter(&path);

        let size: u64 = match calculate_dir_size(&path) {
            Ok(size) => size,
            Err(e) => {
                println!("Failed to calculate directory size: {:?}", e);
                continue;
            }
        };

        let size_in_gb: f64 = size as f64 / 1_073_741_824.0;
        let size_in_gb: f64 = (size_in_gb * 100.0).round() / 100.0;

        // pick a unit based on the size, kb / mb / gb / tb
        let units: Vec<&str> = vec!["KB", "MB", "GB", "TB"];

        let mut size: f64 = size_in_gb;
        let mut unit: &str = units[2];

        if size > 1024.0 {
            size /= 1024.0;
            unit = units[1]; // MB
        }
        if size > 1024.0 {
            size /= 1024.0;
            unit = units[2]; // GB
        }
        if size > 1024.0 {
            size /= 1024.0;
            unit = units[3]; // TB
        }

        
        let value: Value = json!({
            "path": path,
            "size": size,
            "unit": unit,
        });

        result.push(value);
    }

    result
}


fn calculate_dir_size( path: &Path ) -> Result<u64, Error> {
    let mut size: u64 = 0;

    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                size += calculate_dir_size(&path)?;
            } else {
                size += entry.metadata()?.len();
            }
        }
    } else {
        size += path.metadata()?.len();
    }

    Ok(size)
}