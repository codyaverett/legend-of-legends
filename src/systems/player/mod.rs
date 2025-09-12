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
    pub is_grounded: bool,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            speed: 200.0,
            jump_force: 500.0,
            is_grounded: false,
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

        if controller.is_grounded {
            if input.is_key_pressed(Keycode::Space)
                || input.is_key_pressed(Keycode::W)
                || input.is_key_pressed(Keycode::Up)
            {
                body.velocity.y = -controller.jump_force;
                controller.is_grounded = false;
            }
        }

        movement = movement.normalize_or_zero();
        body.velocity.x = movement.x * controller.speed;

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
        }
    }
}
