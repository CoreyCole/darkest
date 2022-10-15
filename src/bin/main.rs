use bevy::prelude::*;
use bevy_editor_pls::controls;
use bevy_editor_pls::prelude::*;
// use bevy_editor_pls_default_windows::hierarchy::picking::EditorRayCastSource;
use bevy_rapier3d::prelude::*;
use darkest::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin, Jumper, Player};
use smooth_bevy_cameras::LookTransformPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .insert_resource(editor_controls())
        .add_startup_system(set_cam3d_controls)
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
        .insert(Collider::cuboid(
            ground_size / 2.0,
            ground_height,
            ground_size / 2.0,
        ));

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // player camera
    let player_size = 7.0;
    let player_position = Vec3::new(0.0, player_size / 2.0 + 1.0, 0.0);
    let looking_at = Vec3::new(20.0, player_size / 2.0, 0.0);
    let jump_impulse = 5.0;
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: 0.1,
                ..Default::default()
            },
            player_position,
            looking_at,
        ))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Jumper {
            jump_impulse,
            is_jumping: false,
        })
        .insert(Player);

    // player model
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: player_size })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(player_position.x, player_position.y, player_position.z),
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
        .insert(Restitution::coefficient(-10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Jumper {
            jump_impulse,
            is_jumping: false,
        })
        .insert(Player);
}

fn editor_controls() -> controls::EditorControls {
    let mut editor_controls = controls::EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}

fn set_cam3d_controls(
    mut query: Query<
        &mut bevy_editor_pls::default_windows::cameras::camera_3d_free::FlycamControls,
    >,
) {
    let mut controls = query.single_mut();
    controls.key_up = KeyCode::Numpad1;
    controls.key_down = KeyCode::Numpad0;
    controls.key_left = KeyCode::Left;
    controls.key_right = KeyCode::Right;
    controls.key_forward = KeyCode::Up;
    controls.key_back = KeyCode::Down;
}
