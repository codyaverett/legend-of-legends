use glam::Vec2;
use crate::engine::core::Color;
use super::{UIElement, Anchor, Panel, Text};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct DebugOverlay {
    pub position: Vec2,
    pub background: Panel,
    pub texts: Vec<Text>,
    pub visible: bool,
    pub anchor: Anchor,
    fps_samples: VecDeque<f32>,
    frame_counter: u32,
}

impl DebugOverlay {
    pub fn new(position: Vec2) -> Self {
        let size = Vec2::new(250.0, 150.0);
        
        let mut background = Panel::new(position, size);
        background.background_color = Color::new(0, 0, 0, 200);
        background.border_color = Color::new(0, 255, 0, 100);
        background.border_width = 1.0;
        
        let mut texts = Vec::new();
        for i in 0..6 {
            let mut text = Text::new(
                position + Vec2::new(10.0, 10.0 + i as f32 * 20.0),
                String::new(),
            );
            text.size = 14;
            text.color = Color::new(0, 255, 0, 255);
            texts.push(text);
        }
        
        Self {
            position,
            background,
            texts,
            visible: false,
            anchor: Anchor::TopRight,
            fps_samples: VecDeque::with_capacity(60),
            frame_counter: 0,
        }
    }

    pub fn update_stats(
        &mut self,
        delta_time: f32,
        entity_count: usize,
        player_pos: Option<Vec2>,
        player_velocity: Option<Vec2>,
    ) {
        self.frame_counter += 1;
        
        let fps = if delta_time > 0.0 {
            1.0 / delta_time
        } else {
            0.0
        };
        
        self.fps_samples.push_back(fps);
        if self.fps_samples.len() > 60 {
            self.fps_samples.pop_front();
        }
        
        let avg_fps = if !self.fps_samples.is_empty() {
            self.fps_samples.iter().sum::<f32>() / self.fps_samples.len() as f32
        } else {
            0.0
        };
        
        if self.texts.len() >= 6 {
            self.texts[0].content = format!("FPS: {:.1} (avg: {:.1})", fps, avg_fps);
            self.texts[1].content = format!("Frame Time: {:.2}ms", delta_time * 1000.0);
            self.texts[2].content = format!("Entities: {}", entity_count);
            self.texts[3].content = format!("Frame: {}", self.frame_counter);
            
            if let Some(pos) = player_pos {
                self.texts[4].content = format!("Player: ({:.0}, {:.0})", pos.x, pos.y);
            } else {
                self.texts[4].content = "Player: N/A".to_string();
            }
            
            if let Some(vel) = player_velocity {
                self.texts[5].content = format!("Velocity: ({:.0}, {:.0})", vel.x, vel.y);
            } else {
                self.texts[5].content = "Velocity: N/A".to_string();
            }
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

impl UIElement for DebugOverlay {
    fn update(&mut self, delta_time: f32) {
        self.background.update(delta_time);
        for text in &mut self.texts {
            text.update(delta_time);
        }
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.background.position = position;
        for (i, text) in self.texts.iter_mut().enumerate() {
            text.position = position + Vec2::new(10.0, 10.0 + i as f32 * 20.0);
        }
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        self.background.visible = visible;
        for text in &mut self.texts {
            text.visible = visible;
        }
    }
}