use crate::assets::{get_asset, AssetType, Scenes};
use crate::config::{NPC_POSITION, NPC_RESTITUTION, PLAYER_SIZE};
// use crate::weapon::{spawn_weapon, WeaponType};
use bevy::core::Name;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Npc;

/// Loads the NPC 3D asset (GTLF)
/// https://bevy-cheatbook.github.io/3d/gltf.html
pub fn spawn_npcs(commands: &mut Commands, scenes: &Res<Scenes>) {
    // NPC
    commands
        .spawn_bundle(SceneBundle {
            scene: get_asset(scenes, AssetType::Npc),
            transform: Transform::from_xyz(NPC_POSITION.x, NPC_POSITION.y, NPC_POSITION.z),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            PLAYER_SIZE / 2.0,
            PLAYER_SIZE / 2.0,
            PLAYER_SIZE / 2.0,
        ))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Ccd::enabled())
        .insert(Restitution::coefficient(NPC_RESTITUTION))
        .insert(Npc)
        .insert(Name::new("NPC"));
    /* .with_children(|parent| {
        parent.spawn_bundle(spawn_weapon(WeaponType::Axe, &ass));
    }); */
}
