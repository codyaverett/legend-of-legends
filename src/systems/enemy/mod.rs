use crate::engine::core::{Rect, Transform};
use crate::engine::physics::RigidBody;
use crate::game::Level;
use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub size: Vec2,
    pub health: f32,
    pub max_health: f32,
    pub attack_range: f32,
    pub shoot_cooldown: f32,
    pub shoot_timer: f32,
    pub projectile_speed: f32,
    pub damage: f32,
    pub health_bar_timer: f32,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            size: Vec2::new(32.0, 48.0),
            health: 50.0,
            max_health: 50.0,
            attack_range: 600.0,  // Increased range
            shoot_cooldown: 2.0,  // Slightly slower fire rate for balance
            shoot_timer: 0.0,
            projectile_speed: 400.0,  // Increased projectile speed
            damage: 10.0,
            health_bar_timer: 0.0,
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.shoot_timer <= 0.0
    }

    pub fn reset_shoot_timer(&mut self) {
        self.shoot_timer = self.shoot_cooldown;
    }

    pub fn update_timer(&mut self, delta_time: f32) {
        if self.shoot_timer > 0.0 {
            self.shoot_timer -= delta_time;
        }
    }
    
    pub fn show_health_bar(&mut self) {
        self.health_bar_timer = 3.0; // Show health bar for 3 seconds
    }
    
    pub fn update_health_bar_timer(&mut self, delta_time: f32) {
        if self.health_bar_timer > 0.0 {
            self.health_bar_timer -= delta_time;
        }
    }
    
    pub fn should_show_health_bar(&self) -> bool {
        self.health_bar_timer > 0.0
    }
}

#[derive(Debug, Clone)]
pub enum EnemyState {
    Idle,
    Targeting,
    Shooting,
}

#[derive(Debug, Clone)]
pub struct EnemyController {
    pub state: EnemyState,
    pub target_position: Option<Vec2>,
    pub facing_direction: Vec2,
}

impl EnemyController {
    pub fn new() -> Self {
        Self {
            state: EnemyState::Idle,
            target_position: None,
            facing_direction: Vec2::new(-1.0, 0.0),
        }
    }

    pub fn update_target(&mut self, player_pos: Vec2, enemy_pos: Vec2, attack_range: f32, player_velocity: Option<Vec2>, projectile_speed: f32) {
        let distance = (player_pos - enemy_pos).length();
        
        if distance <= attack_range {
            // Calculate lead position for better aiming
            let mut aim_position = player_pos;
            
            // If player is moving, predict where they'll be
            if let Some(vel) = player_velocity {
                if vel.length() > 10.0 {  // Only lead if player is moving significantly
                    let time_to_target = distance / projectile_speed;
                    // Add predicted position based on player velocity
                    aim_position += vel * time_to_target * 0.5; // 0.5 factor for partial prediction
                }
            }
            
            self.target_position = Some(aim_position);
            self.state = EnemyState::Targeting;
            
            let direction = (aim_position - enemy_pos).normalize_or_zero();
            if direction != Vec2::ZERO {
                self.facing_direction = direction;
            }
        } else {
            self.target_position = None;
            self.state = EnemyState::Idle;
        }
    }

    pub fn get_shoot_direction(&self) -> Vec2 {
        self.facing_direction
    }
}

pub fn enemy_ai_system(
    world: &mut hecs::World,
    delta_time: f32,
) -> Vec<(Vec2, Vec2, f32, f32)> {
    let mut projectiles_to_spawn = Vec::new();
    
    let mut player_info = None;
    for (_entity, (transform, body)) in world.query::<(&Transform, &RigidBody)>()
        .with::<&crate::systems::player::Player>()
        .iter() 
    {
        player_info = Some((transform.position, body.velocity));
        break;
    }

    if let Some((player_position, player_velocity)) = player_info {
        for (_entity, (enemy, transform, controller)) in world.query_mut::<(
            &mut Enemy,
            &Transform,
            &mut EnemyController,
        )>() {
            enemy.update_timer(delta_time);
            enemy.update_health_bar_timer(delta_time);
            
            controller.update_target(
                player_position, 
                transform.position, 
                enemy.attack_range,
                Some(player_velocity),
                enemy.projectile_speed
            );
            
            if matches!(controller.state, EnemyState::Targeting) && enemy.can_shoot() {
                let shoot_dir = controller.get_shoot_direction();
                let spawn_offset = shoot_dir * (enemy.size.x / 2.0 + 10.0);
                let spawn_pos = transform.position + spawn_offset;
                
                projectiles_to_spawn.push((
                    spawn_pos,
                    shoot_dir,
                    enemy.projectile_speed,
                    enemy.damage,
                ));
                
                enemy.reset_shoot_timer();
                controller.state = EnemyState::Shooting;
            } else if !matches!(controller.state, EnemyState::Targeting) {
                controller.state = EnemyState::Idle;
            }
        }
    }
    
    projectiles_to_spawn
}

pub fn enemy_physics_system(
    world: &mut hecs::World,
    level: &Level,
    delta_time: f32,
) {
    for (_entity, (enemy, transform, body)) in world.query_mut::<(
        &Enemy,
        &mut Transform,
        &mut RigidBody,
    )>()
    .without::<&crate::systems::player::Player>()
    {
        // Apply gravity
        body.apply_force(Vec2::new(0.0, 800.0));
        body.update(delta_time);

        let new_x = transform.position.x + body.velocity.x * delta_time;
        let new_y = transform.position.y + body.velocity.y * delta_time;

        // Check horizontal collision
        let enemy_rect_x = Rect::new(
            new_x - enemy.size.x / 2.0,
            transform.position.y - enemy.size.y / 2.0,
            enemy.size.x,
            enemy.size.y,
        );

        if !level.check_collision(enemy_rect_x) {
            transform.position.x = new_x;
        } else {
            body.velocity.x = 0.0;
        }

        // Check vertical collision
        let enemy_rect_y = Rect::new(
            transform.position.x - enemy.size.x / 2.0,
            new_y - enemy.size.y / 2.0,
            enemy.size.x,
            enemy.size.y,
        );

        if !level.check_collision(enemy_rect_y) {
            transform.position.y = new_y;
        } else {
            body.velocity.y = 0.0;
        }
    }
}