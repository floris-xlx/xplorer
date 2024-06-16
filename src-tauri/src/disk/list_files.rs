use std::fs::{self, DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use base64::encode;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use image::ImageFormat;
use serde_json::{Value, json};
use anyhow::{Result, Error};

use crate::disk::utils::drivepath_helper::drivepath_from_letter;


pub fn list_files_on_drive(path: &str) -> Vec<Value> {
    let path: Box<Path> = drivepath_from_letter(path);

    println!("list_files_on_drive {:?}", path.to_str().unwrap());

    match list_files_at_root(&path) {
        Ok(files) => files,
        Err(_) => Vec::new(),
    }
}


pub fn list_files_at_root(dir: &Path) -> Result<Vec<Value>, Error> {
    let mut files: Vec<Value> = Vec::new();

    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry: DirEntry = entry?;
            let path: PathBuf = entry.path();

            println!("list_files_at_root {:?}", path.to_str().unwrap());

            if path.is_dir() {
                files.push(json!({ 
                    "directory": path.to_str().unwrap(),
                    "name": path.file_name().unwrap().to_str().unwrap(),
                    //"length": read_dir(&path)?.count()
                }));

            } else {
                let file_json: Value = process_file(&path)?;
                files.push(file_json);
            }
        }
    }
    Ok(files)
}


fn process_file(path: &Path) -> Result<Value, Error> {
    let file_str: &str = path.to_str().unwrap();
    let file_extension: String = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    let mut file_json: Value = json!({ "file": file_str });

    if ["png", "jpeg", "jpg", "ico", "gif"].contains(&file_extension.as_str()) {
        match ImageReader::open(&path).and_then(|reader| Ok(reader.decode())) {
            Ok(Ok(img)) => {
                let mut buf: Vec<u8> = Vec::new();
                if img.thumbnail(128, 128).write_to(&mut Cursor::new(&mut buf), ImageFormat::Png).is_ok() {
                    let base64_thumbnail: String = encode(&buf);
                    file_json["preview"] = json!(base64_thumbnail);
                }
            },
            Ok(Err(_)) | Err(_) => {}
        }
    }

    Ok(file_json)
}


fn read_dir(dir: &Path) -> Result<ReadDir, Error> {
    fs::read_dir(dir).map_err(Error::from)
}
