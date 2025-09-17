use glam::Vec2;
use hecs::{World, Entity};
use crate::engine::core::{Transform, Color};
use crate::engine::physics::{RigidBody, Collider};
use crate::engine::rendering::Sprite;
use crate::systems::player::Player;
use crate::systems::mech::{Mech, MechController, MechWeaponInventory};
use log::info;

pub struct TransformationResult {
    pub success: bool,
    pub message: String,
}

pub fn enter_mech(
    world: &mut World,
    player_entity: Entity,
    mech_entity: Entity,
) -> TransformationResult {
    // Check if mech is available
    if let Ok(mech) = world.get::<&Mech>(mech_entity) {
        if mech.is_occupied {
            return TransformationResult {
                success: false,
                message: "Mech is already occupied".to_string(),
            };
        }
    } else {
        return TransformationResult {
            success: false,
            message: "Invalid mech entity".to_string(),
        };
    }

    // Get player position before hiding them
    let player_pos = if let Ok(transform) = world.get::<&Transform>(player_entity) {
        transform.position
    } else {
        return TransformationResult {
            success: false,
            message: "Player has no transform".to_string(),
        };
    };

    // Update mech to be occupied
    if let Ok(mut mech) = world.get::<&mut Mech>(mech_entity) {
        mech.is_occupied = true;
        mech.pilot_entity = Some(player_entity);
        
        // Update mech color to show it's active
        if let Ok(mut sprite) = world.get::<&mut Sprite>(mech_entity) {
            sprite.color = mech.get_color();
        }
    }

    // Hide the player (remove sprite and collider temporarily)
    let _ = world.remove_one::<Sprite>(player_entity);
    let _ = world.remove_one::<Collider>(player_entity);

    info!("Player entered mech at position {:?}", player_pos);

    TransformationResult {
        success: true,
        message: "Entered Titan mech".to_string(),
    }
}

pub fn exit_mech(
    world: &mut World,
    mech_entity: Entity,
) -> TransformationResult {
    // Get mech info
    let (pilot_entity, mech_pos) = if let Ok(mech) = world.get::<&Mech>(mech_entity) {
        if !mech.is_occupied {
            return TransformationResult {
                success: false,
                message: "Mech is not occupied".to_string(),
            };
        }
        
        let pilot = mech.pilot_entity.unwrap();
        let pos = if let Ok(transform) = world.get::<&Transform>(mech_entity) {
            transform.position
        } else {
            Vec2::ZERO
        };
        (pilot, pos)
    } else {
        return TransformationResult {
            success: false,
            message: "Invalid mech entity".to_string(),
        };
    };

    // Update mech to be unoccupied
    if let Ok(mut mech) = world.get::<&mut Mech>(mech_entity) {
        mech.is_occupied = false;
        mech.pilot_entity = None;
        
        // Update mech color to show it's inactive
        if let Ok(mut sprite) = world.get::<&mut Sprite>(mech_entity) {
            sprite.color = mech.get_color();
        }
    }

    // Restore player components
    let _ = world.insert_one(
        pilot_entity,
        Sprite::new(Vec2::new(24.0, 40.0), Color::new(57, 255, 20, 255)),
    );
    let _ = world.insert_one(
        pilot_entity,
        Collider::Box {
            size: Vec2::new(24.0, 40.0),
        },
    );

    // Position player next to mech
    if let Ok(mut transform) = world.get::<&mut Transform>(pilot_entity) {
        transform.position = Vec2::new(mech_pos.x + 60.0, mech_pos.y);
    }

    info!("Player exited mech at position {:?}", mech_pos);

    TransformationResult {
        success: true,
        message: "Exited Titan mech".to_string(),
    }
}

pub fn find_nearest_mech(world: &World, player_pos: Vec2, max_distance: f32) -> Option<Entity> {
    let mut nearest_mech = None;
    let mut nearest_distance = max_distance;

    for (entity, (mech, transform)) in world.query::<(&Mech, &Transform)>().iter() {
        if mech.is_occupied {
            continue; // Skip occupied mechs
        }

        let distance = (transform.position - player_pos).length();
        if distance < nearest_distance {
            nearest_distance = distance;
            nearest_mech = Some(entity);
        }
    }

    nearest_mech
}

pub fn spawn_mech(world: &mut World, position: Vec2) -> Entity {
    let mech = Mech::new();
    let color = mech.get_color();
    let size = mech.size;
    
    world.spawn((
        mech,
        MechController::new(),
        MechWeaponInventory::new(),
        Transform::new(position),
        Sprite::new(size, color),
        RigidBody::new(10.0), // Mechs are heavy
        Collider::Box { size },
    ))
}