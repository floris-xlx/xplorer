use std::path::Path;

pub fn drivepath_from_letter(drive_letter: &str) -> Box<Path> {
    // If the drive_letter is more than 3 characters, return it unchanged
    if drive_letter.len() > 3 {
        return Path::new(drive_letter).to_path_buf().into_boxed_path();
    }
    
    // Ensure the drive letter is a single character followed by a colon and a slash
    let drive_path: String = if drive_letter.len() == 1 {
        format!("{}:/", drive_letter)
    } else {
        drive_letter.to_string()
    };
    
    let path: Box<Path> = Path::new(&drive_path).to_path_buf().into_boxed_path();

    path
}
