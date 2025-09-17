use glam::Vec2;
use crate::engine::core::Color;

#[derive(Clone, Debug)]
pub struct Mech {
    pub size: Vec2,
    pub health: f32,
    pub max_health: f32,
    pub energy: f32,
    pub max_energy: f32,
    pub boost_speed: f32,
    pub normal_speed: f32,
    pub jump_power: f32,
    pub is_occupied: bool,
    pub pilot_entity: Option<hecs::Entity>,
}

impl Mech {
    pub fn new() -> Self {
        Self {
            size: Vec2::new(80.0, 120.0), // Much larger than player
            health: 500.0,
            max_health: 500.0,
            energy: 200.0,
            max_energy: 200.0,
            boost_speed: 800.0,
            normal_speed: 400.0,
            jump_power: 1200.0,
            is_occupied: false,
            pilot_entity: None,
        }
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }

    pub fn use_energy(&mut self, amount: f32) -> bool {
        if self.energy >= amount {
            self.energy -= amount;
            true
        } else {
            false
        }
    }

    pub fn regenerate_energy(&mut self, amount: f32, delta_time: f32) {
        let regen = amount * delta_time;
        self.energy = (self.energy + regen).min(self.max_energy);
    }

    pub fn is_destroyed(&self) -> bool {
        self.health <= 0.0
    }

    pub fn can_boost(&self) -> bool {
        self.energy >= 10.0
    }

    pub fn get_color(&self) -> Color {
        if self.is_occupied {
            // Active mech - bright blue/silver
            Color::new(100, 150, 255, 255)
        } else {
            // Inactive mech - darker gray
            Color::new(80, 80, 100, 255)
        }
    }
}

#[derive(Clone, Debug)]
pub struct MechController {
    pub is_boosting: bool,
    pub boost_cooldown: f32,
    pub stomp_cooldown: f32,
    pub weapon_cooldown: f32,
}

impl MechController {
    pub fn new() -> Self {
        Self {
            is_boosting: false,
            boost_cooldown: 0.0,
            stomp_cooldown: 0.0,
            weapon_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.boost_cooldown = (self.boost_cooldown - delta_time).max(0.0);
        self.stomp_cooldown = (self.stomp_cooldown - delta_time).max(0.0);
        self.weapon_cooldown = (self.weapon_cooldown - delta_time).max(0.0);
    }

    pub fn can_stomp(&self) -> bool {
        self.stomp_cooldown <= 0.0
    }

    pub fn do_stomp(&mut self) {
        self.stomp_cooldown = 2.0; // 2 second cooldown
    }
}

pub mod weapons;
pub mod movement;
pub mod transformation;

pub use weapons::*;
pub use movement::*;
pub use transformation::*;