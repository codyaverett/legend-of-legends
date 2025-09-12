use glam::Vec2;
use crate::engine::core::{Color, Rect};

#[derive(Debug, Clone)]
pub struct Sprite {
    pub size: Vec2,
    pub color: Color,
    pub source_rect: Option<Rect>,
}

impl Sprite {
    pub fn new(size: Vec2, color: Color) -> Self {
        Self {
            size,
            color,
            source_rect: None,
        }
    }
    
    pub fn with_source_rect(mut self, rect: Rect) -> Self {
        self.source_rect = Some(rect);
        self
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            size: Vec2::new(32.0, 32.0),
            color: Color::WHITE,
            source_rect: None,
        }
    }
}