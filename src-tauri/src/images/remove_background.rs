// take an image from a filepath and then remove the background from it and save it to a new file with _NO_BG.(EXTENSION)

use std::path::Path;
use std::fs;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageFormat;


#[tauri::command(rename_all = "snake_case")]
pub fn remove_background(filepath_list: Vec<String>) {
    for filepath in filepath_list {
        let path = Path::new(&filepath);
        if let Some(extension) = path.extension() {
            let new_filename = format!(
                "{}_NO_BG.{}",
                path.file_stem().unwrap().to_string_lossy(),
                extension.to_string_lossy()
            );
            let new_path = path.with_file_name(new_filename);

            match image::open(&path) {
                Ok(mut img) => {
                    // Placeholder for background removal logic
                    // For now, we just save the image as is
                    let format = match extension.to_string_lossy().to_lowercase().as_str() {
                        "png" => ImageFormat::Png,
                        "jpeg" | "jpg" => ImageFormat::Jpeg,
                        "gif" => ImageFormat::Gif,
                        "bmp" => ImageFormat::Bmp,
                        "ico" => ImageFormat::Ico,
                        "tiff" => ImageFormat::Tiff,
                        "webp" => ImageFormat::WebP,
                        _ => {
                            println!("Unsupported image format: {:?}", extension);
                            continue;
                        }
                    };
                    match img.save_with_format(&new_path, format) {
                        Ok(_) => println!("Successfully saved image without background: {:?}", new_path),
                        Err(e) => println!("Failed to save image: {:?}", e),
                    }
                }
                Err(e) => println!("Failed to open image: {:?}", e),
            }
        } else {
            println!("File has no extension: {:?}", path);
        }
    }
}