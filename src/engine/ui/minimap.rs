use crate::engine::core::Color;
use crate::engine::rendering::Renderer;
use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Minimap {
    pub position: Vec2,      // Screen position (top-right corner usually)
    pub size: Vec2,          // Size of the minimap
    pub world_scale: f32,    // How much of the world to show (e.g., 0.05 = 5% scale)
    pub background_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub opacity: u8,         // Overall opacity
}

impl Minimap {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size,
            world_scale: 0.1,  // Show 10% of world scale by default
            background_color: Color::new(20, 20, 30, 180),  // Dark blue-gray
            border_color: Color::new(100, 150, 200, 255),   // Light blue border
            border_width: 2.0,
            opacity: 200,
        }
    }
    
    pub fn render(
        &self,
        renderer: &mut Renderer,
        player_pos: Vec2,
        enemies: &[(Vec2, bool)],  // Position and whether it's a boss
        collectibles: &[Vec2],
        goal_pos: Option<Vec2>,
        camera_offset: Vec2,
    ) {
        // Calculate actual screen position (relative to camera)
        let screen_pos = self.position - camera_offset;
        
        // Draw background using UI rect (not affected by camera)
        renderer.draw_ui_rect(screen_pos, self.size, self.background_color);
        
        // Draw border
        self.draw_border(renderer, screen_pos);
        
        // Calculate minimap center (player is always at center)
        let minimap_center = screen_pos + self.size / 2.0;
        
        // Draw enemies as red dots
        for (enemy_pos, is_boss) in enemies {
            let relative_pos = (*enemy_pos - player_pos) * self.world_scale;
            
            // Check if enemy is within minimap bounds
            if relative_pos.x.abs() < self.size.x / 2.0 && relative_pos.y.abs() < self.size.y / 2.0 {
                let dot_pos = minimap_center + relative_pos;
                let dot_size = if *is_boss { 6.0 } else { 4.0 };
                let dot_color = if *is_boss {
                    Color::new(255, 100, 0, self.opacity)  // Orange for boss
                } else {
                    Color::new(255, 50, 50, self.opacity)   // Red for regular enemies
                };
                
                // Draw enemy dot using UI rect
                renderer.draw_ui_rect(
                    dot_pos - Vec2::splat(dot_size / 2.0),
                    Vec2::splat(dot_size),
                    dot_color
                );
            }
        }
        
        // Draw collectibles as blue dots
        for collectible_pos in collectibles {
            let relative_pos = (*collectible_pos - player_pos) * self.world_scale;
            
            if relative_pos.x.abs() < self.size.x / 2.0 && relative_pos.y.abs() < self.size.y / 2.0 {
                let dot_pos = minimap_center + relative_pos;
                // Draw collectible dot using UI rect
                renderer.draw_ui_rect(
                    dot_pos - Vec2::splat(1.5),
                    Vec2::splat(3.0),
                    Color::new(100, 200, 255, self.opacity)
                );
            }
        }
        
        // Draw goal as green marker
        if let Some(goal) = goal_pos {
            let relative_pos = (goal - player_pos) * self.world_scale;
            
            // Clamp to edge if outside minimap
            let clamped_pos = Vec2::new(
                relative_pos.x.clamp(-self.size.x / 2.0 + 5.0, self.size.x / 2.0 - 5.0),
                relative_pos.y.clamp(-self.size.y / 2.0 + 5.0, self.size.y / 2.0 - 5.0)
            );
            
            let marker_pos = minimap_center + clamped_pos;
            // Draw goal marker using UI rect
            renderer.draw_ui_rect(
                marker_pos - Vec2::splat(3.0),
                Vec2::splat(6.0),
                Color::new(50, 255, 50, self.opacity)
            );
        }
        
        // Draw player at center (white dot) using UI rect
        renderer.draw_ui_rect(
            minimap_center - Vec2::splat(2.5),
            Vec2::splat(5.0),
            Color::new(255, 255, 255, 255)
        );
        
        // Draw direction indicator (small line showing player facing) using UI rect
        // This could be enhanced with actual player facing direction
        renderer.draw_ui_rect(
            minimap_center - Vec2::new(0.5, 8.0),
            Vec2::new(1.0, 8.0),
            Color::new(255, 255, 255, 200)
        );
    }
    
    fn draw_border(&self, renderer: &mut Renderer, screen_pos: Vec2) {
        // Use draw_ui_rect_outline for the border
        renderer.draw_ui_rect_outline(
            screen_pos,
            self.size,
            self.border_color,
            self.border_width
        );
    }
    
    pub fn set_scale(&mut self, scale: f32) {
        self.world_scale = scale.clamp(0.02, 0.5);  // Limit zoom range
    }
    
    pub fn zoom_in(&mut self) {
        self.set_scale(self.world_scale * 0.9);
    }
    
    pub fn zoom_out(&mut self) {
        self.set_scale(self.world_scale * 1.1);
    }
}