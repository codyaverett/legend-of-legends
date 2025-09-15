use glam::Vec2;
use crate::engine::core::Color;
use super::{UIElement, Anchor};

#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub position: Vec2,
    pub size: Vec2,
    pub value: f32,
    pub max_value: f32,
    pub background_color: Color,
    pub fill_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub visible: bool,
    pub anchor: Anchor,
    pub animated_value: f32,
    pub animation_speed: f32,
}

impl ProgressBar {
    pub fn new(position: Vec2, size: Vec2, max_value: f32) -> Self {
        Self {
            position,
            size,
            value: max_value,
            max_value,
            background_color: Color::new(40, 40, 40, 200),
            fill_color: Color::new(0, 255, 0, 255),
            border_color: Color::new(255, 255, 255, 255),
            border_width: 2.0,
            visible: true,
            anchor: Anchor::TopLeft,
            animated_value: max_value,
            animation_speed: 5.0,
        }
    }

    pub fn health_bar(position: Vec2, size: Vec2, max_health: f32) -> Self {
        let mut bar = Self::new(position, size, max_health);
        bar.fill_color = Color::new(220, 20, 60, 255);
        bar.background_color = Color::new(60, 10, 10, 200);
        bar
    }

    pub fn energy_bar(position: Vec2, size: Vec2, max_energy: f32) -> Self {
        let mut bar = Self::new(position, size, max_energy);
        bar.fill_color = Color::new(64, 156, 255, 255);
        bar.background_color = Color::new(10, 30, 60, 200);
        bar
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(0.0, self.max_value);
    }

    pub fn get_fill_percentage(&self) -> f32 {
        self.animated_value / self.max_value
    }

    pub fn get_fill_width(&self) -> f32 {
        self.size.x * self.get_fill_percentage()
    }
}

impl UIElement for ProgressBar {
    fn update(&mut self, delta_time: f32) {
        let diff = self.value - self.animated_value;
        if diff.abs() > 0.01 {
            self.animated_value += diff * self.animation_speed * delta_time;
        } else {
            self.animated_value = self.value;
        }
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[derive(Debug, Clone)]
pub struct Panel {
    pub position: Vec2,
    pub size: Vec2,
    pub background_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub visible: bool,
    pub anchor: Anchor,
}

impl Panel {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size,
            background_color: Color::new(0, 0, 0, 180),
            border_color: Color::new(255, 255, 255, 100),
            border_width: 1.0,
            visible: true,
            anchor: Anchor::TopLeft,
        }
    }
}

impl UIElement for Panel {
    fn update(&mut self, _delta_time: f32) {}

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub position: Vec2,
    pub content: String,
    pub color: Color,
    pub size: u32,
    pub visible: bool,
    pub anchor: Anchor,
}

impl Text {
    pub fn new(position: Vec2, content: String) -> Self {
        Self {
            position,
            content,
            color: Color::new(255, 255, 255, 255),
            size: 16,
            visible: true,
            anchor: Anchor::TopLeft,
        }
    }
}

impl UIElement for Text {
    fn update(&mut self, _delta_time: f32) {}

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}