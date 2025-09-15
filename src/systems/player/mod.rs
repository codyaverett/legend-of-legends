use crate::engine::core::{Rect, Transform};
use crate::engine::physics::RigidBody;
use crate::engine::rendering::Camera;
use crate::game::Level;
use crate::systems::weapons::{Weapon, WeaponInventory};
use crate::systems::particles::{ParticleSystem, spawn_particle};
use glam::Vec2;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use rand::Rng;

pub struct Player {
    pub size: Vec2,
    pub health: f32,
    pub max_health: f32,
    pub energy: f32,
    pub max_energy: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            size: Vec2::new(24.0, 40.0),
            health: 100.0,
            max_health: 100.0,
            energy: 100.0,
            max_energy: 100.0,
        }
    }
}

pub struct PlayerController {
    pub speed: f32,
    pub jump_force: f32,
    pub double_jump_force: f32,
    pub is_grounded: bool,
    pub jump_count: u32,
    pub max_jumps: u32,
    pub is_spinning: bool,
    pub spin_rotation: f32,
    pub spin_speed: f32,
    pub shoot_timer: f32,
    pub weapon_inventory: WeaponInventory,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            speed: 400.0,  // Doubled speed for faster movement
            jump_force: 600.0,  // Slightly increased jump force
            double_jump_force: 500.0,  // Increased double jump force
            is_grounded: false,
            jump_count: 0,
            max_jumps: 2,
            is_spinning: false,
            spin_rotation: 0.0,
            spin_speed: 720.0, // 720 degrees per second (2 full rotations)
            shoot_timer: 0.0,
            weapon_inventory: WeaponInventory::new()
        }
    }
    
    pub fn can_shoot(&self) -> bool {
        self.shoot_timer <= 0.0
    }
    
    pub fn reset_shoot_timer(&mut self) {
        self.shoot_timer = self.weapon_inventory.current_weapon().get_cooldown();
    }
    
    pub fn update_timer(&mut self, delta_time: f32) {
        if self.shoot_timer > 0.0 {
            self.shoot_timer -= delta_time;
        }
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self::new()
    }
}

pub fn player_movement_system(
    world: &mut hecs::World,
    input: &crate::engine::platform::InputState,
    level: &Level,
    delta_time: f32,
) {
    for (_entity, (player, transform, body, controller)) in world.query_mut::<(
        &mut Player,
        &mut Transform,
        &mut RigidBody,
        &mut PlayerController,
    )>() {
        let mut movement = Vec2::ZERO;

        if input.is_key_down(Keycode::A) || input.is_key_down(Keycode::Left) {
            movement.x -= 1.0;
        }
        if input.is_key_down(Keycode::D) || input.is_key_down(Keycode::Right) {
            movement.x += 1.0;
        }

        // Handle jumping (first jump from ground, second jump in air)
        if input.is_key_pressed(Keycode::Space)
            || input.is_key_pressed(Keycode::W)
            || input.is_key_pressed(Keycode::Up)
        {
            if controller.is_grounded {
                // First jump from ground
                body.velocity.y = -controller.jump_force;
                controller.is_grounded = false;
                controller.jump_count = 1;
            } else if controller.jump_count < controller.max_jumps && player.energy >= 20.0 {
                // Double jump in air with booster effect (costs energy)
                body.velocity.y = -controller.double_jump_force;
                controller.jump_count += 1;
                controller.is_spinning = true;
                controller.spin_rotation = 0.0;
                player.energy = (player.energy - 20.0).max(0.0);
            }
        }

        movement = movement.normalize_or_zero();
        body.velocity.x = movement.x * controller.speed;

        // Handle spin flip animation during double jump
        if controller.is_spinning {
            controller.spin_rotation += controller.spin_speed * delta_time;
            transform.rotation = controller.spin_rotation.to_radians();
            
            // Complete spin after one full rotation
            if controller.spin_rotation >= 360.0 {
                controller.is_spinning = false;
                controller.spin_rotation = 0.0;
                transform.rotation = 0.0;
            }
        }

        body.apply_force(Vec2::new(0.0, 800.0));

        body.update(delta_time);

        let new_x = transform.position.x + body.velocity.x * delta_time;
        let new_y = transform.position.y + body.velocity.y * delta_time;

        let player_rect_x = Rect::new(
            new_x - player.size.x / 2.0,
            transform.position.y - player.size.y / 2.0,
            player.size.x,
            player.size.y,
        );

        if !level.check_collision(player_rect_x) {
            transform.position.x = new_x;
        } else {
            body.velocity.x = 0.0;
        }

        let player_rect_y = Rect::new(
            transform.position.x - player.size.x / 2.0,
            new_y - player.size.y / 2.0,
            player.size.x,
            player.size.y,
        );

        if !level.check_collision(player_rect_y) {
            transform.position.y = new_y;
            controller.is_grounded = false;
        } else {
            if body.velocity.y > 0.0 {
                controller.is_grounded = true;
                controller.jump_count = 0;
                // Reset spin when landing
                if controller.is_spinning {
                    controller.is_spinning = false;
                    controller.spin_rotation = 0.0;
                    transform.rotation = 0.0;
                }
            }
            body.velocity.y = 0.0;
        }

        let ground_check = Rect::new(
            transform.position.x - player.size.x / 2.0,
            transform.position.y - player.size.y / 2.0 + 2.0,
            player.size.x,
            player.size.y,
        );

        if level.check_collision(ground_check) && body.velocity.y >= 0.0 {
            controller.is_grounded = true;
            controller.jump_count = 0;
            // Reset spin when landing
            if controller.is_spinning {
                controller.is_spinning = false;
                controller.spin_rotation = 0.0;
                transform.rotation = 0.0;
            }
            // Regenerate energy while grounded
            if player.energy < player.max_energy {
                player.energy = (player.energy + 30.0 * delta_time).min(player.max_energy);
            }
        }
    }
}

