use crate::engine::core::{Color, Rect};
use glam::Vec2;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Procedural mock asset generator for prototyping
pub struct MockAssetGenerator {
    rng: StdRng,
}

impl MockAssetGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Generate a gradient sky background
    pub fn generate_sky_gradient(&self, width: f32, height: f32) -> Vec<(Rect, Color)> {
        let mut strips = Vec::new();
        let strip_height = height / 10.0;
        
        for i in 0..10 {
            let y = i as f32 * strip_height;
            let t = i as f32 / 9.0;
            
            // Gradient from light blue to darker blue
            let color = Color::new(
                (135.0 - t * 50.0) as u8,  // R: 135 -> 85
                (206.0 - t * 56.0) as u8,  // G: 206 -> 150
                (235.0 - t * 35.0) as u8,  // B: 235 -> 200
                255,
            );
            
            strips.push((
                Rect::new(0.0, y, width, strip_height),
                color,
            ));
        }
        
        strips
    }

    /// Generate mountain silhouettes for far background
    pub fn generate_mountains(&mut self, width: f32, base_y: f32) -> Vec<(Vec2, Vec2, Color)> {
        let mut mountains = Vec::new();
        let mountain_count = 5;
        let segment_width = width / mountain_count as f32;
        
        for i in 0..mountain_count {
            let x = i as f32 * segment_width;
            let peak_height = self.rng.gen_range(100.0..250.0);
            let peak_x = x + segment_width * self.rng.gen_range(0.3..0.7);
            
            // Mountain color (dark purple/blue)
            let color = Color::new(40, 35, 60, 200);
            
            // Simple triangle mountain
            mountains.push((
                Vec2::new(x, base_y),
                Vec2::new(peak_x, base_y - peak_height),
                color,
            ));
            mountains.push((
                Vec2::new(peak_x, base_y - peak_height),
                Vec2::new(x + segment_width, base_y),
                color,
            ));
        }
        
        mountains
    }

    /// Generate building silhouettes
    pub fn generate_building(&mut self, x: f32, ground_y: f32, style: BuildingStyle) -> BuildingAsset {
        // Scale up buildings 3-4x to be more realistic relative to 40-pixel tall player
        let (width, height, color) = match style {
            BuildingStyle::Skyscraper => {
                let w = self.rng.gen_range(200.0..350.0);  // Wide enough for multiple rooms
                let h = self.rng.gen_range(600.0..1200.0);  // 15-30 stories tall
                let gray = self.rng.gen_range(40..70);
                (w, h, Color::new(gray, gray, gray + 5, 255))
            }
            BuildingStyle::Industrial => {
                let w = self.rng.gen_range(300.0..500.0);  // Large industrial buildings
                let h = self.rng.gen_range(200.0..400.0);  // 5-10 stories
                let gray = self.rng.gen_range(60..80);
                (w, h, Color::new(gray + 10, gray, gray - 5, 255))
            }
            BuildingStyle::Residential => {
                let w = self.rng.gen_range(150.0..250.0);  // Apartment building width
                let h = self.rng.gen_range(300.0..500.0);  // 7-12 stories
                let gray = self.rng.gen_range(70..90);
                (w, h, Color::new(gray + 15, gray + 10, gray, 255))
            }
            BuildingStyle::Office => {
                let w = self.rng.gen_range(250.0..400.0);  // Office building width
                let h = self.rng.gen_range(400.0..800.0);  // 10-20 stories
                let gray = self.rng.gen_range(50..65);
                (w, h, Color::new(gray, gray + 10, gray + 20, 255))
            }
        };

        let windows = self.generate_windows(width, height, &style);
        
        BuildingAsset {
            position: Vec2::new(x, ground_y - height),
            size: Vec2::new(width, height),
            base_color: color,
            windows,
            style,
        }
    }

    /// Generate window patterns for buildings
    fn generate_windows(&mut self, width: f32, height: f32, style: &BuildingStyle) -> Vec<Rect> {
        let mut windows = Vec::new();
        
        // Scale up windows to match building scale (roughly 3x larger)
        let (window_width, window_height, h_spacing, v_spacing) = match style {
            BuildingStyle::Skyscraper => (20.0, 30.0, 35.0, 45.0),  // Floor-to-ceiling windows
            BuildingStyle::Office => (25.0, 35.0, 40.0, 50.0),  // Large office windows
            BuildingStyle::Industrial => (40.0, 25.0, 60.0, 40.0),  // Wide industrial windows
            BuildingStyle::Residential => (20.0, 25.0, 35.0, 40.0),  // Apartment windows
        };
        
        let margin = 20.0;
        let mut y = margin;
        
        while y + window_height < height - margin {
            let mut x = margin;
            while x + window_width < width - margin {
                // Randomly light some windows
                if self.rng.gen_bool(0.7) {
                    windows.push(Rect::new(x, y, window_width, window_height));
                }
                x += h_spacing;
            }
            y += v_spacing;
        }
        
        windows
    }

    /// Generate simple cloud shapes
    pub fn generate_cloud(&mut self, x: f32, y: f32) -> Vec<(Vec2, f32, Color)> {
        let mut circles = Vec::new();
        let cloud_color = Color::new(255, 255, 255, 180);
        let puff_count = self.rng.gen_range(3..6);
        
        for i in 0..puff_count {
            let offset_x = i as f32 * 25.0 + self.rng.gen_range(-10.0..10.0);
            let offset_y = self.rng.gen_range(-15.0..15.0);
            let radius = self.rng.gen_range(20.0..35.0);
            
            circles.push((
                Vec2::new(x + offset_x, y + offset_y),
                radius,
                cloud_color,
            ));
        }
        
        circles
    }

    /// Generate street decoration (lamp posts, signs, etc.)
    pub fn generate_street_prop(&mut self, x: f32, ground_y: f32, prop_type: StreetPropType) -> StreetProp {
        match prop_type {
            StreetPropType::LampPost => {
                StreetProp {
                    position: Vec2::new(x, ground_y - 120.0),  // 3x taller (3 person heights)
                    size: Vec2::new(15.0, 120.0),  // Thicker and taller
                    color: Color::new(60, 60, 65, 255),
                    prop_type,
                    details: vec![
                        // Lamp head
                        (Vec2::new(-25.0, -10.0), Vec2::new(50.0, 30.0), Color::new(255, 255, 200, 200)),
                    ],
                }
            }
            StreetPropType::TrashCan => {
                StreetProp {
                    position: Vec2::new(x, ground_y - 30.0),  // Waist-high to player
                    size: Vec2::new(25.0, 30.0),  // Realistic trash can size
                    color: Color::new(40, 45, 40, 255),
                    prop_type,
                    details: vec![],
                }
            }
            StreetPropType::Sign => {
                StreetProp {
                    position: Vec2::new(x, ground_y - 100.0),  // Tall street sign
                    size: Vec2::new(10.0, 100.0),  // Thicker pole
                    color: Color::new(70, 70, 75, 255),
                    prop_type,
                    details: vec![
                        // Sign board
                        (Vec2::new(-40.0, -20.0), Vec2::new(80.0, 40.0), Color::new(200, 50, 50, 255)),
                    ],
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BuildingStyle {
    Skyscraper,
    Industrial,
    Residential,
    Office,
}

#[derive(Debug, Clone)]
pub struct BuildingAsset {
    pub position: Vec2,
    pub size: Vec2,
    pub base_color: Color,
    pub windows: Vec<Rect>,
    pub style: BuildingStyle,
}

impl BuildingAsset {
    pub fn get_window_color(&self, lit: bool) -> Color {
        if lit {
            Color::new(255, 240, 180, 200)  // Warm yellow light
        } else {
            Color::new(20, 25, 30, 200)  // Dark window
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StreetPropType {
    LampPost,
    TrashCan,
    Sign,
}

#[derive(Debug, Clone)]
pub struct StreetProp {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub prop_type: StreetPropType,
    pub details: Vec<(Vec2, Vec2, Color)>,  // Additional decorative elements
}