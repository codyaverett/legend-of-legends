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
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            size: Vec2::new(32.0, 48.0),
            health: 50.0,
            max_health: 50.0,
            attack_range: 500.0,
            shoot_cooldown: 1.5,
            shoot_timer: 0.0,
            projectile_speed: 300.0,
            damage: 10.0,
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

    pub fn update_target(&mut self, player_pos: Vec2, enemy_pos: Vec2, attack_range: f32) {
        let distance = (player_pos - enemy_pos).length();
        
        if distance <= attack_range {
            self.target_position = Some(player_pos);
            self.state = EnemyState::Targeting;
            
            let direction = (player_pos - enemy_pos).normalize_or_zero();
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
    
    let mut player_pos = None;
    for (_entity, transform) in world.query::<&Transform>()
        .with::<&crate::systems::player::Player>()
        .iter() 
    {
        player_pos = Some(transform.position);
        break;
    }

    if let Some(player_position) = player_pos {
        for (_entity, (enemy, transform, controller)) in world.query_mut::<(
            &mut Enemy,
            &Transform,
            &mut EnemyController,
        )>() {
            enemy.update_timer(delta_time);
            
            controller.update_target(player_position, transform.position, enemy.attack_range);
            
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