pub fn player_shooting_system(
    world: &mut hecs::World,
    input: &crate::engine::platform::InputState,
    camera: &Camera,
    delta_time: f32,
) -> Vec<ProjectileSpawnData> {
    let mut projectiles_to_spawn = Vec::new();
    let mut particles_to_spawn = Vec::new();
    
    for (_entity, (transform, controller)) in world.query_mut::<(&Transform, &mut PlayerController)>()
        .with::<&Player>()
    {
        controller.update_timer(delta_time);
        
        // Handle weapon switching
        if input.is_key_pressed(Keycode::Num1) {
            controller.weapon_inventory.switch_weapon(0);
        }
        if input.is_key_pressed(Keycode::Num2) {
            controller.weapon_inventory.switch_weapon(1);
        }
        if input.is_key_pressed(Keycode::Num3) {
            controller.weapon_inventory.switch_weapon(2);
        }
        if input.is_key_pressed(Keycode::Num4) {
            controller.weapon_inventory.switch_weapon(3);
        }
        if input.is_key_pressed(Keycode::Num5) {
            controller.weapon_inventory.switch_weapon(4);
        }
        if input.is_key_pressed(Keycode::Q) {
            controller.weapon_inventory.previous_weapon();
        }
        if input.is_key_pressed(Keycode::E) {
            controller.weapon_inventory.next_weapon();
        }
        if input.is_key_pressed(Keycode::R) {
            controller.weapon_inventory.current_weapon_mut().reload();
        }
        
        if input.is_mouse_button_down(MouseButton::Left) && controller.can_shoot() {
            let weapon = controller.weapon_inventory.current_weapon_mut();
            
            if weapon.consume_ammo() {
                // Get mouse position in world coordinates
                let mouse_screen = input.mouse_position();
                let mouse_world = camera.screen_to_world(mouse_screen);
                
                // Calculate shooting direction
                let base_direction = (mouse_world - transform.position).normalize_or_zero();
                
                if base_direction != Vec2::ZERO {
                    let mut rng = rand::thread_rng();
                    
                    // Spawn multiple projectiles for shotgun
                    for _ in 0..weapon.projectile_count {
                        // Apply spread
                        let spread_angle = rng.gen_range(-weapon.spread..weapon.spread);
                        let cos_spread = spread_angle.cos();
                        let sin_spread = spread_angle.sin();
                        
                        let direction = Vec2::new(
                            base_direction.x * cos_spread - base_direction.y * sin_spread,
                            base_direction.x * sin_spread + base_direction.y * cos_spread,
                        );
                        
                        // Spawn projectile slightly offset from player to avoid self-collision
                        let spawn_offset = direction * 25.0;
                        let spawn_pos = transform.position + spawn_offset;
                        
                        projectiles_to_spawn.push(ProjectileSpawnData {
                            position: spawn_pos,
                            direction,
                            weapon: weapon.clone(),
                        });
                    }
                    
                    // Create muzzle flash particles
                    let muzzle_pos = transform.position + base_direction * 20.0;
                    let muzzle_particles = ParticleSystem::create_muzzle_flash(
                        muzzle_pos,
                        base_direction,
                        weapon.weapon_type,
                    );
                    for particle in muzzle_particles {
                        particles_to_spawn.push(particle);
                    }
                    
                    controller.reset_shoot_timer();
                }
            }
        }
    }
    
    // Spawn particles
    for particle in particles_to_spawn {
        spawn_particle(world, particle);
    }
    
    projectiles_to_spawn
}

#[derive(Debug, Clone)]
pub struct ProjectileSpawnData {
    pub position: Vec2,
    pub direction: Vec2,
    pub weapon: Weapon,
}
