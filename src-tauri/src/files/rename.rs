use std::path::Path;
use std::path::PathBuf;

use std::fs;

#[tauri::command(rename_all = "snake_case")]
pub fn rename_files(filepath_list: Vec<String>) -> bool {
    for (index, path) in filepath_list.iter().enumerate() {
        let path = Path::new(path);
        if let Some(extension) = path.extension() {
            let new_name = format!("{}.{}", index + 1, extension.to_string_lossy());
            let new_path = path.with_file_name(new_name);
            if fs::rename(path, new_path).is_err() {
                return false;
            }
        }
    }
    true
}