mod engine;
mod game;
mod systems;

use anyhow::Result;
use glam::Vec2;
use log::info;
use sdl2::keyboard::Keycode;

use engine::core::{Color, Transform};
use engine::physics::{Collider, RigidBody};
use engine::rendering::Sprite;
use engine::physics::RigidBody as RB;
use game::states::GameState;
use game::{DayNightCycle, Level, UIManager, TILE_SIZE};
use systems::player::{player_movement_system, player_shooting_system, Player, PlayerController, ProjectileSpawnData};
use systems::enemy::{Enemy, EnemyController, enemy_ai_system, enemy_physics_system};
use systems::projectile::{Projectile, ProjectileOwner, projectile_system};
use systems::particles::{update_particles};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Legends of Legend...");

    let mut engine = engine::Engine::new("Legends of Legend", 1280, 720)?;

    let level = Level::test_level_1();
    let spawn_pos = level.spawn_point;
    
    // Position camera to show ground at bottom of screen
    // Ground is at (height - 3) * TILE_SIZE, we want it at bottom of 720px viewport
    let ground_y = (level.height - 3) as f32 * TILE_SIZE;
    let camera_y = ground_y - 600.0 / 2.0 + 500.0;  // Position camera so ground is near bottom
    engine.renderer.camera.position = Vec2::new(spawn_pos.x, camera_y);

    let player_entity = engine.world.spawn((
        Player::new(),
        Transform::new(spawn_pos),
        Sprite::new(Vec2::new(24.0, 40.0), Color::new(57, 255, 20, 255)), // Neon green
        RigidBody::new(1.0),
        Collider::Box {
            size: Vec2::new(24.0, 40.0),
        },
        PlayerController::new(),
    ));

    // Spawn enemies at various positions
    let enemy_positions = vec![
        Vec2::new(spawn_pos.x + 400.0, spawn_pos.y),
        Vec2::new(spawn_pos.x - 300.0, spawn_pos.y),
        Vec2::new(spawn_pos.x + 700.0, spawn_pos.y - 100.0),
    ];

    for pos in enemy_positions {
        engine.world.spawn((
            Enemy::new(),
            Transform::new(pos),
            Sprite::new(Vec2::new(32.0, 48.0), Color::new(255, 50, 50, 255)), // Red enemy
            RigidBody::new(1.0),
            Collider::Box {
                size: Vec2::new(32.0, 48.0),
            },
            EnemyController::new(),
        ));
    }

    info!("Created player entity: {:?}", player_entity);
    info!("Level size: {}x{}", level.width, level.height);

    let _game_state = GameState::default();
    let mut day_night_cycle = DayNightCycle::new();
    let mut ui_manager = UIManager::new(1280.0, 720.0);

    engine.run(move |engine, delta_time| {
        // Update day/night cycle
        day_night_cycle.update(delta_time);
        // Clear with black instead of sky blue (sky will be drawn as gradient)
        engine.renderer.clear(Color::new(0, 0, 0, 255));

        player_movement_system(
            &mut engine.world,
            &engine.platform.input,
            &level,
            delta_time,
        );

        // Update enemy physics
        enemy_physics_system(&mut engine.world, &level, delta_time);

        // Player shooting system
        let player_projectiles = player_shooting_system(
            &mut engine.world,
            &engine.platform.input,
            &engine.renderer.camera,
            delta_time,
        );
        
        for spawn_data in player_projectiles {
            let mut body = RigidBody::new(0.1);
            body.velocity = spawn_data.direction * spawn_data.weapon.projectile_force;
            
            let projectile = Projectile::from_weapon(&spawn_data.weapon, ProjectileOwner::Player);
            
            engine.world.spawn((
                projectile.clone(),
                Transform::new(spawn_data.position),
                Sprite::new(projectile.size, projectile.color),
                body,
                Collider::Circle { radius: projectile.size.x / 2.0 },
            ));
        }

        // Run enemy AI and spawn projectiles
        let enemy_projectiles = enemy_ai_system(&mut engine.world, delta_time);
        
        for (pos, dir, speed, damage) in enemy_projectiles {
            let mut body = RigidBody::new(0.1);
            body.velocity = dir * speed;
            
            engine.world.spawn((
                Projectile::new(damage, ProjectileOwner::Enemy),
                Transform::new(pos),
                Sprite::new(Vec2::new(8.0, 8.0), Color::new(255, 200, 0, 255)), // Yellow/orange projectile
                body,
                Collider::Circle { radius: 4.0 },
            ));
        }

        // Update projectiles with physics and check collisions
        let (expired_projectiles, new_particles) = projectile_system(&mut engine.world, &level, delta_time);
        
        // Spawn new particles from projectile impacts
        for particle in new_particles {
            systems::particles::spawn_particle(&mut engine.world, particle);
        }
        
        // Update particles
        let expired_particles = update_particles(&mut engine.world, delta_time);
        
        // Remove expired projectiles and particles
        for entity in expired_projectiles.into_iter().chain(expired_particles) {
            let _ = engine.world.despawn(entity);
        }

        // Camera zoom controls (moved to F keys to free up number keys for weapons)
        if engine.platform.input.is_key_pressed(Keycode::F1) {
            engine.renderer.camera.set_zoom(1.0);  // Normal view
            info!("On-foot view");
        }
        if engine.platform.input.is_key_pressed(Keycode::F2) {
            engine.renderer.camera.set_zoom(0.5);  // Zoomed out for wider view
            info!("Wide view");
        }
        if engine.platform.input.is_key_pressed(Keycode::F4) {
            engine.renderer.camera.set_zoom(0.25);  // Very wide view
            info!("Mech view");
        }

        if engine.platform.input.is_key_pressed(Keycode::F5) {
            for (_entity, (player, transform)) in
                engine.world.query_mut::<(&mut Player, &mut Transform)>()
            {
                transform.position = level.spawn_point;
                player.health = player.max_health;
                player.energy = player.max_energy;
            }
            info!("Reset player position");
        }

        if engine.platform.input.is_key_pressed(Keycode::F3) {
            ui_manager.toggle_debug();
            info!("Toggled debug overlay");
        }

        engine.renderer.camera.update(delta_time);

        for (_entity, (_player, transform)) in engine.world.query::<(&Player, &Transform)>().iter()
        {
            // Follow player horizontally, but limit vertical movement to keep ground visible
            let ground_y = (level.height - 3) as f32 * TILE_SIZE;
            let min_camera_y = ground_y - 720.0 / engine.renderer.camera.zoom / 2.0 + 100.0;
            
            let target_pos = Vec2::new(
                transform.position.x,
                transform.position.y.max(min_camera_y)
            );
            
            engine
                .renderer
                .camera
                .follow(target_pos, 5.0, delta_time);
        }

        // === LAYERED RENDERING ===
        
        // Layer 1: Dynamic sky gradient background based on day/night cycle
        let dynamic_sky = day_night_cycle.generate_sky_gradient(
            level.width as f32 * TILE_SIZE,
            level.height as f32 * TILE_SIZE
        );
        for (rect, color) in &dynamic_sky {
            let transform = Transform::new(Vec2::new(
                rect.x + rect.width / 2.0,
                rect.y + rect.height / 2.0,
            ));
            let sprite = Sprite::new(Vec2::new(rect.width, rect.height), *color);
            engine.renderer.draw_sprite(&sprite, &transform);
        }
        
        // Layer 2: Clouds (with slight parallax and day/night tinting)
        let ambient = day_night_cycle.get_ambient_light();
        for (pos, radius, color) in &level.clouds {
            let parallax_offset = engine.renderer.camera.position * 0.05;
            let cloud_transform = Transform::new(*pos - parallax_offset);
            
            // Tint clouds based on time of day
            let tinted_color = Color::new(
                ((color.r as f32 * ambient.r as f32 / 255.0) as u8).min(255),
                ((color.g as f32 * ambient.g as f32 / 255.0) as u8).min(255),
                ((color.b as f32 * ambient.b as f32 / 255.0) as u8).min(255),
                color.a
            );
            
            let cloud_sprite = Sprite::new(Vec2::new(*radius * 2.0, *radius * 1.5), tinted_color);
            engine.renderer.draw_sprite(&cloud_sprite, &cloud_transform);
        }
        
        // Buildings and props are disabled for now
        // They can be re-enabled later when needed

        // Layer 7: Gameplay tiles - optimized rendering with proper alignment
        // Calculate visible tile range to avoid rendering off-screen tiles
        let cam_pos = engine.renderer.camera.position;
        let zoom = engine.renderer.camera.zoom;
        let viewport = engine.renderer.camera.viewport_size;
        
        let min_x = ((cam_pos.x - viewport.x / zoom / 2.0) / TILE_SIZE).floor() as usize;
        let max_x = ((cam_pos.x + viewport.x / zoom / 2.0) / TILE_SIZE).ceil() as usize + 1;
        let min_y = ((cam_pos.y - viewport.y / zoom / 2.0) / TILE_SIZE).floor() as usize;
        let max_y = ((cam_pos.y + viewport.y / zoom / 2.0) / TILE_SIZE).ceil() as usize + 1;
        
        // Clamp to level bounds
        let min_x = min_x.max(0);
        let max_x = max_x.min(level.width);
        let min_y = min_y.max(0);
        let max_y = max_y.min(level.height);
        
        for y in min_y..max_y {
            for x in min_x..max_x {
                if let Some(tile) = level.get_tile(x, y) {
                    if tile.tile_type != game::level::TileType::Empty {
                        // Use precise tile positioning to ensure alignment
                        let tile_transform = Transform::new(Vec2::new(
                            (x as f32 + 0.5) * TILE_SIZE,
                            (y as f32 + 0.5) * TILE_SIZE,
                        ));
                        // Add a small overlap to prevent gaps (slightly larger than TILE_SIZE)
                        let tile_sprite = Sprite::new(Vec2::new(TILE_SIZE + 0.5, TILE_SIZE + 0.5), tile.color);
                        engine.renderer.draw_sprite(&tile_sprite, &tile_transform);
                    }
                }
            }
        }

        // Layer 8: Entities (player, enemies)
        for (_entity, (transform, sprite)) in engine.world.query::<(&Transform, &Sprite)>().iter() {
            engine.renderer.draw_sprite(sprite, transform);
        }

        // Layer 9: UI Elements (always on top)
        let mut player_health = 100.0;
        let mut player_max_health = 100.0;
        let mut player_energy = 100.0;
        let mut player_max_energy = 100.0;
        let mut player_pos = None;
        let mut player_velocity = None;
        let mut weapon_info = None;
        
        for (_entity, (player, transform, body)) in engine.world.query::<(&Player, &Transform, &RB)>().iter() {
            player_health = player.health;
            player_max_health = player.max_health;
            player_energy = player.energy;
            player_max_energy = player.max_energy;
            player_pos = Some(transform.position);
            player_velocity = Some(body.velocity);
        }
        
        // Get weapon information from PlayerController
        for (_entity, controller) in engine.world.query::<&PlayerController>().with::<&Player>().iter() {
            let weapon = controller.weapon_inventory.current_weapon();
            let index = controller.weapon_inventory.current_weapon_index;
            weapon_info = Some((weapon.clone(), index));
        }
        
        let entity_count = engine.world.len() as usize;
        
        // Create temporary player for UI update
        let ui_player = Player {
            size: Vec2::new(24.0, 40.0),
            health: player_health,
            max_health: player_max_health,
            energy: player_energy,
            max_energy: player_max_energy,
        };
        
        ui_manager.update(
            delta_time,
            Some(&ui_player),
            weapon_info.as_ref().map(|(w, i)| (w, *i)),
            &day_night_cycle,
            entity_count,
            player_pos,
            player_velocity,
        );
        
        ui_manager.render(&mut engine.renderer);
    })?;

    info!("Shutting down...");
    Ok(())
}
