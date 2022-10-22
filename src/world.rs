use crate::config::{GROUND_HEIGHT, GROUND_SIZE, LIGHT_HEIGHT};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// set up a 3D scene
pub fn world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: GROUND_SIZE })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(
            GROUND_SIZE / 2.0,
            GROUND_HEIGHT,
            GROUND_SIZE / 2.0,
        ));

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, LIGHT_HEIGHT, 4.0),
        ..Default::default()
    });
}
