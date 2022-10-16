use bevy::prelude::*;
use bevy_editor_pls::controls;
use bevy_editor_pls::default_windows::cameras::camera_3d_free::FlycamControls;
// use bevy_editor_pls::default_windows::cameras::EditorCamKind; // set default to flycam?
use bevy_editor_pls::prelude::*;
// use bevy_editor_pls_default_windows::hierarchy::picking::EditorRayCastSource;
use bevy_rapier3d::prelude::*;
use darkest::config::{
    GROUND_HEIGHT, GROUND_SIZE, JUMP_IMPULSE, LIGHT_HEIGHT, LOOKING_AT, LOOK_TRANSLATE_SENS,
    PLAYER_POSITION, PLAYER_SIZE,
};
use darkest::npc::spawn_npcs;
use darkest::player::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin, Jumper, Player};
use smooth_bevy_cameras::LookTransformPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // hasn't worked for me
        // .add_plugin(RapierDebugRenderPlugin::default()) // shows collision debug boundaries?
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin) // live editor, inspect (esc)
        .insert_resource(editor_controls()) // editor controls
        .add_startup_system(set_cam3d_controls) // editor camera controls
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(spawn_npcs)
        .run();
}

/// set up a simple 3D scene
fn setup(
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

    // player camera
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: LOOK_TRANSLATE_SENS,
                ..Default::default()
            },
            PLAYER_POSITION,
            LOOKING_AT,
        ))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Jumper {
            jump_impulse: JUMP_IMPULSE,
            is_jumping: false,
        })
        .insert(Player);

    // player model
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: PLAYER_SIZE })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(PLAYER_POSITION.x, PLAYER_POSITION.y, PLAYER_POSITION.z),
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
        .insert(Restitution::coefficient(-10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Jumper {
            jump_impulse: JUMP_IMPULSE,
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

fn set_cam3d_controls(mut query: Query<&mut FlycamControls>) {
    let mut controls = query.single_mut();
    controls.key_up = KeyCode::Numpad1;
    controls.key_down = KeyCode::Numpad0;
    controls.key_left = KeyCode::Left;
    controls.key_right = KeyCode::Right;
    controls.key_forward = KeyCode::Up;
    controls.key_back = KeyCode::Down;
    // set_active_editor_camera_marker(&world, EditorCamKind::D3Free);
}
