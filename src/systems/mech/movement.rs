use glam::Vec2;
use hecs::World;
use crate::engine::platform::InputState;
use crate::engine::physics::RigidBody;
use crate::engine::core::Transform;
use crate::game::Level;
use crate::systems::mech::{Mech, MechController};
use sdl2::keyboard::Keycode;

pub fn mech_movement_system(
    world: &mut World,
    input: &InputState,
    level: &Level,
    delta_time: f32,
) {
    for (_entity, (mech, controller, transform, body)) in world
        .query_mut::<(&mut Mech, &mut MechController, &mut Transform, &mut RigidBody)>()
    {
        if !mech.is_occupied {
            continue; // Don't move unmanned mechs
        }

        controller.update(delta_time);
        
        // Regenerate energy when not boosting
        if !controller.is_boosting {
            mech.regenerate_energy(30.0, delta_time); // 30 energy per second
        }

        // Horizontal movement
        let mut move_dir = 0.0;
        if input.is_key_down(Keycode::A) || input.is_key_down(Keycode::Left) {
            move_dir = -1.0;
        }
        if input.is_key_down(Keycode::D) || input.is_key_down(Keycode::Right) {
            move_dir = 1.0;
        }

        // Check for boost (Shift key)
        controller.is_boosting = false;
        let mut speed = mech.normal_speed;
        
        if input.is_key_down(Keycode::LShift) || input.is_key_down(Keycode::RShift) {
            if mech.can_boost() && move_dir != 0.0 {
                controller.is_boosting = true;
                speed = mech.boost_speed;
                mech.use_energy(20.0 * delta_time); // Consume 20 energy per second while boosting
            }
        }

        body.velocity.x = move_dir * speed;

        // Check ground collision for jumping
        let ground_check_pos = Vec2::new(
            transform.position.x,
            transform.position.y + mech.size.y / 2.0 + 5.0,
        );
        
        let tile_x = (ground_check_pos.x / crate::game::TILE_SIZE) as usize;
        let tile_y = (ground_check_pos.y / crate::game::TILE_SIZE) as usize;
        
        let on_ground = if let Some(tile) = level.get_tile(tile_x, tile_y) {
            tile.tile_type != crate::game::level::TileType::Empty
        } else {
            false
        };

        // Jumping with more power than regular player
        if on_ground && (input.is_key_pressed(Keycode::W) || 
                        input.is_key_pressed(Keycode::Up) || 
                        input.is_key_pressed(Keycode::Space)) {
            body.velocity.y = -mech.jump_power;
        }

        // Gravity (mechs are heavier, fall faster)
        if !on_ground {
            body.velocity.y += 2000.0 * delta_time; // Higher gravity for mechs
        } else if body.velocity.y > 0.0 {
            body.velocity.y = 0.0;
        }

        // Ground stomp ability (Down + Jump while in air)
        if !on_ground && controller.can_stomp() {
            if (input.is_key_down(Keycode::S) || input.is_key_down(Keycode::Down)) &&
               input.is_key_pressed(Keycode::Space) {
                body.velocity.y = 1500.0; // Fast downward slam
                controller.do_stomp();
            }
        }

        // Apply movement
        transform.position += body.velocity * delta_time;

        // Keep mech in bounds
        transform.position.x = transform.position.x.max(mech.size.x / 2.0);
        transform.position.x = transform.position.x.min(level.width as f32 * crate::game::TILE_SIZE - mech.size.x / 2.0);
        
        // Prevent falling through the world
        let ground_y = (level.height - 3) as f32 * crate::game::TILE_SIZE - mech.size.y / 2.0;
        if transform.position.y > ground_y {
            transform.position.y = ground_y;
            body.velocity.y = 0.0;
        }
    }
}