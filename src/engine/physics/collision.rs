use glam::Vec2;
use crate::engine::core::Rect;

#[derive(Debug, Clone)]
pub enum Collider {
    Box { size: Vec2 },
    Circle { radius: f32 },
}

impl Collider {
    pub fn check_collision(&self, pos1: Vec2, other: &Collider, pos2: Vec2) -> bool {
        match (self, other) {
            (Collider::Box { size: size1 }, Collider::Box { size: size2 }) => {
                let rect1 = Rect::new(
                    pos1.x - size1.x / 2.0,
                    pos1.y - size1.y / 2.0,
                    size1.x,
                    size1.y,
                );
                let rect2 = Rect::new(
                    pos2.x - size2.x / 2.0,
                    pos2.y - size2.y / 2.0,
                    size2.x,
                    size2.y,
                );
                rect1.intersects(&rect2)
            }
            (Collider::Circle { radius: r1 }, Collider::Circle { radius: r2 }) => {
                let distance = (pos2 - pos1).length();
                distance < r1 + r2
            }
            (Collider::Box { size }, Collider::Circle { radius }) |
            (Collider::Circle { radius }, Collider::Box { size }) => {
                let (box_pos, circle_pos) = if matches!(self, Collider::Box { .. }) {
                    (pos1, pos2)
                } else {
                    (pos2, pos1)
                };
                
                let rect = Rect::new(
                    box_pos.x - size.x / 2.0,
                    box_pos.y - size.y / 2.0,
                    size.x,
                    size.y,
                );
                
                let closest_x = circle_pos.x.clamp(rect.x, rect.x + rect.width);
                let closest_y = circle_pos.y.clamp(rect.y, rect.y + rect.height);
                let closest = Vec2::new(closest_x, closest_y);
                
                (circle_pos - closest).length() < *radius
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub drag: f32,
}

impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass,
            drag: 0.1,
        }
    }
    
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.velocity += self.acceleration * delta_time;
        self.velocity *= 1.0 - self.drag * delta_time;
        self.acceleration = Vec2::ZERO;
    }
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::new(1.0)
    }
}