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

use crate::disk::utils::drivepath_helper::drivepath_from_letter;

pub fn list_files_on_drive(path: &str) -> Vec<Value> {
    let path: Box<Path> = drivepath_from_letter(path);

    match list_files_at_root(&path) {
        Ok(files) => files,
        Err(_) => Vec::new(),
    }
}

pub fn list_files_at_root(dir: &Path) -> Result<Vec<Value>, Error> {
    let start_time: Instant = Instant::now();
    let files: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));

    if dir.is_dir() {
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];

        for entry in read_dir(dir)? {
            let files: Arc<Mutex<Vec<Value>>> = Arc::clone(&files);
            let entry: DirEntry = entry?;
            let path: PathBuf = entry.path();

            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                let loop_start_time: Instant = Instant::now();

                let result: std::prelude::v1::Result<Value, Error> = if path.is_dir() {
                    serialize_directory(&path)
                } else {
                    process_file(&path)
                };

                if let Ok(value) = result {
                    let mut files: MutexGuard<Vec<Value>> = files.lock().unwrap();
                    files.push(value);
                }

                let loop_duration: Duration = loop_start_time.elapsed();
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    let duration: Duration = start_time.elapsed();

    let files: Vec<Value> = Arc::try_unwrap(files).unwrap().into_inner().unwrap();
    Ok(files)
}

fn serialize_directory(path: &Path) -> Result<Value, Error> {
    let start_time: Instant = Instant::now();

    let result: Value =
        json!({
        "directory": path.to_str().unwrap(),
        "name": path.file_name().unwrap().to_str().unwrap(),
    });

    let duration: Duration = start_time.elapsed();
    println!("\x1b[34;1mserialize_directory took {} milliseconds\x1b[0m", duration.as_millis());

    Ok(result)
}

fn process_file(path: &Path) -> Result<Value, Error> {
    let start_time: Instant = Instant::now();

    let file_str: &str = path.to_str().unwrap();
    let file_extension: String = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let filename: String = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();

    let file_json: Value = json!({ 
        "file": file_str, 
        "filename": filename, 
        "extension": file_extension
    });

    let duration: Duration = start_time.elapsed();
    println!("\x1b[34;1mprocess_file took {} milliseconds\x1b[0m", duration.as_millis());

    Ok(file_json)
}

fn get_image_thumbnail_base64(path: &Path) -> Result<String, Error> {
    use std::time::Instant;
    let start_time: Instant = Instant::now();

    let image: DynamicImage = ImageReader::open(path)?.decode()?;
    let thumbnail: DynamicImage = image.thumbnail(100, 100);
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    thumbnail.write_to(&mut buffer, ImageFormat::Png)?;
    let base64_thumbnail: String = base64::encode(buffer.get_ref());

    let duration: Duration = start_time.elapsed();
    println!(
        "\x1b[34;1mget_image_thumbnail_base64 took {} milliseconds\x1b[0m",
        duration.as_millis()
    );
    Ok(base64_thumbnail)
}

fn is_image_extension(extension: &str) -> bool {
    use std::time::Instant;
    let start_time: Instant = Instant::now();

    let result: bool = matches!(extension, "png" | "jpeg" | "jpg" | "ico" | "gif");

    let duration: Duration = start_time.elapsed();
    println!("\x1b[34;1mis_image_extension took {} milliseconds\x1b[0m", duration.as_millis());

    result
}

fn read_dir(dir: &Path) -> Result<ReadDir, Error> {
    use std::time::Instant;
    let start_time: Instant = Instant::now();

    let result: V1Result<ReadDir, Error> = fs::read_dir(dir).map_err(Error::from);

    let duration: Duration = start_time.elapsed();
    println!("\x1b[34;1mread_dir took {} milliseconds\x1b[0m", duration.as_millis());

    result
}
