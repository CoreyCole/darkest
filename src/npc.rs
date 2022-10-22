use crate::config::{NPC_POSITION, NPC_RESTITUTION, PLAYER_SIZE};
use crate::weapon::{spawn_weapon, WeaponType};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
#[derive(Component)]
pub struct Npc;

/// Loads the NPC 3D asset (GTLF)
/// https://bevy-cheatbook.github.io/3d/gltf.html
pub fn spawn_npcs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    // NPC
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: PLAYER_SIZE })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
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
        .with_children(|parent| {
            parent.spawn_bundle(spawn_weapon(WeaponType::Axe, &ass));
        });
}
