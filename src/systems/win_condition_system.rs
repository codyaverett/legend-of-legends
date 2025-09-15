use crate::game::{LevelManager, WinProgress};
use crate::systems::enemy::Enemy;
use crate::systems::player::Player;
use crate::engine::core::{Transform, Color, Rect};
use crate::engine::rendering::Sprite;
use glam::Vec2;
use hecs::World;

pub fn check_win_conditions(
    world: &mut World,
    level_manager: &mut LevelManager,
) -> (bool, WinProgress) {
    // Get player position
    let mut player_pos = Vec2::ZERO;
    for (_entity, (_, transform)) in world.query::<(&Player, &Transform)>().iter() {
        player_pos = transform.position;
        break;
    }
    
    // Count enemies
    let enemy_count = world.query::<&Enemy>().iter().count();
    
    // Check win condition
    level_manager.check_win_condition(player_pos, enemy_count)
}

pub fn check_collectibles(
    world: &mut World,
    level_manager: &mut LevelManager,
) {
    let mut player_rect = None;
    
    // Get player rect
    for (_entity, (player, transform)) in world.query::<(&Player, &Transform)>().iter() {
        player_rect = Some(Rect::new(
            transform.position.x - player.size.x / 2.0,
            transform.position.y - player.size.y / 2.0,
            player.size.x,
            player.size.y,
        ));
        break;
    }
    
    if let Some(p_rect) = player_rect {
        let collectibles = level_manager.get_collectibles_status();
        
        for (index, (pos, collected)) in collectibles.iter().enumerate() {
            if !collected {
                // Check if player is touching this collectible
                let collectible_rect = Rect::new(
                    pos.x - 16.0,
                    pos.y - 16.0,
                    32.0,
                    32.0,
                );
                
                if p_rect.intersects(&collectible_rect) {
                    level_manager.collect_item(index);
                }
            }
        }
    }
}

pub fn spawn_collectibles(world: &mut World, level_manager: &LevelManager) {
    let collectibles = level_manager.get_collectibles_status();
    
    for (pos, collected) in collectibles {
        if !collected {
            world.spawn((
                CollectibleMarker,
                Transform::new(pos),
                Sprite::new(Vec2::new(32.0, 32.0), Color::new(100, 200, 255, 255)), // Blue collectible
            ));
        }
    }
}

pub fn spawn_goal_marker(world: &mut World, level_manager: &LevelManager) {
    let level = level_manager.get_current_level();
    
    if let Some(goal_pos) = level.goal_position {
        world.spawn((
            GoalMarker,
            Transform::new(goal_pos),
            Sprite::new(Vec2::new(64.0, 64.0), Color::new(50, 255, 50, 200)), // Green goal
        ));
    }
}

// Marker components
#[derive(Debug, Clone)]
pub struct CollectibleMarker;

#[derive(Debug, Clone)]
pub struct GoalMarker;