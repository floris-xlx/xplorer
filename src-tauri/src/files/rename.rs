use std::path::Path;
use std::path::PathBuf;

use std::fs;

#[tauri::command(rename_all = "snake_case")]
pub fn rename_files(filepath_list: Vec<String>) -> bool {
    for (index, path) in filepath_list.iter().enumerate() {
        let path: &Path = Path::new(&path);
        if let Some(extension) = path.extension() {
            let new_name: String = format!("{}.{}", index + 1, extension.to_string_lossy());
            let new_path: PathBuf = path.with_file_name(new_name);

            match fs::rename(path, new_path) {
                Ok(_) => println!("Successfully renamed file: {:?}", path),
                Err(e) => {
                    println!("Failed to rename file: {:?}", path);
                    println!("Error: {}", e);
                    return false;
                }
            }

        } else {
            println!("File has no extension: {:?}", path);
            return false;
        }
    }
    true
}
