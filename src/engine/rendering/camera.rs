use glam::Vec2;

pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
    pub viewport_size: Vec2,
    target_zoom: f32,
    zoom_speed: f32,
    pub pixel_perfect: bool,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,  // Normal zoom to start
            viewport_size: Vec2::new(width, height),
            target_zoom: 1.0,
            zoom_speed: 2.0,
            pixel_perfect: true,  // Enable pixel-perfect rendering by default
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let zoom_diff = self.target_zoom - self.zoom;
        if zoom_diff.abs() > 0.01 {
            self.zoom += zoom_diff * self.zoom_speed * delta_time;
        }
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.target_zoom = zoom.clamp(0.1, 5.0);
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let relative = (world_pos - self.position) * self.zoom;
        let screen_pos = relative + self.viewport_size / 2.0;
        
        // Apply pixel-perfect rounding if enabled
        if self.pixel_perfect {
            Vec2::new(screen_pos.x.round(), screen_pos.y.round())
        } else {
            screen_pos
        }
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let relative = screen_pos - self.viewport_size / 2.0;
        relative / self.zoom + self.position
    }

    pub fn follow(&mut self, target: Vec2, smoothing: f32, delta_time: f32) {
        let diff = target - self.position;
        let new_pos = self.position + diff * smoothing * delta_time;
        
        // Round camera position for pixel-perfect rendering to prevent sub-pixel movement
        if self.pixel_perfect {
            self.position = Vec2::new(
                (new_pos.x * self.zoom).round() / self.zoom,
                (new_pos.y * self.zoom).round() / self.zoom
            );
        } else {
            self.position = new_pos;
        }
    }
}
