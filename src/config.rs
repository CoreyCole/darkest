use bevy::prelude::Vec3;

/// Size of the platform
pub const GROUND_SIZE: f32 = 100.0;
/// Vertical thickness of the platform
pub const GROUND_HEIGHT: f32 = 0.1;
/// Player's height, length and width
pub const PLAYER_SIZE: f32 = 7.0;
/// Players spawn position
pub const PLAYER_POSITION: Vec3 = Vec3::new(0.0, PLAYER_SIZE / 2.0 + 1.0, 0.0);
/// Distance from player to place weapon
pub const WEAPON_Z_OFFSET: f32 = -1.0 * PLAYER_SIZE / 2.0;
/// NPC spawn position
pub const NPC_POSITION: Vec3 = Vec3::new(GROUND_SIZE / 4.0, PLAYER_POSITION.y, PLAYER_POSITION.z);
/// NPC restitution
/// https://docs.rs/bevy_rapier3d/latest/bevy_rapier3d/geometry/struct.Restitution.html
pub const NPC_RESTITUTION: f32 = 0.0;
/// Battleaxe asset path
pub const AXE_ASSET_PATH: &str = "battleaxe.gltf#Scene0";
/// Spawned player looking at this point
pub const LOOKING_AT: Vec3 = Vec3::new(GROUND_SIZE, PLAYER_SIZE / 2.0, 0.0);
/// Rate at which mouse movements are translated into rotation
pub const LOOK_TRANSLATE_SENS: f32 = 0.1;
/// Initial velocity of jumping
pub const JUMP_IMPULSE: f32 = 5.0;
/// Y position for lighting/ceiling
pub const LIGHT_HEIGHT: f32 = PLAYER_SIZE * 2.0;
