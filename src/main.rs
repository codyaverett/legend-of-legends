mod engine;
mod systems;
mod game;

use anyhow::Result;
use glam::Vec2;
use log::info;
use sdl2::keyboard::Keycode;

use engine::core::{Color, Transform};
use engine::rendering::Sprite;
use engine::physics::{Collider, RigidBody};
use systems::player::{Player, PlayerController, player_movement_system};
use game::{Level, TILE_SIZE};
use game::states::GameState;

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Legends of Legend...");
    
    let mut engine = engine::Engine::new("Legends of Legend", 1280, 720)?;
    
    let level = Level::test_level_1();
    let spawn_pos = level.spawn_point;
    
    let player_entity = engine.world.spawn((
        Player::new(),
        Transform::new(spawn_pos),
        Sprite::new(Vec2::new(24.0, 40.0), Color::new(100, 150, 255, 255)),
        RigidBody::new(1.0),
        Collider::Box { size: Vec2::new(24.0, 40.0) },
        PlayerController::new(),
    ));
    
    info!("Created player entity: {:?}", player_entity);
    info!("Level size: {}x{}", level.width, level.height);
    
    let _game_state = GameState::default();
    
    engine.run(|engine, delta_time| {
        engine.renderer.clear(Color::new(135, 206, 235, 255));
        
        player_movement_system(&mut engine.world, &engine.platform.input, &level, delta_time);
        
        if engine.platform.input.is_key_pressed(Keycode::Num1) {
            engine.renderer.camera.set_zoom(1.0);
            info!("On-foot view");
        }
        if engine.platform.input.is_key_pressed(Keycode::Num2) {
            engine.renderer.camera.set_zoom(0.3);
            info!("Mech view");
        }
        
        if engine.platform.input.is_key_pressed(Keycode::R) {
            for (_entity, (_player, transform)) in engine.world.query_mut::<(&Player, &mut Transform)>() {
                transform.position = level.spawn_point;
            }
            info!("Reset player position");
        }
        
        engine.renderer.camera.update(delta_time);
        
        for (_entity, (_player, transform)) in engine.world.query::<(&Player, &Transform)>().iter() {
            engine.renderer.camera.follow(transform.position, 5.0, delta_time);
        }
        
        for y in 0..level.height {
            for x in 0..level.width {
                if let Some(tile) = level.get_tile(x, y) {
                    if tile.tile_type != game::level::TileType::Empty {
                        let tile_transform = Transform::new(Vec2::new(
                            x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                            y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                        ));
                        let tile_sprite = Sprite::new(Vec2::new(TILE_SIZE, TILE_SIZE), tile.color);
                        engine.renderer.draw_sprite(&tile_sprite, &tile_transform);
                    }
                }
            }
        }
        
        for (_entity, (transform, sprite)) in engine.world.query::<(&Transform, &Sprite)>().iter() {
            engine.renderer.draw_sprite(sprite, transform);
        }
    })?;
    
    info!("Shutting down...");
    Ok(())
}
