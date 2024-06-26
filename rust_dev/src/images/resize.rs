use image::{ GenericImage, GenericImageView, DynamicImage, ImageFormat };
use std::path::{ Path, PathBuf };

#[tauri::command(rename_all = "snake_case")]
pub fn resize_images(filepath_list: Vec<String>) {
    for filepath in filepath_list {
        let path: &Path = Path::new(&filepath);
        if let Some(extension) = path.extension() {
            let new_filename: String = match path.file_stem() {
                Some(stem) =>
                    format!("{}_1200x1200.{}", stem.to_string_lossy(), extension.to_string_lossy()),
                None => {
                    println!("Failed to get file stem for: {:?}", path);
                    continue;
                }
            };
            let new_path: PathBuf = path.with_file_name(new_filename);

            match image::open(&path) {
                Ok(img) => {
                    let (width, height) = img.dimensions();
                    if width > 1200 || height > 1200 {
                        println!("Image dimensions exceed 1200x1200 for: {:?}", path);
                        continue;
                    }
                    let mut new_img: DynamicImage = DynamicImage::new_rgba8(1200, 1200);
                    // Calculate the position to place the original image in the center
                    let x_offset: u32 = (1200 - width) / 2;
                    let y_offset: u32 = (1200 - height) / 2;

                    // Overlay the original image onto the new image
                    if new_img.copy_from(&img, x_offset, y_offset).is_err() {
                        println!("Failed to copy image data for: {:?}", path);
                        continue;
                    }

                    let format: ImageFormat = match
                        extension.to_string_lossy().to_lowercase().as_str()
                    {
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

                    match new_img.save_with_format(&new_path, format) {
                        Ok(_) => println!("Successfully saved resized image: {:?}", new_path),
                        Err(e) => println!("Failed to save resized image: {:?}", e),
                    }
                }
                Err(e) => println!("Failed to open image: {:?}", e),
            }
        } else {
            println!("File has no extension: {:?}", path);
        }
    }
}
