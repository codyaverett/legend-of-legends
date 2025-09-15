use glam::Vec2;
use crate::engine::core::Color;
use crate::game::DayNightCycle;
use super::{UIElement, Anchor, Panel, Text};

#[derive(Debug, Clone)]
pub struct ClockWidget {
    pub position: Vec2,
    pub size: Vec2,
    pub background: Panel,
    pub time_text: Text,
    pub visible: bool,
    pub anchor: Anchor,
}

impl ClockWidget {
    pub fn new(position: Vec2) -> Self {
        let size = Vec2::new(120.0, 40.0);
        
        let mut background = Panel::new(position, size);
        background.background_color = Color::new(0, 0, 0, 150);
        background.border_color = Color::new(200, 200, 200, 100);
        
        let text_pos = position + Vec2::new(10.0, 10.0);
        let mut time_text = Text::new(text_pos, "00:00".to_string());
        time_text.size = 20;
        
        Self {
            position,
            size,
            background,
            time_text,
            visible: true,
            anchor: Anchor::TopCenter,
        }
    }

    pub fn update_time(&mut self, day_night_cycle: &DayNightCycle) {
        let hours_per_cycle = 24.0;
        let time_ratio = day_night_cycle.time / day_night_cycle.cycle_duration;
        let hour_float = time_ratio * hours_per_cycle;
        let hours = hour_float as u32 % 24;
        let minutes = ((hour_float.fract() * 60.0) as u32) % 60;
        
        self.time_text.content = format!("{:02}:{:02}", hours, minutes);
        
        let ambient = day_night_cycle.get_ambient_light();
        self.background.background_color = Color::new(
            (ambient.r as f32 * 0.2) as u8,
            (ambient.g as f32 * 0.2) as u8,
            (ambient.b as f32 * 0.3) as u8,
            150,
        );
        
        if day_night_cycle.is_day {
            self.time_text.color = Color::new(255, 220, 100, 255);
        } else {
            self.time_text.color = Color::new(150, 150, 255, 255);
        }
    }

    pub fn get_time_icon(&self, day_night_cycle: &DayNightCycle) -> &str {
        let hours_per_cycle = 24.0;
        let time_ratio = day_night_cycle.time / day_night_cycle.cycle_duration;
        let hour = (time_ratio * hours_per_cycle) as u32 % 24;
        
        match hour {
            6..=8 => "🌅",
            9..=16 => "☀️",
            17..=19 => "🌇",
            20..=23 | 0..=5 => "🌙",
            _ => "⭐",
        }
    }
}

impl UIElement for ClockWidget {
    fn update(&mut self, delta_time: f32) {
        self.background.update(delta_time);
        self.time_text.update(delta_time);
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.background.position = position;
        self.time_text.position = position + Vec2::new(10.0, 10.0);
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        self.background.visible = visible;
        self.time_text.visible = visible;
    }
}