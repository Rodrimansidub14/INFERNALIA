
use rusttype::point;
use rusttype::Font;
use rusttype::PositionedGlyph;
use rusttype::Scale;
// Removed unused import


pub struct Framebuffer {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    current_color: u32,
    background_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height];
        Framebuffer {
            buffer,
            width,
            height,
            current_color: 0xFFFFFFFF, // White by default
            background_color: 0x00000000, // Black by default
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        } else {
            println!("Attempted to draw outside of framebuffer bounds at ({}, {})", x, y);
        }
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, font: &Font, scale: f32) {
        let scale = Scale::uniform(scale);
        let v_metrics = font.v_metrics(scale);

        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, point(x as f32, y as f32 + v_metrics.ascent)).collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                if bounding_box.min.x < 0 || bounding_box.min.y < 0 || 
                   bounding_box.max.x as usize > self.width || bounding_box.max.y as usize > self.height {
                    println!("Glyph bounding box out of framebuffer bounds: {:?}", bounding_box);
                    continue;
                }
                
                glyph.draw(|gx, gy, v| {
                    let px = gx as i32 + bounding_box.min.x;
                    let py = gy as i32 + bounding_box.min.y;
                    if px >= 0 && py >= 0 && (px as usize) < self.width && (py as usize) < self.height {
                        if v > 0.5 {
                            self.point(px as usize, py as usize);
                        }
                    }
                });
            }
        }
    }
}
pub struct Texture {
    pub data: Vec<u32>, // Datos de la textura en formato RGB
    pub width: usize,   // Ancho de la textura
    pub height: usize,  // Alto de la textura
}

impl Texture {
    pub fn load_from_file(file_path: &str) -> Result<Texture, String> {
        let img = image::open(file_path).map_err(|e| e.to_string())?;
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();
        let data = img.into_raw();

        // Convierte el formato de imagen a un vector de u32 en formato RGBA
        let mut texture_data = Vec::with_capacity((width * height) as usize);
        for pixel in data.chunks_exact(3) {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            let a = 255;
            let rgba = (a << 24) | (r << 16) | (g << 8) | b;
            texture_data.push(rgba);
        }

        Ok(Texture {
            data: texture_data,
            width: width as usize,
            height: height as usize,
        })
    }
}
