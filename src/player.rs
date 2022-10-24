use crate::assets::Scenes;
use crate::config::{JUMP_IMPULSE, LOOKING_AT, LOOK_TRANSLATE_SENS, PLAYER_POSITION, PLAYER_SIZE};
use crate::weapon::{spawn_weapon, WeaponType};
use bevy::{
    app::prelude::*,
    core::Name,
    ecs::{bundle::Bundle, prelude::*},
    input::{mouse::MouseMotion, prelude::*},
    math::prelude::*,
    prelude::*,
    transform::components::Transform,
    window::Windows,
};
use bevy_editor_pls::EditorState;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::{dynamics::Velocity, pipeline::CollisionEvent};
use serde::{Deserialize, Serialize};
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, Smoother};

impl Plugin for FpsCameraPlugin {
    fn build(&self, app: &mut App) {
        let app = app
            .add_system(player_jumps)
            .add_system(jump_reset)
            .add_system(control_system)
            .add_system(grab_cursor)
            .add_event::<ControlEvent>();

        if !self.override_input_system {
            app.add_system(default_input_map);
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
}

#[derive(Default)]
pub struct FpsCameraPlugin {
    pub override_input_system: bool,
}

impl FpsCameraPlugin {
    pub fn new(override_input_system: bool) -> Self {
        Self {
            override_input_system,
        }
    }
}

#[derive(Bundle)]
pub struct FpsCameraBundle {
    controller: FpsCameraController,
    #[bundle]
    look_transform: LookTransformBundle,
    transform: Transform,
}

/// Your typical first-person camera controller.
#[derive(Clone, Component, Copy, Debug, Deserialize, Serialize)]
pub struct FpsCameraController {
    pub enabled: bool,
    pub mouse_rotate_sensitivity: Vec2,
    pub translate_sensitivity: f32,
    pub smoothing_weight: f32,
}

impl Default for FpsCameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            mouse_rotate_sensitivity: Vec2::splat(0.002),
            translate_sensitivity: 0.5,
            smoothing_weight: 0.9,
        }
    }
}

pub enum ControlEvent {
    Rotate(Vec2),
    TranslateEye(Vec3),
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(modify_body_locked_flags);
    }
}

/* Lock translations and/or rotations inside of a system. */
fn modify_body_locked_flags(mut locked_axes: Query<&mut LockedAxes>) {
    for mut locked_axes in locked_axes.iter_mut() {
        *locked_axes = LockedAxes::ROTATION_LOCKED;
    }
}

/// player model
pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ass: &Res<AssetServer>,
    _scenes: &Res<Scenes>,
) {
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
        .insert(LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED_X)
        .insert(Name::new("Player"))
        .insert(Player)
        .with_children(|parent| {
            // weapon
            parent
                .spawn_bundle(spawn_weapon(WeaponType::Axe, &ass))
                .insert(RigidBody::Fixed)
                .insert(Name::new("Weapon"))
                .insert(Collider::ball(PLAYER_SIZE / 10.0))
                /* .insert_bundle(TransformBundle::from(Transform::from_xyz(
                    0.0,
                    PLAYER_SIZE / 2., // translate collider to melee weapon hitbox
                    0.0,
                ))) */
                .insert(Ccd::enabled());
            // camera
            parent
                .spawn_bundle(Camera3dBundle::default())
                .insert_bundle(FpsCameraBundle::new(
                    FpsCameraController {
                        translate_sensitivity: LOOK_TRANSLATE_SENS,
                        ..Default::default()
                    },
                    PLAYER_POSITION,
                    LOOKING_AT,
                ));
        });
}

pub fn default_input_map(
    mut events: EventWriter<ControlEvent>,
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    controllers: Query<&FpsCameraController>,
) {
    // Can only control one camera at a time.
    let controller = if let Some(controller) = controllers.iter().find(|c| c.enabled) {
        controller
    } else {
        return;
    };
    let FpsCameraController {
        translate_sensitivity,
        mouse_rotate_sensitivity,
        ..
    } = *controller;

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta += event.delta;
    }

    events.send(ControlEvent::Rotate(
        mouse_rotate_sensitivity * cursor_delta,
    ));

    let mut speed_mod = 1.0;
    if keyboard.pressed(KeyCode::LShift) {
        speed_mod = 2.0;
    }

    for (key, dir) in [
        (KeyCode::W, Vec3::Z),
        (KeyCode::A, Vec3::X),
        (KeyCode::S, -Vec3::Z),
        (KeyCode::D, -Vec3::X),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            events.send(ControlEvent::TranslateEye(
                speed_mod * translate_sensitivity * dir,
            ));
        }
    }
}

fn player_jumps(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Jumper, &mut Velocity), With<Player>>,
) {
    for (mut jumper, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && !jumper.is_jumping {
            velocity.linvel = Vec3::new(0., jumper.jump_impulse, 0.);
            jumper.is_jumping = true;
        }
    }
}

pub fn jump_reset(
    mut query: Query<(Entity, &mut Jumper)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        for (entity, mut jumper) in query.iter_mut() {
            set_jumping_false_if_touching_floor(&entity, &mut jumper, contact_event);
        }
    }
}

fn set_jumping_false_if_touching_floor(
    entity: &Entity,
    jumper: &mut Jumper,
    event: &CollisionEvent,
) {
    if let CollisionEvent::Started(e1, e2, _flags) = event {
        if e1 == entity || e2 == entity {
            jumper.is_jumping = false;
        }
    }
}

impl FpsCameraBundle {
    pub fn new(controller: FpsCameraController, eye: Vec3, target: Vec3) -> Self {
        // Make sure the transform is consistent with the controller to start.
        let transform = Transform::from_translation(eye).looking_at(target, Vec3::Y);

        Self {
            controller,
            look_transform: LookTransformBundle {
                transform: LookTransform::new(eye, target),
                smoother: Smoother::new(controller.smoothing_weight),
            },
            transform,
        }
    }
}

pub fn control_system(
    mut events: EventReader<ControlEvent>,
    mut cameras: Query<(&FpsCameraController, &mut LookTransform)>,
    mut players: Query<(&Player, &mut Transform)>,
) {
    // Can only control one camera at a time.
    let mut transform = if let Some((_, transform)) = cameras.iter_mut().find(|c| c.0.enabled) {
        transform
    } else {
        return;
    };

    let look_vector = transform.look_direction().unwrap();
    let mut look_angles = LookAngles::from_vector(look_vector);

    let yaw_rot = Quat::from_axis_angle(Vec3::Y, look_angles.get_yaw());
    let rot_x = yaw_rot * Vec3::X;
    let rot_y = yaw_rot * Vec3::Y;
    let rot_z = yaw_rot * Vec3::Z;

    for event in events.iter() {
        match event {
            ControlEvent::Rotate(delta) => {
                // Rotates with pitch and yaw.
                look_angles.add_yaw(-delta.x);
                look_angles.add_pitch(-delta.y);
                for (_player, mut transform) in players.iter_mut() {
                    let quat = Quat::from_rotation_y(-delta.x);
                    transform.rotate(quat);
                }
            }
            ControlEvent::TranslateEye(delta) => {
                // Translates up/down (Y) left/right (X) and forward/back (Z).
                let translation = delta.x * rot_x + delta.y * rot_y + delta.z * rot_z;
                // transform.eye += translation;
                for (_player, mut transform) in players.iter_mut() {
                    transform.translation += translation;
                }
            }
        }
    }

    look_angles.assert_not_looking_up();

    transform.target = transform.eye + transform.radius() * look_angles.unit_vector();
}

fn grab_cursor(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    editor: Res<EditorState>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) && !editor.active {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}
