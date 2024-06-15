use std::fs;
use std::io;
use std::path::Path;

use serde_json::{Value, json};
use anyhow::{Result, Error};


pub fn list_files_on_drive(drive_letter: &str) -> Vec<Value> {
    use base64::encode;
    use image::io::Reader as ImageReader;
    use std::io::Cursor;
    use image::ImageFormat;

    let drive_path = format!("{}:/", drive_letter);
    let path = Path::new(&drive_path);

    fn list_files_at_root(dir: &Path) -> Result<Vec<Value>, Error> {
        let mut files = Vec::new();
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    files.push(json!({ "directory": path.to_str().unwrap() }));

                } else {
                    let file_str = path.to_str().unwrap();
                    let file_extension = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                    let mut file_json = json!({ "file": file_str });

                    if ["png", "jpeg", "jpg", "ico", "gif"].contains(&file_extension.as_str()) {
                        if let Ok(img) = ImageReader::open(&path).and_then(|reader| Ok(reader.decode())) {
                            if let Ok(img) = img {
                                let mut buf = Vec::new();
                                img.thumbnail(64, 64).write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)?;
                              

                                let base64_thumbnail = encode(&buf);
                                file_json["preview"] = json!(base64_thumbnail);
                            }
                        }
                    
                    }

                    files.push(file_json);
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