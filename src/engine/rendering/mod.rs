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

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
