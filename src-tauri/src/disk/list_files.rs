use std::fs;
use std::io;
use std::path::Path;

use serde_json::{Value, json};
use anyhow::{Result, Error};

pub fn list_files_on_drive(drive_letter: &str) -> Vec<Value> {
    let drive_path = format!("{}:/", drive_letter);
    let path = Path::new(&drive_path);

    fn list_files_at_root(dir: &Path) -> Result<Vec<Value>, io::Error> {
        let mut files = Vec::new();
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                   
                    files.push(json!({ "directory": path.to_str().unwrap() }));
                } else {
                 
                    files.push(json!({ "file": path.to_str().unwrap() }));
                }
            }
        }
        Ok(files)
    }

    match list_files_at_root(&path) {
        Ok(files) => files,
        Err(_) => Vec::new(),
    }
}