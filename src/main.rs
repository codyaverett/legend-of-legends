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
use engine::ui::Minimap;
use game::states::{GameState, PlayState};
use game::{DayNightCycle, Level, LevelManager, UIManager, WinProgress, TILE_SIZE};
use systems::player::{player_movement_system, player_shooting_system, Player, PlayerController};
use systems::enemy::{Enemy, EnemyController, enemy_ai_system, enemy_physics_system};
use systems::projectile::{Projectile, ProjectileOwner, projectile_system};
use systems::particles::{update_particles};
use systems::enemy_spawner::EnemySpawner;
use systems::win_condition_system::{check_win_conditions, check_collectibles, spawn_collectibles, spawn_goal_marker};
use systems::mech::{Mech, MechController, MechWeaponInventory, mech_movement_system, spawn_mech, enter_mech, exit_mech, find_nearest_mech};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting Legends of Legend...");

    let mut engine = engine::Engine::new("Legends of Legend", 1280, 720)?;

    let mut level_manager = LevelManager::new();
    let spawn_pos = {
        let level = level_manager.get_current_level();
        let spawn = level.spawn_point;
        // Position camera to show ground at bottom of screen
        // Ground is at (height - 3) * TILE_SIZE, we want it at bottom of 720px viewport
        let ground_y = (level.height - 3) as f32 * TILE_SIZE;
        let camera_y = ground_y - 600.0 / 2.0 + 500.0;  // Position camera so ground is near bottom
        engine.renderer.camera.position = Vec2::new(spawn.x, camera_y);
        spawn
    };

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

    // Define spawn points for enemies (off-screen left and right)
    let spawn_points = vec![
        Vec2::new(spawn_pos.x - 800.0, spawn_pos.y),  // Far left
        Vec2::new(spawn_pos.x + 800.0, spawn_pos.y),  // Far right
        Vec2::new(spawn_pos.x - 1000.0, spawn_pos.y - 100.0), // Upper left
        Vec2::new(spawn_pos.x + 1000.0, spawn_pos.y - 100.0), // Upper right
    ];
    
    let mut enemy_spawner = EnemySpawner::new(spawn_points);
    
    // Spawn a few initial enemies
    for i in 0..3 {
        let offset = (i as f32 - 1.0) * 300.0;
        engine.world.spawn((
            Enemy::new(),
            Transform::new(Vec2::new(spawn_pos.x + offset, spawn_pos.y)),
            Sprite::new(Vec2::new(32.0, 48.0), Color::new(255, 50, 50, 255)),
            RigidBody::new(1.0),
            Collider::Box {
                size: Vec2::new(32.0, 48.0),
            },
            EnemyController::new(),
        ));
    }

    info!("Created player entity: {:?}", player_entity);
    {
        let level = level_manager.get_current_level();
        info!("Level size: {}x{}", level.width, level.height);
    }

    let mut game_state = GameState::default();
    let mut current_play_mode = PlayState::OnFoot;
    let mut current_pilot_entity = Some(player_entity);
    let mut current_mech_entity: Option<hecs::Entity> = None;
    
    // Spawn a mech near the player for testing
    let mech_entity = spawn_mech(&mut engine.world, Vec2::new(spawn_pos.x + 200.0, spawn_pos.y));
    info!("Spawned test mech: {:?}", mech_entity);
    let mut day_night_cycle = DayNightCycle::new();
    let mut ui_manager = UIManager::new(1280.0, 720.0);
    
    // Create minimap in top-right corner
    let minimap_pos = Vec2::new(1280.0 - 160.0 - 20.0, 20.0);  // 20px margin from edges
    let minimap_size = Vec2::new(150.0, 150.0);
    let mut minimap = Minimap::new(minimap_pos, minimap_size);

    engine.run(move |engine, delta_time| {
        // Update day/night cycle
        day_night_cycle.update(delta_time);
        // Clear with black instead of sky blue (sky will be drawn as gradient)
        engine.renderer.clear(Color::new(0, 0, 0, 255));

        let level = level_manager.get_current_level();
        
        // Handle mech entry/exit with E key
        if engine.platform.input.is_key_pressed(Keycode::E) {
            match current_play_mode {
                PlayState::OnFoot => {
                    // Try to enter a nearby mech
                    if let Some(pilot_entity) = current_pilot_entity {
                        let pilot_pos = if let Ok(transform) = engine.world.get::<&Transform>(pilot_entity) {
                            Some(transform.position)
                        } else {
                            None
                        };
                        
                        if let Some(pos) = pilot_pos {
                            if let Some(mech) = find_nearest_mech(&engine.world, pos, 100.0) {
                                let result = enter_mech(&mut engine.world, pilot_entity, mech);
                                if result.success {
                                    current_play_mode = PlayState::InMech;
                                    current_mech_entity = Some(mech);
                                    info!("{}", result.message);
                                }
                            }
                        }
                    }
                },
                PlayState::InMech => {
                    // Exit the current mech
                    if let Some(mech) = current_mech_entity {
                        let result = exit_mech(&mut engine.world, mech);
                        if result.success {
                            current_play_mode = PlayState::OnFoot;
                            current_mech_entity = None;
                            info!("{}", result.message);
                        }
                    }
                },
                _ => {}
            }
        }
        
        // Run appropriate movement system based on current mode
        match current_play_mode {
            PlayState::OnFoot => {
                player_movement_system(
                    &mut engine.world,
                    &engine.platform.input,
                    level,
                    delta_time,
                );
            },
            PlayState::InMech => {
                mech_movement_system(
                    &mut engine.world,
                    &engine.platform.input,
                    level,
                    delta_time,
                );
            },
            _ => {}
        }

        // Update enemy spawner
        enemy_spawner.update(&mut engine.world, delta_time);
        
        // Update enemy physics
        enemy_physics_system(&mut engine.world, level, delta_time);

        // Player/Mech shooting system
        let player_projectiles = if current_play_mode == PlayState::OnFoot {
            player_shooting_system(
                &mut engine.world,
                &engine.platform.input,
                &engine.renderer.camera,
                delta_time,
            )
        } else {
            Vec::new() // TODO: Implement mech shooting system
        };
        
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
        let (expired_projectiles, new_particles) = projectile_system(&mut engine.world, level, delta_time);
        
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
        // Auto-adjust zoom based on play mode
        let target_zoom = match current_play_mode {
            PlayState::OnFoot => 1.0,
            PlayState::InMech => 0.5,
            _ => engine.renderer.camera.zoom,
        };
        
        // Smoothly transition camera zoom
        let zoom_speed = 3.0;
        let current_zoom = engine.renderer.camera.zoom;
        let new_zoom = current_zoom + (target_zoom - current_zoom) * zoom_speed * delta_time;
        engine.renderer.camera.set_zoom(new_zoom);
        
        // Manual zoom override
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

        // Camera follow logic based on current mode
        let follow_entity = if current_play_mode == PlayState::InMech {
            current_mech_entity
        } else {
            current_pilot_entity
        };
        
        if let Some(entity) = follow_entity {
            if let Ok(transform) = engine.world.get::<&Transform>(entity) {
                // Follow entity horizontally, but limit vertical movement to keep ground visible
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

        // Layer 8.5: Enemy health bars (rendered above enemies but below UI)
        for (_entity, (enemy, transform)) in engine.world.query::<(&Enemy, &Transform)>().iter() {
            if enemy.should_show_health_bar() && enemy.health > 0.0 {
                // Calculate health bar position (above enemy)
                let bar_width = 40.0;
                let bar_height = 4.0;
                let bar_offset_y = enemy.size.y / 2.0 + 10.0;
                let bar_pos = Vec2::new(
                    transform.position.x - bar_width / 2.0,
                    transform.position.y - bar_offset_y
                );
                
                // Draw background (dark red)
                let bg_transform = Transform::new(Vec2::new(
                    transform.position.x,
                    transform.position.y - bar_offset_y
                ));
                let bg_sprite = Sprite::new(
                    Vec2::new(bar_width, bar_height),
                    Color::new(80, 20, 20, 200)
                );
                engine.renderer.draw_sprite(&bg_sprite, &bg_transform);
                
                // Draw health fill (bright red)
                let health_ratio = enemy.health / enemy.max_health;
                let fill_width = bar_width * health_ratio;
                if fill_width > 0.0 {
                    let fill_transform = Transform::new(Vec2::new(
                        transform.position.x - (bar_width - fill_width) / 2.0,
                        transform.position.y - bar_offset_y
                    ));
                    let fill_sprite = Sprite::new(
                        Vec2::new(fill_width, bar_height),
                        Color::new(255, 60, 60, 200)
                    );
                    engine.renderer.draw_sprite(&fill_sprite, &fill_transform);
                }
                
                // Draw border (white)
                let border_thickness = 1.0;
                // Top border
                let top_border = Sprite::new(
                    Vec2::new(bar_width + border_thickness * 2.0, border_thickness),
                    Color::new(255, 255, 255, 150)
                );
                let top_transform = Transform::new(Vec2::new(
                    transform.position.x,
                    transform.position.y - bar_offset_y - bar_height / 2.0 - border_thickness / 2.0
                ));
                engine.renderer.draw_sprite(&top_border, &top_transform);
                
                // Bottom border
                let bottom_transform = Transform::new(Vec2::new(
                    transform.position.x,
                    transform.position.y - bar_offset_y + bar_height / 2.0 + border_thickness / 2.0
                ));
                engine.renderer.draw_sprite(&top_border, &bottom_transform);
                
                // Left border
                let side_border = Sprite::new(
                    Vec2::new(border_thickness, bar_height + border_thickness * 2.0),
                    Color::new(255, 255, 255, 150)
                );
                let left_transform = Transform::new(Vec2::new(
                    transform.position.x - bar_width / 2.0 - border_thickness / 2.0,
                    transform.position.y - bar_offset_y
                ));
                engine.renderer.draw_sprite(&side_border, &left_transform);
                
                // Right border
                let right_transform = Transform::new(Vec2::new(
                    transform.position.x + bar_width / 2.0 + border_thickness / 2.0,
                    transform.position.y - bar_offset_y
                ));
                engine.renderer.draw_sprite(&side_border, &right_transform);
            }
        }

        // Layer 9: UI Elements (always on top)
        let mut player_health = 100.0;
        let mut player_max_health = 100.0;
        let mut player_energy = 100.0;
        let mut player_max_energy = 100.0;
        let mut player_pos = None;
        let mut player_velocity = None;
        let mut weapon_info = None;
        
        // Collect enemy positions for minimap
        let mut enemy_positions = Vec::new();
        for (_entity, (enemy, transform)) in engine.world.query::<(&Enemy, &Transform)>().iter() {
            if enemy.health > 0.0 {
                let is_boss = enemy.enemy_type == systems::enemy::EnemyType::Tank; // Future boss type
                enemy_positions.push((transform.position, is_boss));
            }
        }
        
        // Get stats based on current play mode
        if current_play_mode == PlayState::InMech {
            // Get mech stats
            if let Some(mech_entity) = current_mech_entity {
                if let Ok(mech) = engine.world.get::<&Mech>(mech_entity) {
                    player_health = mech.health;
                    player_max_health = mech.max_health;
                    player_energy = mech.energy;
                    player_max_energy = mech.max_energy;
                }
                if let Ok(transform) = engine.world.get::<&Transform>(mech_entity) {
                    player_pos = Some(transform.position);
                }
                if let Ok(body) = engine.world.get::<&RB>(mech_entity) {
                    player_velocity = Some(body.velocity);
                }
            }
        } else {
            // Get player stats
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
        }
        
        let entity_count = engine.world.len() as usize;
        
        // Create temporary player for UI update (works for both player and mech)
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
        
        // Render minimap
        if let Some(player_position) = player_pos {
            let level = level_manager.get_current_level();
            let collectibles: Vec<Vec2> = level.collectibles
                .iter()
                .filter(|(_, collected)| !collected)
                .map(|(pos, _)| *pos)
                .collect();
            
            let camera_pos = engine.renderer.camera.position;
            let goal_pos = level.goal_position;
            
            minimap.render(
                &mut engine.renderer,
                player_position,
                &enemy_positions,
                &collectibles,
                goal_pos,
                Vec2::ZERO,  // Camera offset should be zero for screen-space UI
            );
        }
    })?;

    info!("Shutting down...");
    Ok(())
}
