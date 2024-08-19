use image::{DynamicImage, RgbaImage};
use std::path::Path;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
}

impl Texture {
    pub fn load_from_file<P: AsRef<Path>>(filename: P) -> Result<Texture, String> {
        let img = match image::open(filename) {
            Ok(img) => img,
            Err(e) => return Err(format!("Failed to load texture: {}", e)),
        };

        let rgba = img.to_rgba8();
        let width = rgba.width() as usize;
        let height = rgba.height() as usize;

        let mut data = Vec::with_capacity(width * height);
        for pixel in rgba.pixels() {
            let rgba = pixel.0;
            let color = ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | (rgba[2] as u32);
            data.push(color);
        }

        Ok(Texture {
            width,
            height,
            data,
        })
    }
}
