use crate::engine::core::{Rect, Transform};
use crate::engine::physics::RigidBody;
use crate::game::Level;
use glam::Vec2;
use sdl2::keyboard::Keycode;

pub struct Player {
    pub size: Vec2,
}

impl Player {
    pub fn new() -> Self {
        Self {
            size: Vec2::new(24.0, 40.0),
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
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            speed: 200.0,
            jump_force: 500.0,
            double_jump_force: 400.0,
            is_grounded: false,
            jump_count: 0,
            max_jumps: 2,
            is_spinning: false,
            spin_rotation: 0.0,
            spin_speed: 720.0, // 720 degrees per second (2 full rotations)
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
        &Player,
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
            } else if controller.jump_count < controller.max_jumps {
                // Double jump in air with booster effect
                body.velocity.y = -controller.double_jump_force;
                controller.jump_count += 1;
                controller.is_spinning = true;
                controller.spin_rotation = 0.0;
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
        }
    }
}
