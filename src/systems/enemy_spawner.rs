use crate::engine::core::{Color, Transform};
use crate::engine::physics::{Collider, RigidBody};
use crate::engine::rendering::Sprite;
use crate::systems::enemy::{Enemy, EnemyController};
use glam::Vec2;
use hecs::World;
use rand::Rng;

pub struct EnemySpawner {
    spawn_timer: f32,
    spawn_interval: f32,
    max_enemies: usize,
    spawn_points: Vec<Vec2>,
    difficulty_multiplier: f32,
    time_elapsed: f32,
}

impl EnemySpawner {
    pub fn new(spawn_points: Vec<Vec2>) -> Self {
        Self {
            spawn_timer: 0.0,
            spawn_interval: 5.0, // Start with 5 second intervals
            max_enemies: 10,
            spawn_points,
            difficulty_multiplier: 1.0,
            time_elapsed: 0.0,
        }
    }
    
    pub fn update(&mut self, world: &mut World, delta_time: f32) {
        self.time_elapsed += delta_time;
        self.spawn_timer += delta_time;
        
        // Gradually increase difficulty over time
        // Every 30 seconds, spawn enemies faster and increase max count
        let difficulty_level = (self.time_elapsed / 30.0).floor();
        self.spawn_interval = (5.0 - difficulty_level * 0.5).max(1.5); // Min 1.5 seconds
        self.max_enemies = (10.0 + difficulty_level * 2.0).min(20.0) as usize; // Max 20 enemies
        self.difficulty_multiplier = 1.0 + difficulty_level * 0.2; // Health and damage scaling
        
        // Check if we should spawn an enemy
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_timer = 0.0;
            
            // Count current enemies
            let enemy_count = world.query::<&Enemy>().iter().count();
            
            if enemy_count < self.max_enemies && !self.spawn_points.is_empty() {
                self.spawn_enemy(world);
            }
        }
    }
    
    fn spawn_enemy(&self, world: &mut World) {
        let mut rng = rand::thread_rng();
        
        // Choose random spawn point
        let spawn_index = rng.gen_range(0..self.spawn_points.len());
        let base_pos = self.spawn_points[spawn_index];
        
        // Add some randomness to spawn position
        let offset_x = rng.gen_range(-50.0..50.0);
        let spawn_pos = Vec2::new(base_pos.x + offset_x, base_pos.y);
        
        // Create enemy with scaled stats based on difficulty
        let mut enemy = Enemy::new();
        enemy.health *= self.difficulty_multiplier;
        enemy.max_health *= self.difficulty_multiplier;
        enemy.damage *= self.difficulty_multiplier;
        
        // Vary enemy appearance slightly
        let size_variation = rng.gen_range(0.9..1.1);
        let enemy_size = enemy.size * size_variation;
        
        // Vary enemy color slightly (shades of red/orange)
        let red = rng.gen_range(200..255);
        let green = rng.gen_range(30..80);
        let blue = rng.gen_range(30..80);
        
        world.spawn((
            enemy,
            Transform::new(spawn_pos),
            Sprite::new(enemy_size, Color::new(red, green, blue, 255)),
            RigidBody::new(1.0),
            Collider::Box { size: enemy_size },
            EnemyController::new(),
        ));
    }
    
    pub fn get_spawn_wave_info(&self) -> (f32, usize, f32) {
        (self.spawn_interval, self.max_enemies, self.difficulty_multiplier)
    }
}