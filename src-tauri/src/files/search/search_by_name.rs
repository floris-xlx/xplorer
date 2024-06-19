use crate::disk::list_files::list_files_at_root;

use std::path::Path;
use std::prelude::v1::Result as V1Result;
use serde_json::{Value, json};
use std::time::SystemTime;

#[tauri::command(rename_all = "snake_case")]
pub fn search_keyword_in_files(keyword: &str, filepath: &Path) -> Result<Value, String> {
    let start_time = SystemTime::now();
    let keyword = keyword.to_lowercase();
    let files = list_files_at_root(filepath).map_err(|e| e.to_string())?;
    let matching_files: Vec<Value> = files.into_iter().filter(|file| {
        file["filename"].as_str().map_or(false, |name| name.to_lowercase().contains(&keyword))
    }).collect();

    let duration: std::time::Duration = SystemTime::now().duration_since(start_time).expect("Time went backwards");

    Ok(json!({
        "results": matching_files,
        "amount_results": matching_files.len(),
        "duration": SystemTime::now().duration_since(start_time).expect("Time went backwards").as_millis(),
        "items_per_second": matching_files.len() as f64 / duration.as_secs_f64(),
    }))
}