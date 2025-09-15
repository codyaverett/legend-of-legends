pub mod components;
pub mod clock;
pub mod debug;
pub mod font;
pub mod layout;
pub mod minimap;

pub use components::*;
pub use clock::*;
pub use debug::*;
pub use layout::*;
pub use minimap::*;

use glam::Vec2;
use crate::engine::core::Color;

pub trait UIElement {
    fn update(&mut self, delta_time: f32);
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, position: Vec2);
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
}

#[derive(Debug, Clone, Copy)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    pub fn calculate_position(&self, screen_size: Vec2, element_size: Vec2, offset: Vec2) -> Vec2 {
        let base_pos = match self {
            Anchor::TopLeft => Vec2::new(0.0, 0.0),
            Anchor::TopCenter => Vec2::new(screen_size.x / 2.0 - element_size.x / 2.0, 0.0),
            Anchor::TopRight => Vec2::new(screen_size.x - element_size.x, 0.0),
            Anchor::MiddleLeft => Vec2::new(0.0, screen_size.y / 2.0 - element_size.y / 2.0),
            Anchor::Center => Vec2::new(
                screen_size.x / 2.0 - element_size.x / 2.0,
                screen_size.y / 2.0 - element_size.y / 2.0,
            ),
            Anchor::MiddleRight => Vec2::new(
                screen_size.x - element_size.x,
                screen_size.y / 2.0 - element_size.y / 2.0,
            ),
            Anchor::BottomLeft => Vec2::new(0.0, screen_size.y - element_size.y),
            Anchor::BottomCenter => Vec2::new(
                screen_size.x / 2.0 - element_size.x / 2.0,
                screen_size.y - element_size.y,
            ),
            Anchor::BottomRight => Vec2::new(
                screen_size.x - element_size.x,
                screen_size.y - element_size.y,
            ),
        };
        base_pos + offset
    }
}