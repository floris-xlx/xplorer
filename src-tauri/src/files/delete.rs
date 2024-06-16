use std::path::Path;


#[tauri::command(rename_all = "snake_case")]
pub fn delete_files(filepath_list: Vec<String>) -> bool {
    use std::fs;
    for path in filepath_list {
        let path: &Path = Path::new(&path);
        if path.is_file() {
            match fs::remove_file(path) {
                Ok(_) => println!("Successfully deleted file: {:?}", path),
                Err(e) => {
                    println!("Failed to delete file: {:?}", path);
                    println!("Error: {}", e);
                    return false;
                }
            }
        } else if path.is_dir() {
            match fs::remove_dir_all(path) {
                Ok(_) => println!("Successfully deleted directory: {:?}", path),
                Err(e) => {
                    println!("Failed to delete directory: {:?}", path);
                    println!("Error: {}", e);
                    return false;
                }
            }
        }
    }
    true
}