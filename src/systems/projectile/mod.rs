use crate::engine::core::{Color, Rect, Transform};
use crate::engine::physics::RigidBody;
use crate::engine::rendering::Sprite;
use crate::game::Level;
use crate::systems::enemy::Enemy;
use crate::systems::particles::{ParticleSystem, spawn_particle};
use crate::systems::player::Player;
use crate::systems::weapons::WeaponType;
use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Projectile {
    pub damage: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: Vec2,
    pub owner_type: ProjectileOwner,
    pub weapon_type: Option<WeaponType>,
    pub gravity_scale: f32,
    pub explosion_radius: f32,
    pub pierce_count: u32,
    pub has_trail: bool,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectileOwner {
    Enemy,
    Player,
}

impl Projectile {
    pub fn new(damage: f32, owner: ProjectileOwner) -> Self {
        Self {
            damage,
            lifetime: 0.0,
            max_lifetime: 5.0,
            size: Vec2::new(8.0, 8.0),
            owner_type: owner,
            weapon_type: None,
            gravity_scale: 1.0,
            explosion_radius: 0.0,
            pierce_count: 0,
            has_trail: false,
            color: Color::new(255, 200, 0, 255),
        }
    }
    
    pub fn from_weapon(weapon: &crate::systems::weapons::Weapon, owner: ProjectileOwner) -> Self {
        Self {
            damage: weapon.damage,
            lifetime: 0.0,
            max_lifetime: weapon.projectile_lifetime,
            size: weapon.projectile_size,
            owner_type: owner,
            weapon_type: Some(weapon.weapon_type),
            gravity_scale: weapon.gravity_scale,
            explosion_radius: weapon.explosion_radius,
            pierce_count: weapon.pierce_count,
            has_trail: matches!(weapon.weapon_type, 
                WeaponType::RocketLauncher | 
                WeaponType::LaserRifle | 
                WeaponType::PlasmaGun
            ),
            color: weapon.projectile_color,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime >= self.max_lifetime
    }
}

pub fn projectile_system(
    world: &mut hecs::World,
    level: &Level,
    delta_time: f32,
) -> (Vec<hecs::Entity>, Vec<crate::systems::particles::Particle>) {
    let mut expired_projectiles = Vec::new();
    let mut player_hits = Vec::new();
    let mut enemy_hits = Vec::new();
    let mut particles_to_spawn = Vec::new();
    
    // Collect player info
    let mut player_info = None;
    for (_player_entity, (player, player_transform)) in world
        .query::<(&Player, &Transform)>()
        .iter()
    {
        player_info = Some((player_transform.position, player.size));
        break;
    }
    
    // Collect enemy info
    let mut enemy_info = Vec::new();
    for (enemy_entity, (enemy, enemy_transform)) in world
        .query::<(&Enemy, &Transform)>()
        .iter()
    {
        enemy_info.push((enemy_entity, enemy_transform.position, enemy.size));
    }

    // Process projectiles
    for (proj_entity, (projectile, proj_transform, proj_body)) in world
        .query_mut::<(&mut Projectile, &mut Transform, &mut RigidBody)>()
    {
        projectile.lifetime += delta_time;
        
        if projectile.is_expired() {
            expired_projectiles.push(proj_entity);
            continue;
        }

        // Apply gravity to projectiles based on their gravity scale
        let gravity_force = 500.0 * projectile.gravity_scale;
        proj_body.apply_force(Vec2::new(0.0, gravity_force));
        proj_body.update(delta_time);
        
        // Create trail particles for certain projectiles
        if projectile.has_trail && projectile.lifetime > 0.05 {
            let trail_particles = ParticleSystem::create_trail_particles(
                proj_transform.position,
                proj_body.velocity.normalize_or_zero()
            );
            particles_to_spawn.extend(trail_particles);
        }
        
        // Update projectile position
        let new_pos = proj_transform.position + proj_body.velocity * delta_time;
        
        // Check collision with level
        let proj_rect = Rect::new(
            new_pos.x - projectile.size.x / 2.0,
            new_pos.y - projectile.size.y / 2.0,
            projectile.size.x,
            projectile.size.y,
        );
        
        if level.check_collision(proj_rect) {
            // Create impact particles
            let impact_particles = ParticleSystem::create_impact_particles(
                proj_transform.position,
                proj_body.velocity.normalize_or_zero(),
                5
            );
            particles_to_spawn.extend(impact_particles);
            
            // Create explosion if applicable
            if projectile.explosion_radius > 0.0 {
                let explosion_particles = ParticleSystem::create_explosion_particles(
                    proj_transform.position,
                    20,
                    projectile.explosion_radius / 50.0
                );
                particles_to_spawn.extend(explosion_particles);
                
                // TODO: Apply explosion damage in radius
            }
            
            expired_projectiles.push(proj_entity);
            continue;
        }
        
        proj_transform.position = new_pos;

        // Check collisions based on owner
        if projectile.owner_type == ProjectileOwner::Enemy {
            // Check collision with player
            if let Some((player_pos, player_size)) = player_info {
                let player_rect = Rect::new(
                    player_pos.x - player_size.x / 2.0,
                    player_pos.y - player_size.y / 2.0,
                    player_size.x,
                    player_size.y,
                );

                if player_rect.intersects(&proj_rect) {
                    player_hits.push((proj_entity, projectile.damage));
                    
                    // Create impact effects
                    let impact_particles = ParticleSystem::create_impact_particles(
                        proj_transform.position,
                        proj_body.velocity.normalize_or_zero(),
                        8
                    );
                    particles_to_spawn.extend(impact_particles);
                }
            }
        } else if projectile.owner_type == ProjectileOwner::Player {
            // Check collision with enemies
            for (enemy_entity, enemy_pos, enemy_size) in &enemy_info {
                let enemy_rect = Rect::new(
                    enemy_pos.x - enemy_size.x / 2.0,
                    enemy_pos.y - enemy_size.y / 2.0,
                    enemy_size.x,
                    enemy_size.y,
                );

                if enemy_rect.intersects(&proj_rect) {
                    enemy_hits.push((proj_entity, *enemy_entity, projectile.damage));
                    
                    // Create impact effects
                    let impact_particles = ParticleSystem::create_impact_particles(
                        proj_transform.position,
                        proj_body.velocity.normalize_or_zero(),
                        8
                    );
                    particles_to_spawn.extend(impact_particles);
                    
                    // Handle piercing
                    if projectile.pierce_count > 0 {
                        projectile.pierce_count -= 1;
                    } else {
                        break; // Projectile can only hit one enemy if no pierce
                    }
                }
            }
        }
    }

    // Apply damage to player
    for (proj_entity, damage) in player_hits {
        for (_player_entity, player) in world.query_mut::<&mut Player>() {
            player.health = (player.health - damage).max(0.0);
        }
        expired_projectiles.push(proj_entity);
    }
    
    // Apply damage to enemies
    for (proj_entity, enemy_entity, damage) in enemy_hits {
        if let Ok(mut enemy) = world.get::<&mut Enemy>(enemy_entity) {
            enemy.health = (enemy.health - damage).max(0.0);
            enemy.show_health_bar(); // Show health bar when hit
        }
        expired_projectiles.push(proj_entity);
    }

    // Remove dead enemies
    let mut dead_enemies = Vec::new();
    for (enemy_entity, enemy) in world.query::<&Enemy>().iter() {
        if enemy.health <= 0.0 {
            dead_enemies.push(enemy_entity);
        }
    }
    for entity in dead_enemies {
        let _ = world.despawn(entity);
    }

    (expired_projectiles, particles_to_spawn)
}

// This function is no longer needed as projectile positions are updated in projectile_system