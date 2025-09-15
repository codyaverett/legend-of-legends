use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Defines rendering layers for depth-based drawing order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RenderLayer {
    /// Furthest background (sky, distant mountains)
    FarBackground = 0,
    /// Mid-distance background (city skyline)
    MidBackground = 1,
    /// Near background (buildings behind gameplay)
    NearBackground = 2,
    /// Main gameplay layer (tiles, platforms)
    Gameplay = 3,
    /// Entities (player, enemies, items)
    Entities = 4,
    /// Foreground decorations
    Foreground = 5,
    /// UI and overlay elements
    UI = 6,
}

impl Default for RenderLayer {
    fn default() -> Self {
        Self::Gameplay
    }
}

/// Configuration for parallax scrolling effects
#[derive(Debug, Clone, Copy)]
pub struct ParallaxConfig {
    /// Horizontal scroll factor (0.0 = static, 1.0 = normal scroll)
    pub scroll_factor_x: f32,
    /// Vertical scroll factor
    pub scroll_factor_y: f32,
    /// Optional auto-scroll speed for animated backgrounds
    pub auto_scroll_x: f32,
    pub auto_scroll_y: f32,
}

impl ParallaxConfig {
    pub fn new(scroll_factor_x: f32, scroll_factor_y: f32) -> Self {
        Self {
            scroll_factor_x,
            scroll_factor_y,
            auto_scroll_x: 0.0,
            auto_scroll_y: 0.0,
        }
    }

    pub fn static_layer() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn with_auto_scroll(mut self, x: f32, y: f32) -> Self {
        self.auto_scroll_x = x;
        self.auto_scroll_y = y;
        self
    }
}

impl RenderLayer {
    /// Get default parallax configuration for this layer
    pub fn default_parallax(&self) -> ParallaxConfig {
        match self {
            Self::FarBackground => ParallaxConfig::new(0.1, 0.05),
            Self::MidBackground => ParallaxConfig::new(0.3, 0.2),
            Self::NearBackground => ParallaxConfig::new(0.6, 0.5),
            Self::Gameplay => ParallaxConfig::new(1.0, 1.0),
            Self::Entities => ParallaxConfig::new(1.0, 1.0),
            Self::Foreground => ParallaxConfig::new(1.2, 1.1),
            Self::UI => ParallaxConfig::static_layer(),
        }
    }

    /// Get atmospheric tint color for depth perception
    pub fn get_atmosphere_tint(&self, base_color: crate::engine::core::Color) -> crate::engine::core::Color {
        use crate::engine::core::Color;
        
        match self {
            Self::FarBackground => {
                // Add blue haze for distance
                Color::new(
                    (base_color.r as f32 * 0.7 + 50.0) as u8,
                    (base_color.g as f32 * 0.7 + 60.0) as u8,
                    (base_color.b as f32 * 0.8 + 70.0) as u8,
                    (base_color.a as f32 * 0.8) as u8,
                )
            }
            Self::MidBackground => {
                // Slight blue tint
                Color::new(
                    (base_color.r as f32 * 0.85 + 20.0) as u8,
                    (base_color.g as f32 * 0.85 + 25.0) as u8,
                    (base_color.b as f32 * 0.9 + 30.0) as u8,
                    (base_color.a as f32 * 0.9) as u8,
                )
            }
            Self::NearBackground => {
                // Very slight tint
                Color::new(
                    (base_color.r as f32 * 0.95) as u8,
                    (base_color.g as f32 * 0.95) as u8,
                    (base_color.b as f32 * 0.97) as u8,
                    base_color.a,
                )
            }
            _ => base_color,
        }
    }
}

/// Component for layered rendering
#[derive(Debug, Clone)]
pub struct LayeredSprite {
    pub sprite: crate::engine::rendering::Sprite,
    pub layer: RenderLayer,
    pub parallax_override: Option<ParallaxConfig>,
}

impl LayeredSprite {
    pub fn new(sprite: crate::engine::rendering::Sprite, layer: RenderLayer) -> Self {
        Self {
            sprite,
            layer,
            parallax_override: None,
        }
    }

    pub fn with_parallax(mut self, config: ParallaxConfig) -> Self {
        self.parallax_override = Some(config);
        self
    }

    pub fn get_parallax(&self) -> ParallaxConfig {
        self.parallax_override.unwrap_or_else(|| self.layer.default_parallax())
    }

    /// Calculate rendered position with parallax effect
    pub fn calculate_position(&self, world_pos: Vec2, camera_offset: Vec2, elapsed_time: f32) -> Vec2 {
        let parallax = self.get_parallax();
        
        let parallax_offset = Vec2::new(
            camera_offset.x * (1.0 - parallax.scroll_factor_x),
            camera_offset.y * (1.0 - parallax.scroll_factor_y),
        );
        
        let auto_scroll = Vec2::new(
            parallax.auto_scroll_x * elapsed_time,
            parallax.auto_scroll_y * elapsed_time,
        );
        
        world_pos + parallax_offset + auto_scroll
    }
}