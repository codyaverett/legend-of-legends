use crate::engine::core::{Rect, Transform};
use crate::engine::physics::RigidBody;
use crate::game::Level;
use glam::Vec2;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum EnemyType {
    Ranger,      // Current type - maintains distance and shoots
    Rusher,      // Future type - charges at player
    Sniper,      // Future type - long range, slow fire
    Tank,        // Future type - slow, high health
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub size: Vec2,
    pub health: f32,
    pub max_health: f32,
    pub attack_range: f32,
    pub optimal_distance: f32,  // Preferred distance to maintain from player
    pub movement_speed: f32,
    pub shoot_cooldown: f32,
    pub shoot_timer: f32,
    pub projectile_speed: f32,
    pub damage: f32,
    pub health_bar_timer: f32,
}

impl Enemy {
    pub fn new() -> Self {
        Self::ranger()  // Default to Ranger type
    }
    
    pub fn ranger() -> Self {
        Self {
            enemy_type: EnemyType::Ranger,
            size: Vec2::new(32.0, 48.0),
            health: 50.0,
            max_health: 50.0,
            attack_range: 600.0,
            optimal_distance: 300.0,  // Stay about 300 pixels away
            movement_speed: 150.0,    // Movement speed in pixels/second
            shoot_cooldown: 2.0,
            shoot_timer: 0.0,
            projectile_speed: 600.0,  // Increased for better accuracy
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
    Pursuing,      // Moving toward optimal distance
    Retreating,    // Moving away if too close
    Strafing,      // Moving sideways while shooting
    Shooting,
}

#[derive(Debug, Clone)]
pub struct EnemyController {
    pub state: EnemyState,
    pub target_position: Option<Vec2>,
    pub facing_direction: Vec2,
    pub movement_direction: Vec2,
    pub strafe_timer: f32,
    pub strafe_direction: f32,  // 1.0 or -1.0 for left/right strafing
}

impl EnemyController {
    pub fn new() -> Self {
        Self {
            state: EnemyState::Idle,
            target_position: None,
            facing_direction: Vec2::new(-1.0, 0.0),
            movement_direction: Vec2::ZERO,
            strafe_timer: 0.0,
            strafe_direction: 1.0,
        }
    }

    pub fn update_movement_and_targeting(
        &mut self, 
        player_pos: Vec2, 
        enemy_pos: Vec2, 
        optimal_distance: f32,
        attack_range: f32, 
        player_velocity: Option<Vec2>, 
        projectile_speed: f32,
        delta_time: f32,
    ) {
        let to_player = player_pos - enemy_pos;
        let distance = to_player.length();
        
        if distance > 0.1 {  // Avoid division by zero
            let direction_to_player = to_player / distance;
            
            // Always face the player
            self.facing_direction = direction_to_player;
            
            // Determine movement based on distance
            if distance > attack_range {
                // Too far - move toward player
                self.state = EnemyState::Idle;
                self.movement_direction = direction_to_player;
            } else if distance > optimal_distance + 50.0 {
                // In range but not at optimal distance - pursue
                self.state = EnemyState::Pursuing;
                self.movement_direction = direction_to_player;
            } else if distance < optimal_distance - 50.0 {
                // Too close - retreat
                self.state = EnemyState::Retreating;
                self.movement_direction = -direction_to_player;
            } else {
                // At optimal distance - strafe while shooting
                self.state = EnemyState::Strafing;
                
                // Update strafe timer
                self.strafe_timer -= delta_time;
                if self.strafe_timer <= 0.0 {
                    let mut rng = rand::thread_rng();
                    self.strafe_timer = 2.0 + rng.gen::<f32>() * 2.0; // 2-4 seconds
                    self.strafe_direction *= -1.0; // Switch strafe direction
                }
                
                // Calculate perpendicular direction for strafing
                let strafe_dir = Vec2::new(-direction_to_player.y, direction_to_player.x) * self.strafe_direction;
                self.movement_direction = strafe_dir * 0.5; // Slower strafing speed
            }
            
            // Calculate aim position with prediction
            let mut aim_position = player_pos;
            if let Some(vel) = player_velocity {
                if vel.length() > 10.0 {
                    let time_to_target = distance / projectile_speed;
                    aim_position += vel * time_to_target * 0.5;
                }
            }
            self.target_position = Some(aim_position);
        } else {
            self.state = EnemyState::Idle;
            self.movement_direction = Vec2::ZERO;
        }
    }

    pub fn get_shoot_direction(&self) -> Vec2 {
        self.facing_direction
    }
    
    pub fn calculate_projectile_velocity(&self, enemy_pos: Vec2, target_pos: Vec2, projectile_speed: f32) -> Vec2 {
        // Calculate ballistic trajectory to hit target
        let gravity = 500.0; // Match the gravity in projectile_system
        let delta = target_pos - enemy_pos;
        let distance_x = delta.x;
        let distance_y = delta.y;
        
        // For close range, just shoot straight with some upward arc
        let distance = delta.length();
        if distance < 150.0 {
            let direction = delta.normalize_or_zero();
            // Add slight upward arc for close shots
            return Vec2::new(direction.x * projectile_speed, direction.y * projectile_speed - 50.0);
        }
        
        // For longer ranges, calculate proper ballistic arc
        // We'll use a 45-degree launch angle for optimal range
        let angle = 45.0_f32.to_radians();
        
        // Calculate required launch speed for this angle
        // Range equation: R = v^2 * sin(2θ) / g
        // Solving for v: v = sqrt(R * g / sin(2θ))
        let required_speed = (distance * gravity / (2.0 * angle).sin()).sqrt();
        
        // Use the projectile speed but adjust angle if needed
        let speed = projectile_speed.min(required_speed * 1.5); // Cap at 1.5x required
        
        // Calculate velocity components
        let direction_x = if distance_x != 0.0 { distance_x.signum() } else { 0.0 };
        let v_x = direction_x * speed * angle.cos();
        let v_y = -speed * angle.sin(); // Negative for upward
        
        Vec2::new(v_x, v_y)
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
        for (_entity, (enemy, transform, controller, body)) in world.query_mut::<(
            &mut Enemy,
            &mut Transform,
            &mut EnemyController,
            &mut RigidBody,
        )>() {
            enemy.update_timer(delta_time);
            enemy.update_health_bar_timer(delta_time);
            
            controller.update_movement_and_targeting(
                player_position, 
                transform.position,
                enemy.optimal_distance,
                enemy.attack_range,
                Some(player_velocity),
                enemy.projectile_speed,
                delta_time,
            );
            
            // Apply movement based on controller state
            let move_velocity = controller.movement_direction * enemy.movement_speed;
            body.velocity.x = move_velocity.x;
            // Don't override Y velocity to preserve gravity
            
            // Check if can shoot based on state
            let can_shoot = matches!(controller.state, EnemyState::Strafing | EnemyState::Pursuing | EnemyState::Retreating);
            if can_shoot && enemy.can_shoot() {
                // Calculate proper projectile velocity with gravity compensation
                let projectile_velocity = controller.calculate_projectile_velocity(
                    transform.position,
                    player_position,
                    enemy.projectile_speed
                );
                
                let shoot_dir = projectile_velocity.normalize_or_zero();
                let spawn_offset = controller.facing_direction * (enemy.size.x / 2.0 + 10.0);
                let spawn_pos = transform.position + spawn_offset;
                
                projectiles_to_spawn.push((
                    spawn_pos,
                    shoot_dir,
                    projectile_velocity.length(),
                    enemy.damage,
                ));
                
                enemy.reset_shoot_timer();
                controller.state = EnemyState::Shooting;
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