use std::path::Path;
use std::path::PathBuf;



#[tauri::command(rename_all = "snake_case")]
pub fn rename_files(filepath_list: Vec<String>) -> bool {
    use std::fs;
    for (index, path) in filepath_list.iter().enumerate() {
        let path: &Path = Path::new(&path);
        let new_name = format!("{}", index + 1);
        let new_path: PathBuf = path.with_file_name(new_name);
        match fs::rename(path, new_path) {
            Ok(_) => println!("Successfully renamed file: {:?}", path),
            Err(e) => {
                println!("Failed to rename file: {:?}", path);
                println!("Error: {}", e);
                return false;
            }
        }
    }
    true
}