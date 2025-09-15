use crate::engine::core::{Color, Rect};
use glam::Vec2;

pub struct DayNightCycle {
    pub time: f32,  // 0.0 to 120.0 (60s day + 60s night)
    pub cycle_duration: f32,  // Total cycle duration in seconds
    pub is_day: bool,
}

impl DayNightCycle {
    pub fn new() -> Self {
        Self {
            time: 30.0,  // Start at noon
            cycle_duration: 120.0,  // 2 minutes total (1 min day, 1 min night)
            is_day: true,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
        if self.time >= self.cycle_duration {
            self.time -= self.cycle_duration;
        }
        self.is_day = self.time < 60.0;
    }

    // Get normalized time (0.0 to 1.0) within current phase
    pub fn get_phase_progress(&self) -> f32 {
        if self.is_day {
            self.time / 60.0
        } else {
            (self.time - 60.0) / 60.0
        }
    }

    // Generate dynamic sky gradient based on time of day
    pub fn generate_sky_gradient(&self, width: f32, height: f32) -> Vec<(Rect, Color)> {
        let mut gradient = Vec::new();
        let bands = 100;  // Much more detailed gradient with 100 bands
        let band_height = height / bands as f32;
        
        let phase = self.get_phase_progress();
        
        for i in 0..bands {
            let y = i as f32 * band_height;
            let t = i as f32 / bands as f32;
            
            let color = if self.is_day {
                // Day cycle: dawn -> noon -> dusk
                if phase < 0.25 {
                    // Dawn (0.0 - 0.25): Dark purple/orange to light blue
                    let dawn_t = phase * 4.0;
                    self.interpolate_color(
                        self.get_dawn_color(t),
                        self.get_morning_color(t),
                        dawn_t
                    )
                } else if phase < 0.5 {
                    // Morning to Noon (0.25 - 0.5): Light blue to bright blue
                    let morning_t = (phase - 0.25) * 4.0;
                    self.interpolate_color(
                        self.get_morning_color(t),
                        self.get_noon_color(t),
                        morning_t
                    )
                } else if phase < 0.75 {
                    // Afternoon (0.5 - 0.75): Bright blue to golden
                    let afternoon_t = (phase - 0.5) * 4.0;
                    self.interpolate_color(
                        self.get_noon_color(t),
                        self.get_sunset_color(t),
                        afternoon_t
                    )
                } else {
                    // Sunset (0.75 - 1.0): Golden/orange to purple
                    let sunset_t = (phase - 0.75) * 4.0;
                    self.interpolate_color(
                        self.get_sunset_color(t),
                        self.get_dusk_color(t),
                        sunset_t
                    )
                }
            } else {
                // Night cycle: dusk -> midnight -> dawn
                if phase < 0.5 {
                    // Evening to Midnight (0.0 - 0.5): Purple to dark blue/black
                    let evening_t = phase * 2.0;
                    self.interpolate_color(
                        self.get_dusk_color(t),
                        self.get_midnight_color(t),
                        evening_t
                    )
                } else {
                    // Midnight to Dawn (0.5 - 1.0): Dark blue/black to purple/orange
                    let predawn_t = (phase - 0.5) * 2.0;
                    self.interpolate_color(
                        self.get_midnight_color(t),
                        self.get_dawn_color(t),
                        predawn_t
                    )
                }
            };
            
            gradient.push((
                Rect::new(0.0, y, width, band_height),
                color
            ));
        }
        
        gradient
    }

    fn get_dawn_color(&self, t: f32) -> Color {
        // Smooth gradient from top to horizon
        let t = t.clamp(0.0, 1.0);
        
        // Define key colors for dawn
        let top_color = (25, 15, 45);  // Very dark purple-blue
        let upper_mid = (45, 25, 70);  // Dark purple
        let mid_color = (90, 45, 100); // Purple with hint of pink
        let lower_mid = (140, 70, 110); // Pink-purple
        let horizon = (200, 100, 120);  // Orange-pink
        
        if t < 0.25 {
            let local_t = t / 0.25;
            self.interpolate_rgb(top_color, upper_mid, local_t)
        } else if t < 0.5 {
            let local_t = (t - 0.25) / 0.25;
            self.interpolate_rgb(upper_mid, mid_color, local_t)
        } else if t < 0.75 {
            let local_t = (t - 0.5) / 0.25;
            self.interpolate_rgb(mid_color, lower_mid, local_t)
        } else {
            let local_t = (t - 0.75) / 0.25;
            self.interpolate_rgb(lower_mid, horizon, local_t)
        }
    }

    fn get_morning_color(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        
        // Smooth morning gradient
        let top_color = (100, 140, 200);  // Soft blue
        let upper_mid = (120, 160, 220);  // Light blue
        let mid_color = (140, 180, 235);  // Brighter blue
        let lower_mid = (160, 195, 245);  // Very light blue
        let horizon = (190, 210, 250);    // Pale blue-white
        
        if t < 0.25 {
            let local_t = t / 0.25;
            self.interpolate_rgb(top_color, upper_mid, local_t)
        } else if t < 0.5 {
            let local_t = (t - 0.25) / 0.25;
            self.interpolate_rgb(upper_mid, mid_color, local_t)
        } else if t < 0.75 {
            let local_t = (t - 0.5) / 0.25;
            self.interpolate_rgb(mid_color, lower_mid, local_t)
        } else {
            let local_t = (t - 0.75) / 0.25;
            self.interpolate_rgb(lower_mid, horizon, local_t)
        }
    }

    fn get_noon_color(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        
        // Clear noon sky gradient
        let top_color = (50, 110, 230);   // Deep azure
        let upper_mid = (70, 130, 245);   // Rich blue
        let mid_color = (90, 150, 255);   // Bright blue
        let lower_mid = (120, 170, 255);  // Sky blue
        let horizon = (160, 200, 255);    // Light sky blue
        
        if t < 0.25 {
            let local_t = t / 0.25;
            self.interpolate_rgb(top_color, upper_mid, local_t)
        } else if t < 0.5 {
            let local_t = (t - 0.25) / 0.25;
            self.interpolate_rgb(upper_mid, mid_color, local_t)
        } else if t < 0.75 {
            let local_t = (t - 0.5) / 0.25;
            self.interpolate_rgb(mid_color, lower_mid, local_t)
        } else {
            let local_t = (t - 0.75) / 0.25;
            self.interpolate_rgb(lower_mid, horizon, local_t)
        }
    }

    fn get_sunset_color(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        
        // Dramatic sunset gradient
        let top_color = (70, 90, 150);    // Blue-purple
        let upper_mid = (120, 100, 160);  // Purple with warmth
        let mid_color = (200, 140, 120);  // Golden orange
        let lower_mid = (240, 160, 90);   // Bright orange
        let horizon = (255, 140, 60);     // Deep orange-red
        
        if t < 0.25 {
            let local_t = t / 0.25;
            self.interpolate_rgb(top_color, upper_mid, local_t)
        } else if t < 0.5 {
            let local_t = (t - 0.25) / 0.25;
            self.interpolate_rgb(upper_mid, mid_color, local_t)
        } else if t < 0.75 {
            let local_t = (t - 0.5) / 0.25;
            self.interpolate_rgb(mid_color, lower_mid, local_t)
        } else {
            let local_t = (t - 0.75) / 0.25;
            self.interpolate_rgb(lower_mid, horizon, local_t)
        }
    }

    fn get_dusk_color(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        
        // Twilight gradient
        let top_color = (40, 30, 80);     // Dark purple-blue
        let upper_mid = (60, 45, 100);    // Purple
        let mid_color = (80, 60, 120);    // Lighter purple
        let lower_mid = (100, 75, 135);   // Purple-pink
        let horizon = (130, 90, 150);     // Dusky pink
        
        if t < 0.25 {
            let local_t = t / 0.25;
            self.interpolate_rgb(top_color, upper_mid, local_t)
        } else if t < 0.5 {
            let local_t = (t - 0.25) / 0.25;
            self.interpolate_rgb(upper_mid, mid_color, local_t)
        } else if t < 0.75 {
            let local_t = (t - 0.5) / 0.25;
            self.interpolate_rgb(mid_color, lower_mid, local_t)
        } else {
            let local_t = (t - 0.75) / 0.25;
            self.interpolate_rgb(lower_mid, horizon, local_t)
        }
    }

    fn get_midnight_color(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        
        // Night sky gradient with stars hint
        let top_color = (5, 5, 20);       // Almost black with blue tint
        let upper_mid = (10, 10, 35);     // Very dark blue
        let mid_color = (15, 15, 50);     // Dark blue
        let lower_mid = (25, 25, 65);     // Slightly lighter blue
        let horizon = (35, 35, 80);       // Dark horizon blue
        
        if t < 0.25 {
            let local_t = t / 0.25;
            self.interpolate_rgb(top_color, upper_mid, local_t)
        } else if t < 0.5 {
            let local_t = (t - 0.25) / 0.25;
            self.interpolate_rgb(upper_mid, mid_color, local_t)
        } else if t < 0.75 {
            let local_t = (t - 0.5) / 0.25;
            self.interpolate_rgb(mid_color, lower_mid, local_t)
        } else {
            let local_t = (t - 0.75) / 0.25;
            self.interpolate_rgb(lower_mid, horizon, local_t)
        }
    }

    fn interpolate_rgb(&self, from: (u8, u8, u8), to: (u8, u8, u8), t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        // Use smooth step for even smoother transitions
        let t = t * t * (3.0 - 2.0 * t);
        
        Color::new(
            (from.0 as f32 * (1.0 - t) + to.0 as f32 * t) as u8,
            (from.1 as f32 * (1.0 - t) + to.1 as f32 * t) as u8,
            (from.2 as f32 * (1.0 - t) + to.2 as f32 * t) as u8,
            255
        )
    }

    fn interpolate_color(&self, from: Color, to: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color::new(
            (from.r as f32 * (1.0 - t) + to.r as f32 * t) as u8,
            (from.g as f32 * (1.0 - t) + to.g as f32 * t) as u8,
            (from.b as f32 * (1.0 - t) + to.b as f32 * t) as u8,
            255
        )
    }

    // Get ambient light color for affecting other elements
    pub fn get_ambient_light(&self) -> Color {
        let phase = self.get_phase_progress();
        
        if self.is_day {
            if phase < 0.25 {
                // Dawn
                Color::new(150, 120, 180, 255)
            } else if phase < 0.75 {
                // Day
                Color::new(255, 255, 255, 255)
            } else {
                // Sunset
                Color::new(255, 180, 120, 255)
            }
        } else {
            if phase < 0.5 {
                // Evening
                Color::new(100, 100, 150, 255)
            } else {
                // Night
                Color::new(50, 50, 100, 255)
            }
        }
    }
}