use std::path::Path;


pub fn drivepath_from_letter(drive_letter: &str) -> Box<Path> {
    if drive_letter.len() > 3 {
        return Path::new(drive_letter).to_path_buf().into_boxed_path();
    }

    let drive_path = if drive_letter.len() == 1 {
        format!("{}:/", drive_letter)
    } else {
        drive_letter.to_string()
    };

    Path::new(&drive_path).to_path_buf().into_boxed_path()
}
