use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use darkest::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin, Jumper, Player};
use smooth_bevy_cameras::LookTransformPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    let ground_size = 50.0;
    let ground_height = 0.1;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Ccd::enabled())
        .insert(Collider::cuboid(ground_size, ground_height, ground_size));

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // player camera
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: 0.1,
                ..Default::default()
            },
            Vec3::new(-2.0, 1.0, 5.0),
            Vec3::new(0., 0., 0.),
        ))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Ccd::enabled())
        .insert(Collider::ball(0.5))
        .insert(Jumper {
            jump_impulse: 1.,
            is_jumping: false,
        })
        .insert(Player);

    // player model
    let player_size = 1.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: player_size })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            player_size / 2.0,
            player_size / 2.0,
            player_size / 2.0,
        ))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Ccd::enabled())
        .insert(GravityScale(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Jumper {
            jump_impulse: 3.0,
            is_jumping: false,
        })
        .insert(Player);
}
