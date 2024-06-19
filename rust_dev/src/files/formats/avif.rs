use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use image::{ImageFormat, DynamicImage};
use image::codecs::webp::WebPEncoder;


pub fn convert_avif_to_webp(path: &str) -> Result<(), String> {
    println!("Starting conversion from AVIF to WebP for file: {}", path);
    
    let path: &Path = Path::new(path);
    let output_path: PathBuf = path.with_extension("webp");

    println!("Output path will be: {:?}", output_path);

    let file: File = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            let err_msg: String = format!("Failed to open input file: {}", e);
            println!("{}", err_msg);
            return Err(err_msg);
        }
    };
    let reader: BufReader<File> = BufReader::new(file);

    let img: DynamicImage = match image::load(reader, ImageFormat::Avif) {
        Ok(img) => img,
        Err(e) => {
            let err_msg: String = format!("Failed to decode AVIF image: {}", e);
            println!("{}", err_msg);
            return Err(err_msg);
        }
    };

    println!("Successfully decoded AVIF image.");

    let output_file: File = match File::create(&output_path) {
        Ok(file) => file,
        Err(e) => {
            let err_msg: String = format!("Failed to create output file: {}", e);
            println!("{}", err_msg);
            return Err(err_msg);
        }
    };
    let writer: BufWriter<File> = BufWriter::new(output_file);

    let encoder: WebPEncoder<BufWriter<File>> = WebPEncoder::new_lossless(writer);
    if let Err(e) = encoder.encode(&img.to_rgba8(), img.width(), img.height(), img.color().into()) {
        let err_msg: String = format!("Failed to encode WebP image: {}", e);
        println!("{}", err_msg);
        return Err(err_msg);
    }

    println!("Successfully encoded and saved WebP image to: {:?}", output_path);

    Ok(())
}
