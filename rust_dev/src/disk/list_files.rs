use std::fs;
use std::io;
use std::path::Path;

pub fn list_files_on_drive(drive_letter: &str) -> io::Result<()> {
    let drive_path = format!("{}:/", drive_letter);
    let path = Path::new(&drive_path);

    fn list_files_recursively(dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    println!("Directory: {:?}", path);
                    list_files_recursively(&path)?;
                } else {
                    println!("File: {:?}", path);
                }
            }
        }
        Ok(())
    }

    if path.exists() {
        list_files_recursively(path)?;
    } else {
        eprintln!("Drive {} does not exist.", drive_letter);
    }
    
    Ok(())
}