use crate::engine::core::Color;
use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponType {
    Pistol,
    Shotgun,
    RocketLauncher,
    LaserRifle,
    PlasmaGun,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub projectile_speed: f32,
    pub projectile_force: f32,  // How much force/velocity to apply
    pub fire_rate: f32,         // Shots per second
    pub spread: f32,             // Accuracy spread in radians
    pub projectile_count: u32,   // Number of projectiles per shot (shotgun)
    pub projectile_size: Vec2,
    pub projectile_color: Color,
    pub projectile_lifetime: f32,
    pub gravity_scale: f32,      // How much gravity affects projectiles
    pub explosion_radius: f32,   // For explosive weapons
    pub pierce_count: u32,       // How many enemies can be pierced
    pub ammo: Option<u32>,       // None for infinite ammo
    pub max_ammo: Option<u32>,
}

impl Weapon {
    pub fn pistol() -> Self {
        Self {
            weapon_type: WeaponType::Pistol,
            damage: 20.0,
            projectile_speed: 1200.0,
            projectile_force: 1200.0,
            fire_rate: 4.0,
            spread: 0.05,
            projectile_count: 1,
            projectile_size: Vec2::new(6.0, 6.0),
            projectile_color: Color::new(255, 255, 100, 255), // Yellow
            projectile_lifetime: 2.0,
            gravity_scale: 0.3,
            explosion_radius: 0.0,
            pierce_count: 0,
            ammo: None,
            max_ammo: None,
        }
    }

    pub fn shotgun() -> Self {
        Self {
            weapon_type: WeaponType::Shotgun,
            damage: 15.0,
            projectile_speed: 1500.0,
            projectile_force: 1800.0,
            fire_rate: 1.5,
            spread: 0.3,
            projectile_count: 8,
            projectile_size: Vec2::new(4.0, 4.0),
            projectile_color: Color::new(255, 200, 50, 255), // Orange
            projectile_lifetime: 0.5,
            gravity_scale: 0.1,
            explosion_radius: 0.0,
            pierce_count: 0,
            ammo: Some(50),
            max_ammo: Some(50),
        }
    }

    pub fn rocket_launcher() -> Self {
        Self {
            weapon_type: WeaponType::RocketLauncher,
            damage: 100.0,
            projectile_speed: 800.0,
            projectile_force: 1000.0,
            fire_rate: 0.8,
            spread: 0.02,
            projectile_count: 1,
            projectile_size: Vec2::new(12.0, 8.0),
            projectile_color: Color::new(255, 100, 50, 255), // Red-orange
            projectile_lifetime: 5.0,
            gravity_scale: 0.2,
            explosion_radius: 100.0,
            pierce_count: 0,
            ammo: Some(20),
            max_ammo: Some(20),
        }
    }

    pub fn laser_rifle() -> Self {
        Self {
            weapon_type: WeaponType::LaserRifle,
            damage: 30.0,
            projectile_speed: 3000.0,
            projectile_force: 3000.0,
            fire_rate: 10.0,
            spread: 0.01,
            projectile_count: 1,
            projectile_size: Vec2::new(20.0, 3.0),
            projectile_color: Color::new(255, 50, 255, 255), // Magenta
            projectile_lifetime: 1.0,
            gravity_scale: 0.0, // No gravity for lasers
            explosion_radius: 0.0,
            pierce_count: 3, // Can pierce through 3 enemies
            ammo: Some(200),
            max_ammo: Some(200),
        }
    }

    pub fn plasma_gun() -> Self {
        Self {
            weapon_type: WeaponType::PlasmaGun,
            damage: 50.0,
            projectile_speed: 1000.0,
            projectile_force: 1200.0,
            fire_rate: 2.0,
            spread: 0.08,
            projectile_count: 1,
            projectile_size: Vec2::new(16.0, 16.0),
            projectile_color: Color::new(100, 255, 255, 255), // Cyan
            projectile_lifetime: 3.0,
            gravity_scale: 0.15,
            explosion_radius: 50.0,
            pierce_count: 1,
            ammo: Some(100),
            max_ammo: Some(100),
        }
    }

    pub fn get_cooldown(&self) -> f32 {
        1.0 / self.fire_rate
    }

    pub fn consume_ammo(&mut self) -> bool {
        if let Some(ammo) = &mut self.ammo {
            if *ammo > 0 {
                *ammo -= 1;
                true
            } else {
                false
            }
        } else {
            true // Infinite ammo
        }
    }

    pub fn reload(&mut self) {
        if let (Some(max), ammo) = (self.max_ammo, &mut self.ammo) {
            *ammo = Some(max);
        }
    }
}

#[derive(Debug, Clone)]
pub struct WeaponInventory {
    pub weapons: Vec<Weapon>,
    pub current_weapon_index: usize,
}

impl WeaponInventory {
    pub fn new() -> Self {
        Self {
            weapons: vec![
                Weapon::pistol(),
                Weapon::shotgun(),
                Weapon::rocket_launcher(),
                Weapon::laser_rifle(),
                Weapon::plasma_gun(),
            ],
            current_weapon_index: 0,
        }
    }

    pub fn current_weapon(&self) -> &Weapon {
        &self.weapons[self.current_weapon_index]
    }

    pub fn current_weapon_mut(&mut self) -> &mut Weapon {
        &mut self.weapons[self.current_weapon_index]
    }

    pub fn switch_weapon(&mut self, index: usize) {
        if index < self.weapons.len() {
            self.current_weapon_index = index;
        }
    }

    pub fn next_weapon(&mut self) {
        self.current_weapon_index = (self.current_weapon_index + 1) % self.weapons.len();
    }

    pub fn previous_weapon(&mut self) {
        if self.current_weapon_index == 0 {
            self.current_weapon_index = self.weapons.len() - 1;
        } else {
            self.current_weapon_index -= 1;
        }
    }
}