use std::{fs::File, io::{Write, Cursor}};

use image::ImageOutputFormat;

#[tauri::command]
pub async fn convert_image(
  image: Vec<u8>,
  format: &str,
  path: String,
) -> Result<String, String> {
    let fmt = match format {
        "png" => ImageOutputFormat::Png,
        "jpeg" => ImageOutputFormat::Jpeg(100),
        "ico" => ImageOutputFormat::Ico,
        _ => panic!("Unsupported format"),
    };
    let image = image::load_from_memory(&image).unwrap();
    let mut buffer = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buffer), fmt)
        .unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(&buffer).unwrap();
    Ok("Image converted".to_string())
}