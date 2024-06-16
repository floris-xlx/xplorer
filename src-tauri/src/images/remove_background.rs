// take an image from a filepath and then remove the background from it and save it to a new file with _NO_BG.(EXTENSION)

use std::path::Path;
use std::fs;
use image::{ DynamicImage, GenericImageView, ImageFormat, Rgba };
use imageproc::contrast::threshold;

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
                    // Convert image to RGBA
                    let mut rgba_img = img.to_rgba8();

                    // Placeholder for background removal logic
                    // For now, we just make the background transparent based on a simple threshold
                    for pixel in rgba_img.pixels_mut() {
                        if pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200 {
                            *pixel = Rgba([255, 255, 255, 0]);
                        }
                    }

                    // Find the bounding box of the non-transparent pixels
                    let (mut min_x, mut min_y, mut max_x, mut max_y) = (u32::MAX, u32::MAX, 0, 0);
                    for (x, y, pixel) in rgba_img.enumerate_pixels() {
                        if pixel[3] != 0 {
                            // Check if the pixel is not transparent
                            if x < min_x {
                                min_x = x;
                            }
                            if y < min_y {
                                min_y = y;
                            }
                            if x > max_x {
                                max_x = x;
                            }
                            if y > max_y {
                                max_y = y;
                            }
                        }
                    }

                    // Crop the image to the bounding box
                    if min_x <= max_x && min_y <= max_y {
                        let cropped_img = DynamicImage::ImageRgba8(rgba_img).crop_imm(
                            min_x,
                            min_y,
                            max_x - min_x + 1,
                            max_y - min_y + 1
                        );

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
                        match cropped_img.save_with_format(&new_path, format) {
                            Ok(_) =>
                                println!(
                                    "Successfully saved image without background: {:?}",
                                    new_path
                                ),
                            Err(e) => println!("Failed to save image: {:?}", e),
                        }
                    } else {
                        println!("No non-transparent pixels found in the image: {:?}", path);
                    }
                }
                Err(e) => println!("Failed to open image: {:?}", e),
            }
        } else {
            println!("File has no extension: {:?}", path);
        }
    }
}
