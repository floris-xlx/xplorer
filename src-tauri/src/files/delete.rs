use std::path::Path;
use std::fs;

#[tauri::command(rename_all = "snake_case")]
pub fn delete_files(filepath_list: Vec<String>) -> bool {
    for path in filepath_list {
        let path = Path::new(&path);
        if path.is_file() {
            if fs::remove_file(path).is_err() {
                return false;
            }
        } else if path.is_dir() {
            if fs::remove_dir_all(path).is_err() {
                return false;
            }
        }
    }
    true
}