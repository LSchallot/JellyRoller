use base64::{engine::general_purpose, Engine as _};
use image::ImageFormat;
use std::io::Cursor;
///
/// Function that converts an image into a base64 png image.
///
pub fn image_to_base64(path: String) -> String {
    let base_img = image::open(path).unwrap();
    let mut image_data: Vec<u8> = Vec::new();
    base_img
        .write_to(&mut Cursor::new(&mut image_data), ImageFormat::Png)
        .unwrap();
    general_purpose::STANDARD.encode(image_data)
}