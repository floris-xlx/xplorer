use std::path::Path;

pub fn drivepath_from_letter(drive_letter: &str) -> Box<Path> {
    let drive_path: String = format!("{}:/", drive_letter);
    let path: Box<Path> = Path::new(&drive_path).to_path_buf().into_boxed_path();

    path
}

