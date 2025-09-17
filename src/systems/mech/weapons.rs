use glam::Vec2;
use crate::engine::core::Color;
use crate::systems::weapons::{Weapon, WeaponType};
use crate::systems::projectile::{Projectile, ProjectileOwner};

#[derive(Clone, Debug)]
pub enum MechWeaponType {
    PlasmaCanon,
    MissileLauncher,
    Railgun,
    FlameThrower,
}

#[derive(Clone, Debug)]
pub struct MechWeapon {
    pub weapon_type: MechWeaponType,
    pub damage: f32,
    pub fire_rate: f32,
    pub projectile_speed: f32,
    pub projectile_size: Vec2,
    pub energy_cost: f32,
    pub projectile_color: Color,
    pub splash_radius: Option<f32>,
}

impl MechWeapon {
    pub fn plasma_canon() -> Self {
        Self {
            weapon_type: MechWeaponType::PlasmaCanon,
            damage: 50.0,
            fire_rate: 2.0, // 2 shots per second
            projectile_speed: 800.0,
            projectile_size: Vec2::new(20.0, 20.0),
            energy_cost: 10.0,
            projectile_color: Color::new(100, 200, 255, 255), // Blue plasma
            splash_radius: Some(50.0),
        }
    }

    pub fn missile_launcher() -> Self {
        Self {
            weapon_type: MechWeaponType::MissileLauncher,
            damage: 100.0,
            fire_rate: 0.5, // 1 shot per 2 seconds
            projectile_speed: 600.0,
            projectile_size: Vec2::new(16.0, 8.0),
            energy_cost: 25.0,
            projectile_color: Color::new(255, 150, 50, 255), // Orange missile
            splash_radius: Some(100.0),
        }
    }

    pub fn railgun() -> Self {
        Self {
            weapon_type: MechWeaponType::Railgun,
            damage: 150.0,
            fire_rate: 0.33, // 1 shot per 3 seconds
            projectile_speed: 2000.0,
            projectile_size: Vec2::new(30.0, 4.0),
            energy_cost: 40.0,
            projectile_color: Color::new(255, 255, 100, 255), // Yellow beam
            splash_radius: None, // Penetrating shot
        }
    }

    pub fn flamethrower() -> Self {
        Self {
            weapon_type: MechWeaponType::FlameThrower,
            damage: 10.0, // Low damage but high fire rate
            fire_rate: 10.0, // 10 shots per second
            projectile_speed: 400.0,
            projectile_size: Vec2::new(15.0, 15.0),
            energy_cost: 2.0,
            projectile_color: Color::new(255, 100, 50, 255), // Red/orange flame
            splash_radius: Some(30.0),
        }
    }

    pub fn cooldown_time(&self) -> f32 {
        1.0 / self.fire_rate
    }

    pub fn to_projectile(&self) -> Projectile {
        let mut projectile = Projectile::new(self.damage, ProjectileOwner::Player);
        projectile.size = self.projectile_size;
        projectile.color = self.projectile_color;
        projectile.max_lifetime = 3.0; // Mech projectiles last longer
        if let Some(radius) = self.splash_radius {
            projectile.explosion_radius = radius;
        }
        projectile
    }
}

pub struct MechWeaponInventory {
    pub primary: MechWeapon,
    pub secondary: MechWeapon,
    pub primary_cooldown: f32,
    pub secondary_cooldown: f32,
}

impl MechWeaponInventory {
    pub fn new() -> Self {
        Self {
            primary: MechWeapon::plasma_canon(),
            secondary: MechWeapon::missile_launcher(),
            primary_cooldown: 0.0,
            secondary_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.primary_cooldown = (self.primary_cooldown - delta_time).max(0.0);
        self.secondary_cooldown = (self.secondary_cooldown - delta_time).max(0.0);
    }

    pub fn can_fire_primary(&self) -> bool {
        self.primary_cooldown <= 0.0
    }

    pub fn can_fire_secondary(&self) -> bool {
        self.secondary_cooldown <= 0.0
    }

    pub fn fire_primary(&mut self) {
        self.primary_cooldown = self.primary.cooldown_time();
    }

    pub fn fire_secondary(&mut self) {
        self.secondary_cooldown = self.secondary.cooldown_time();
    }
}