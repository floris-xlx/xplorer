use io_plus::read_dir;

use std::path::Path;

use serde_json::{Value, json};

use anyhow::{Result, Error};

use std::sync::{Arc, Mutex};



#[tauri::command(rename_all = "snake_case")]
pub fn v2_list_files(path: &str) -> Result<Value, Error> {
    let path = Path::new(path);
    let files = tokio::task::block_in_place(|| read_dir(path))?;
    
    Ok(files)
}