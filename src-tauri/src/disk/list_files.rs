use std::fs::{ self, DirEntry, ReadDir };
use std::path::{ Path, PathBuf };
use base64::encode;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use image::{ ImageFormat, DynamicImage };
use serde_json::{ Value, json };
use anyhow::{ Result, Error };
use std::time::{ Instant, Duration };
use std::prelude::v1::Result as V1Result;
use std::sync::{ Arc, Mutex, MutexGuard };
use std::thread;
use std::ffi::OsStr;

use crate::disk::utils::drivepath_helper::drivepath_from_letter;

pub fn list_files_on_drive(path: &str) -> Vec<Value> {
    match list_files_at_root(&drivepath_from_letter(path)) {
        Ok(files) => files,
        Err(_) => Vec::new(),
    }
}

pub fn list_files_at_root(dir: &Path) -> Result<Vec<Value>, Error> {
    let files = Arc::new(Mutex::new(Vec::new()));
    if dir.is_dir() {
        let handles: Vec<_> = read_dir(dir)?.filter_map(Result::ok).map(|entry| {
            let files = Arc::clone(&files);
            thread::spawn(move || {
                let path = entry.path();
                let value = if path.is_dir() {
                    serialize_directory(&path)
                } else {
                    process_file(&path)
                };
                if let Ok(value) = value {
                    files.lock().unwrap().push(value);
                }
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
    Ok(Arc::try_unwrap(files).unwrap().into_inner().unwrap())
}


fn serialize_directory(path: &Path) -> Result<Value, Error> {
    Ok(json!({
        "directory": path.to_str().unwrap(),
        "name": path.file_name().unwrap().to_str().unwrap(),
    }))
}

fn process_file(path: &Path) -> Result<Value, Error> {
    let file_str = path.to_str().unwrap();
    let file_extension = path.extension().and_then(OsStr::to_str).unwrap_or("").to_lowercase();
    let filename = path.file_name().and_then(OsStr::to_str).unwrap_or("").to_string();
    Ok(json!({
        "file": file_str,
        "filename": filename,
        "extension": file_extension
    }))
}

fn get_image_thumbnail_base64(path: &Path) -> Result<String, Error> {
    let image = ImageReader::open(path)?.decode()?;
    let thumbnail = image.thumbnail(100, 100);
    let mut buffer = Cursor::new(Vec::new());
    thumbnail.write_to(&mut buffer, ImageFormat::Png)?;
    Ok(encode(buffer.get_ref()))
}


fn is_image_extension(extension: &str) -> bool {
    matches!(extension, "png" | "jpeg" | "jpg" | "ico" | "gif")
}

fn read_dir(dir: &Path) -> Result<ReadDir, Error> {
    Ok(fs::read_dir(dir)?)
}