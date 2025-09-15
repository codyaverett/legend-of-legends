use crate::engine::ui::*;
use crate::engine::core::Color;
use crate::engine::rendering::Renderer;
use crate::systems::player::Player;
use crate::systems::weapons::Weapon;
use crate::game::DayNightCycle;
use glam::Vec2;

pub struct UIManager {
    pub health_bar: ProgressBar,
    pub energy_bar: ProgressBar,
    pub weapon_display: WeaponDisplay,
    pub clock: ClockWidget,
    pub debug_overlay: DebugOverlay,
    screen_size: Vec2,
}

impl UIManager {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let screen_size = Vec2::new(screen_width, screen_height);
        
        let health_bar = ProgressBar::health_bar(
            Vec2::new(20.0, 20.0),
            Vec2::new(200.0, 20.0),
            100.0,
        );
        
        let energy_bar = ProgressBar::energy_bar(
            Vec2::new(20.0, 50.0),
            Vec2::new(200.0, 20.0),
            100.0,
        );
        
        let weapon_display = WeaponDisplay::new(Vec2::new(20.0, 90.0));
        
        let clock_pos = Anchor::TopCenter.calculate_position(
            screen_size,
            Vec2::new(120.0, 40.0),
            Vec2::new(0.0, 20.0),
        );
        let clock = ClockWidget::new(clock_pos);
        
        let debug_pos = Anchor::TopRight.calculate_position(
            screen_size,
            Vec2::new(250.0, 150.0),
            Vec2::new(-20.0, 20.0),
        );
        let debug_overlay = DebugOverlay::new(debug_pos);
        
        Self {
            health_bar,
            energy_bar,
            weapon_display,
            clock,
            debug_overlay,
            screen_size,
        }
    }
    
    pub fn update(
        &mut self,
        delta_time: f32,
        player: Option<&Player>,
        weapon: Option<(&Weapon, usize)>,
        day_night_cycle: &DayNightCycle,
        entity_count: usize,
        player_pos: Option<Vec2>,
        player_velocity: Option<Vec2>,
    ) {
        if let Some(player) = player {
            self.health_bar.set_value(player.health);
            self.energy_bar.set_value(player.energy);
        }
        
        self.health_bar.update(delta_time);
        self.energy_bar.update(delta_time);
        
        if let Some((weapon, index)) = weapon {
            self.weapon_display.update_weapon(weapon, index);
        }
        self.weapon_display.update(delta_time);
        
        self.clock.update_time(day_night_cycle);
        self.clock.update(delta_time);
        
        self.debug_overlay.update_stats(delta_time, entity_count, player_pos, player_velocity);
        self.debug_overlay.update(delta_time);
    }
    
    pub fn render(&self, renderer: &mut Renderer) {
        if self.health_bar.is_visible() {
            self.render_progress_bar(renderer, &self.health_bar);
        }
        
        if self.energy_bar.is_visible() {
            self.render_progress_bar(renderer, &self.energy_bar);
        }
        
        if self.weapon_display.is_visible() {
            self.render_weapon_display(renderer, &self.weapon_display);
        }
        
        if self.clock.is_visible() {
            self.render_clock(renderer, &self.clock);
        }
        
        if self.debug_overlay.is_visible() {
            self.render_debug_overlay(renderer, &self.debug_overlay);
        }
    }
    
    fn render_progress_bar(&self, renderer: &mut Renderer, bar: &ProgressBar) {
        renderer.draw_ui_rect(bar.position, bar.size, bar.background_color);
        
        let fill_width = bar.get_fill_width();
        if fill_width > 0.0 {
            renderer.draw_ui_rect(
                bar.position,
                Vec2::new(fill_width, bar.size.y),
                bar.fill_color,
            );
        }
        
        if bar.border_width > 0.0 {
            renderer.draw_ui_rect_outline(
                bar.position,
                bar.size,
                bar.border_color,
                bar.border_width,
            );
        }
    }
    
    fn render_clock(&self, renderer: &mut Renderer, clock: &ClockWidget) {
        if clock.background.is_visible() {
            renderer.draw_ui_rect(
                clock.background.position,
                clock.background.size,
                clock.background.background_color,
            );
            
            if clock.background.border_width > 0.0 {
                renderer.draw_ui_rect_outline(
                    clock.background.position,
                    clock.background.size,
                    clock.background.border_color,
                    clock.background.border_width,
                );
            }
        }
        
        if clock.time_text.is_visible() {
            renderer.draw_ui_text(
                clock.time_text.position,
                &clock.time_text.content,
                clock.time_text.color,
                clock.time_text.size,
            );
        }
    }
    
    fn render_debug_overlay(&self, renderer: &mut Renderer, overlay: &DebugOverlay) {
        if overlay.background.is_visible() {
            renderer.draw_ui_rect(
                overlay.background.position,
                overlay.background.size,
                overlay.background.background_color,
            );
            
            if overlay.background.border_width > 0.0 {
                renderer.draw_ui_rect_outline(
                    overlay.background.position,
                    overlay.background.size,
                    overlay.background.border_color,
                    overlay.background.border_width,
                );
            }
        }
        
        for text in &overlay.texts {
            if text.is_visible() {
                renderer.draw_ui_text(
                    text.position,
                    &text.content,
                    text.color,
                    text.size,
                );
            }
        }
    }
    
    fn render_weapon_display(&self, renderer: &mut Renderer, display: &WeaponDisplay) {
        // Render background panel
        renderer.draw_ui_rect(
            display.background.position,
            display.background.size,
            display.background.background_color,
        );
        
        if display.background.border_width > 0.0 {
            renderer.draw_ui_rect_outline(
                display.background.position,
                display.background.size,
                display.background.border_color,
                display.background.border_width,
            );
        }
        
        // Render weapon name
        renderer.draw_ui_text(
            display.weapon_name_text.position,
            &display.weapon_name_text.content,
            display.weapon_name_text.color,
            display.weapon_name_text.size,
        );
        
        // Render ammo text
        renderer.draw_ui_text(
            display.ammo_text.position,
            &display.ammo_text.content,
            display.ammo_text.color,
            display.ammo_text.size,
        );
        
        // Render weapon slot indicators
        for (i, slot) in display.weapon_slots.iter().enumerate() {
            renderer.draw_ui_rect(
                slot.position,
                slot.size,
                slot.background_color,
            );
            
            if slot.border_width > 0.0 {
                renderer.draw_ui_rect_outline(
                    slot.position,
                    slot.size,
                    slot.border_color,
                    slot.border_width,
                );
            }
            
            // Draw weapon number
            let number_pos = slot.position + Vec2::new(slot.size.x / 2.0 - 5.0, 2.0);
            renderer.draw_ui_text(
                number_pos,
                &(i + 1).to_string(),
                Color::new(200, 200, 200, 255),
                16,
            );
        }
    }
    
    pub fn toggle_debug(&mut self) {
        self.debug_overlay.toggle();
    }
}