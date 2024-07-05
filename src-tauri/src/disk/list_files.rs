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

/// this handles the actual file return
fn process_file(path: &Path) -> Result<Value, Error> {
    let file_str: &str = path.to_str().unwrap();
    let file_extension: String = path.extension().and_then(OsStr::to_str).unwrap_or("").to_lowercase();
    let filename: String = path.file_name().and_then(OsStr::to_str).unwrap_or("").to_string();

    Ok(json!({
        "file": file_str,
        "filename": filename,
        "extension": file_extension
    }))
}


#[tauri::command(rename_all = "snake_case")]
pub async fn get_image_thumbnail(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or("").to_lowercase();
    
    if !is_image_extension(&extension) {
        return Err("File is not an image".to_string());
    }

    match get_image_thumbnail_base64(path).await {
        Ok(thumbnail) => Ok(thumbnail),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_video_thumbnail(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or("").to_lowercase();

    if !is_video_extension(&extension) {
        return Err("File is not a video".to_string());
    }

    match get_video_thumbnail_base_64(path) {
        Ok(thumbnail) => Ok(thumbnail),
        Err(e) => Err(e.to_string()),
    }
}


pub async fn get_image_thumbnail_base64(path: &Path) -> Result<String, Error> {
    println!("Opening image at path: {:?}", path);
    let metadata = fs::metadata(path)?;
    if metadata.len() == 0 {
        return Err(Error::msg("Image file is empty"));
    }

    let image = ImageReader::open(path)?.decode()?;
    println!("Image opened and decoded successfully.");

    println!("Creating thumbnail for the image.");
    let thumbnail = image.thumbnail(512, 512    );
    let mut buffer = Cursor::new(Vec::new());

    println!("Writing thumbnail to buffer.");
    thumbnail.write_to(&mut buffer, ImageFormat::Png)?;
    println!("Thumbnail written to buffer successfully.");

    if buffer.get_ref().is_empty() {
        return Err(Error::msg("Thumbnail buffer is empty"));
    }

    let encoded_thumbnail = encode(buffer.get_ref());
    println!("Thumbnail encoded to base64 successfully.");

    Ok(encoded_thumbnail)
}

use std::process::Command;

pub fn get_video_thumbnail_base_64(path: &Path) -> Result<String, Error> {
    println!("Opening video at path: {:?}", path);
    let metadata = fs::metadata(path)?;
    if metadata.len() == 0 {
        return Err(Error::msg("Video file is empty"));
    }

    // Use ffmpeg to generate a thumbnail
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", path.to_str().unwrap(),
            "-ss", "00:00:01.000",
            "-vframes", "1",
            "-f", "image2pipe",
            "-vcodec", "png",
            "-",
        ])
        .output()?;

    if !output.status.success() {
        return Err(Error::msg("Failed to generate video thumbnail"));
    }

    let thumbnail = ImageReader::new(Cursor::new(output.stdout)).with_guessed_format()?.decode()?;
    println!("Video thumbnail generated and decoded successfully.");

    println!("Creating thumbnail for the video.");
    let thumbnail = thumbnail.thumbnail(512, 512);
    let mut buffer = Cursor::new(Vec::new());

    println!("Writing thumbnail to buffer.");
    thumbnail.write_to(&mut buffer, ImageFormat::Png)?;
    println!("Thumbnail written to buffer successfully.");

    if buffer.get_ref().is_empty() {
        return Err(Error::msg("Thumbnail buffer is empty"));
    }

    let encoded_thumbnail = encode(buffer.get_ref());
    println!("Thumbnail encoded to base64 successfully.");

    Ok(encoded_thumbnail)
}


fn is_image_extension(extension: &str) -> bool {
    matches!(extension, "png" | "jpeg" | "jpg" | "ico" | "gif")
}


fn is_video_extension(extension: &str) -> bool {
    matches!(extension.to_lowercase().as_str(), "mp4" | "mkv" | "avi" | "mov" | "flv" | "wmv")
}

fn read_dir(dir: &Path) -> Result<ReadDir, Error> {
    Ok(fs::read_dir(dir)?)
}