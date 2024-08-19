extern crate rusttype;

use rusttype::{Font, Scale, point};
use crate::framebuffer::Framebuffer;
use crate::color::Color;

pub struct TextRenderer {
    font: Font<'static>,
}

impl TextRenderer {
    pub fn new(font_path: &str) -> Self {
        let font_data: Vec<u8> = std::fs::read(font_path).expect("Error al leer la fuente TTF");
        let font = Font::try_from_vec(font_data).expect("Error al cargar la fuente");
        Self { font }
    }

    pub fn render_fps(&self, framebuffer: &mut Framebuffer, fps: u32) {
        let scale = Scale::uniform(12.0);
        let fps_text = format!("FPS: {}", fps);
        
        // Ajustar la posición para la esquina superior derecha
        let x = framebuffer.get_width() as i32 - 100;
        let y = 20;
    
        for glyph in self.font.layout(&fps_text, scale, point(x as f32, y as f32)) {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let x = gx as i32 + bounding_box.min.x;
                    let y = gy as i32 + bounding_box.min.y;
    
                    if x >= 0 && x < framebuffer.get_width() as i32 && y >= 0 && y < framebuffer.get_height() as i32 {
                        let alpha = (v * 255.0) as u8;
                        let color = Color::new(alpha, alpha, alpha); // Modificar para respetar la transparencia
                        framebuffer.set_current_color(color);
                        framebuffer.point(x as isize, y as isize);
                    }
                });
            }
        }
    }
}    