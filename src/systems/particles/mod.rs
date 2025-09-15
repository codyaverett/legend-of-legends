use crate::engine::core::{Color, Transform};
use crate::engine::rendering::Sprite;
use glam::Vec2;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum ParticleType {
    Impact,
    Explosion,
    Trail,
    Spark,
    Smoke,
    Blood,
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: Vec2,
    pub color: Color,
    pub gravity_scale: f32,
    pub drag: f32,
    pub fade_out: bool,
    pub size_decay: f32, // How fast the particle shrinks
}

impl Particle {
    pub fn new(particle_type: ParticleType, position: Vec2, velocity: Vec2) -> Self {
        let (max_lifetime, size, color, gravity_scale, drag, fade_out, size_decay) = match particle_type {
            ParticleType::Impact => (
                0.3,
                Vec2::new(4.0, 4.0),
                Color::new(255, 255, 200, 255),
                0.5,
                2.0,
                true,
                0.5,
            ),
            ParticleType::Explosion => (
                0.8,
                Vec2::new(8.0, 8.0),
                Color::new(255, 150, 50, 255),
                0.1,
                1.0,
                true,
                0.3,
            ),
            ParticleType::Trail => (
                0.5,
                Vec2::new(3.0, 3.0),
                Color::new(200, 200, 255, 128),
                0.0,
                3.0,
                true,
                0.8,
            ),
            ParticleType::Spark => (
                0.4,
                Vec2::new(2.0, 2.0),
                Color::new(255, 255, 100, 255),
                1.0,
                0.5,
                true,
                0.2,
            ),
            ParticleType::Smoke => (
                1.5,
                Vec2::new(12.0, 12.0),
                Color::new(100, 100, 100, 128),
                -0.2, // Floats up
                2.0,
                true,
                -0.3, // Grows over time
            ),
            ParticleType::Blood => (
                0.6,
                Vec2::new(3.0, 3.0),
                Color::new(200, 50, 50, 255),
                2.0,
                1.0,
                true,
                0.1,
            ),
        };

        Self {
            particle_type,
            position,
            velocity,
            lifetime: 0.0,
            max_lifetime,
            size,
            color,
            gravity_scale,
            drag,
            fade_out,
            size_decay,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.lifetime += delta_time;
        
        // Apply physics
        self.velocity.y += self.gravity_scale * 500.0 * delta_time;
        self.velocity *= 1.0 - self.drag * delta_time;
        self.position += self.velocity * delta_time;
        
        // Update size
        if self.size_decay != 0.0 {
            let size_factor = 1.0 - (self.lifetime / self.max_lifetime) * self.size_decay.abs();
            let size_multiplier = if self.size_decay > 0.0 {
                size_factor.max(0.1)
            } else {
                2.0 - size_factor.min(1.9)
            };
            self.size = self.size * size_multiplier;
        }
        
        // Fade out
        if self.fade_out {
            let alpha_factor = 1.0 - (self.lifetime / self.max_lifetime);
            self.color.a = (self.color.a as f32 * alpha_factor) as u8;
        }
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime >= self.max_lifetime
    }
}

pub struct ParticleSystem;

impl ParticleSystem {
    pub fn create_impact_particles(position: Vec2, impact_direction: Vec2, count: u32) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        
        for _ in 0..count {
            let angle = rng.gen_range(-1.5..1.5);
            let speed = rng.gen_range(50.0..200.0);
            let velocity = Vec2::new(
                -impact_direction.x * speed + angle * 50.0,
                -impact_direction.y * speed - rng.gen_range(50.0..150.0),
            );
            
            particles.push(Particle::new(ParticleType::Spark, position, velocity));
        }
        
        particles
    }

    pub fn create_explosion_particles(position: Vec2, count: u32, force: f32) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Create explosion particles
        for _ in 0..count {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(100.0..300.0) * force;
            let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
            
            particles.push(Particle::new(ParticleType::Explosion, position, velocity));
        }
        
        // Add smoke
        for _ in 0..count/2 {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(20.0..60.0);
            let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed - 30.0);
            
            particles.push(Particle::new(ParticleType::Smoke, position, velocity));
        }
        
        particles
    }

    pub fn create_trail_particles(position: Vec2, direction: Vec2) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Create a small trail
        let velocity = -direction * 50.0 + Vec2::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        );
        
        particles.push(Particle::new(ParticleType::Trail, position, velocity));
        
        particles
    }

    pub fn create_muzzle_flash(position: Vec2, direction: Vec2, weapon_type: crate::systems::weapons::WeaponType) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        
        let count = match weapon_type {
            crate::systems::weapons::WeaponType::Shotgun => 12,
            crate::systems::weapons::WeaponType::RocketLauncher => 8,
            _ => 4,
        };
        
        for _ in 0..count {
            let spread = rng.gen_range(-0.3..0.3);
            let speed = rng.gen_range(100.0..250.0);
            let velocity = Vec2::new(
                direction.x * speed + spread * 50.0,
                direction.y * speed + spread * 50.0,
            );
            
            let mut particle = Particle::new(ParticleType::Spark, position, velocity);
            particle.color = match weapon_type {
                crate::systems::weapons::WeaponType::LaserRifle => Color::new(255, 50, 255, 255),
                crate::systems::weapons::WeaponType::PlasmaGun => Color::new(100, 255, 255, 255),
                _ => Color::new(255, 200, 100, 255),
            };
            particle.max_lifetime = 0.2;
            
            particles.push(particle);
        }
        
        particles
    }
}

pub fn update_particles(world: &mut hecs::World, delta_time: f32) -> Vec<hecs::Entity> {
    let mut expired_particles = Vec::new();
    
    for (entity, (particle, transform)) in world.query_mut::<(&mut Particle, &mut Transform)>() {
        particle.update(delta_time);
        transform.position = particle.position;
        
        if particle.is_expired() {
            expired_particles.push(entity);
        }
    }
    
    expired_particles
}

pub fn spawn_particle(world: &mut hecs::World, particle: Particle) {
    world.spawn((
        particle.clone(),
        Transform::new(particle.position),
        Sprite::new(particle.size, particle.color),
    ));
}