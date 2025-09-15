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

#[derive(Debug, Clone)]
pub struct WeaponDisplay {
    pub position: Vec2,
    pub size: Vec2,
    pub background: Panel,
    pub weapon_name_text: Text,
    pub ammo_text: Text,
    pub weapon_slots: Vec<Panel>,
    pub current_weapon_index: usize,
    pub visible: bool,
    pub flash_timer: f32,
    pub out_of_ammo: bool,
}

impl WeaponDisplay {
    pub fn new(position: Vec2) -> Self {
        let size = Vec2::new(250.0, 80.0);
        
        let mut background = Panel::new(position, size);
        background.background_color = Color::new(20, 20, 30, 220);
        background.border_color = Color::new(100, 100, 150, 255);
        background.border_width = 2.0;
        
        let weapon_name_text = Text::new(
            position + Vec2::new(10.0, 10.0),
            "Pistol".to_string(),
        );
        
        let mut ammo_text = Text::new(
            position + Vec2::new(10.0, 35.0),
            "∞".to_string(),
        );
        ammo_text.size = 20;
        ammo_text.color = Color::new(200, 200, 200, 255);
        
        // Create weapon slot indicators
        let mut weapon_slots = Vec::new();
        for i in 0..5 {
            let slot_pos = position + Vec2::new(10.0 + i as f32 * 45.0, 55.0);
            let mut slot = Panel::new(slot_pos, Vec2::new(40.0, 20.0));
            slot.background_color = Color::new(40, 40, 50, 200);
            slot.border_color = Color::new(80, 80, 100, 255);
            slot.border_width = 1.0;
            weapon_slots.push(slot);
        }
        
        Self {
            position,
            size,
            background,
            weapon_name_text,
            ammo_text,
            weapon_slots,
            current_weapon_index: 0,
            visible: true,
            flash_timer: 0.0,
            out_of_ammo: false,
        }
    }
    
    pub fn update_weapon(&mut self, weapon: &crate::systems::weapons::Weapon, index: usize) {
        use crate::systems::weapons::WeaponType;
        
        // Update weapon name
        self.weapon_name_text.content = match weapon.weapon_type {
            WeaponType::Pistol => "Pistol",
            WeaponType::Shotgun => "Shotgun",
            WeaponType::RocketLauncher => "Rocket Launcher",
            WeaponType::LaserRifle => "Laser Rifle",
            WeaponType::PlasmaGun => "Plasma Gun",
        }.to_string();
        
        // Update weapon name color to match projectile color
        self.weapon_name_text.color = weapon.projectile_color;
        
        // Update ammo display
        if let Some(ammo) = weapon.ammo {
            if let Some(max_ammo) = weapon.max_ammo {
                self.ammo_text.content = format!("{}/{}", ammo, max_ammo);
                self.out_of_ammo = ammo == 0;
                
                // Color code ammo based on amount
                if ammo == 0 {
                    self.ammo_text.color = Color::new(255, 50, 50, 255); // Red
                } else if ammo <= max_ammo / 4 {
                    self.ammo_text.color = Color::new(255, 200, 50, 255); // Yellow
                } else {
                    self.ammo_text.color = Color::new(200, 200, 200, 255); // White
                }
            }
        } else {
            self.ammo_text.content = "∞".to_string();
            self.ammo_text.color = Color::new(100, 255, 100, 255); // Green for infinite
            self.out_of_ammo = false;
        }
        
        // Update current weapon index
        if self.current_weapon_index != index {
            self.current_weapon_index = index;
            self.flash_timer = 0.3; // Flash for 0.3 seconds on weapon switch
        }
        
        // Update weapon slot highlights
        for (i, slot) in self.weapon_slots.iter_mut().enumerate() {
            if i == index {
                slot.background_color = Color::new(60, 60, 100, 255);
                slot.border_color = weapon.projectile_color;
                slot.border_width = 2.0;
            } else {
                slot.background_color = Color::new(40, 40, 50, 200);
                slot.border_color = Color::new(80, 80, 100, 255);
                slot.border_width = 1.0;
            }
        }
    }
}

impl UIElement for WeaponDisplay {
    fn update(&mut self, delta_time: f32) {
        if self.flash_timer > 0.0 {
            self.flash_timer -= delta_time;
            
            // Flash effect
            let flash_intensity = (self.flash_timer * 10.0).sin().abs();
            self.background.border_color = Color::new(
                (100.0 + 155.0 * flash_intensity) as u8,
                (100.0 + 155.0 * flash_intensity) as u8,
                150,
                255,
            );
        } else {
            self.background.border_color = Color::new(100, 100, 150, 255);
        }
        
        // Flash red if out of ammo
        if self.out_of_ammo {
            let flash = ((delta_time * 1000.0) as i32 % 500) > 250;
            if flash {
                self.ammo_text.color = Color::new(255, 50, 50, 255);
            } else {
                self.ammo_text.color = Color::new(150, 30, 30, 255);
            }
        }
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.background.position = position;
        self.weapon_name_text.position = position + Vec2::new(10.0, 10.0);
        self.ammo_text.position = position + Vec2::new(10.0, 35.0);
        
        for i in 0..5 {
            self.weapon_slots[i].position = position + Vec2::new(10.0 + i as f32 * 45.0, 55.0);
        }
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}