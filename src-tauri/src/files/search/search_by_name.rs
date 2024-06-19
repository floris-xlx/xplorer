use crate::disk::list_files::list_files_at_root;

use std::path::Path;
use std::prelude::v1::Result as V1Result;
use serde_json::Value;


#[tauri::command(rename_all = "snake_case")]
pub fn search_keyword_in_files(keyword: &str, filepath: &Path) -> Result<Vec<Value>, String> {
    let keyword: String = keyword.to_lowercase();

    let files: Vec<Value> = match list_files_at_root(filepath) {
        Ok(files) => files,
        Err(e) => return Err(e.to_string()),
    };

    println!("Searching for keyword: {}", keyword);
    println!("Files: {:?}", files);



    let mut matching_files = Vec::new();

    for file in files {
        let filename: String = match file["filename"].as_str() {
            Some(name) => name.to_lowercase(),
            None => continue,
        };

        if filename.contains(&keyword) {
            println!("Found matching file: {:?}", file);
            matching_files.push(file);
        }
    }

    Ok(matching_files)
}
