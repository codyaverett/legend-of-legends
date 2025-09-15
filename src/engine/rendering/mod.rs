pub mod camera;
pub mod layers;
pub mod mock_assets;
pub mod sprite;

use crate::engine::platform::Platform;
use anyhow::Result;

pub use camera::*;
pub use layers::*;
pub use mock_assets::*;
pub use sprite::*;

pub struct Renderer {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub camera: Camera,
}

impl Renderer {
    pub fn new(platform: &Platform) -> Result<Self> {
        let canvas = platform.window.canvas()?;
        let (width, height) = platform.window.size();
        let camera = Camera::new(width as f32, height as f32);

        Ok(Self { canvas, camera })
    }

    pub fn clear(&mut self, color: crate::engine::core::Color) {
        self.canvas.set_draw_color(color.to_sdl());
        self.canvas.clear();
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, transform: &crate::engine::core::Transform) {
        let screen_pos = self.camera.world_to_screen(transform.position);
        let size = sprite.size * transform.scale * self.camera.zoom;

        // Round positions to prevent sub-pixel gaps and add 1 pixel overlap for tiles
        let x = (screen_pos.x - size.x / 2.0).round() as i32;
        let y = (screen_pos.y - size.y / 2.0).round() as i32;
        // Ceiling the size to ensure full coverage (prevents gaps)
        let width = size.x.ceil() as u32;
        let height = size.y.ceil() as u32;

        let dest = sdl2::rect::Rect::new(x, y, width, height);

        self.canvas.set_draw_color(sprite.color.to_sdl());
        let _ = self.canvas.fill_rect(dest);
    }

    pub fn draw_ui_rect(&mut self, position: glam::Vec2, size: glam::Vec2, color: crate::engine::core::Color) {
        let x = position.x.round() as i32;
        let y = position.y.round() as i32;
        let width = size.x.ceil() as u32;
        let height = size.y.ceil() as u32;

        let dest = sdl2::rect::Rect::new(x, y, width, height);
        self.canvas.set_draw_color(color.to_sdl());
        let _ = self.canvas.fill_rect(dest);
    }

    pub fn draw_ui_rect_outline(&mut self, position: glam::Vec2, size: glam::Vec2, color: crate::engine::core::Color, thickness: f32) {
        let x = position.x.round() as i32;
        let y = position.y.round() as i32;
        let width = size.x.ceil() as u32;
        let height = size.y.ceil() as u32;

        self.canvas.set_draw_color(color.to_sdl());
        
        for i in 0..thickness.ceil() as i32 {
            let rect = sdl2::rect::Rect::new(x + i, y + i, width - (i * 2) as u32, height - (i * 2) as u32);
            let _ = self.canvas.draw_rect(rect);
        }
    }

    pub fn draw_ui_text(&mut self, position: glam::Vec2, text: &str, color: crate::engine::core::Color, size: u32) {
        let font = crate::engine::ui::font::BitmapFont::new_5x7();
        let scale = size as f32 / 14.0; // Base size is 14px (7 height * 2)
        
        let mut cursor_x = position.x;
        let cursor_y = position.y;
        
        for ch in text.chars() {
            if let Some(bitmap) = font.get_char_bitmap(ch) {
                for (row_idx, row) in bitmap.iter().enumerate() {
                    for (col_idx, &pixel) in row.iter().enumerate() {
                        if pixel {
                            let x = cursor_x + (col_idx as f32 * scale);
                            let y = cursor_y + (row_idx as f32 * scale);
                            self.draw_ui_rect(
                                glam::Vec2::new(x, y),
                                glam::Vec2::new(scale, scale),
                                color,
                            );
                        }
                    }
                }
            }
            cursor_x += (font.get_char_size().x + 1.0) * scale; // Add spacing between characters
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
