use bevy::prelude::*;
use bevy_editor_pls::controls;
use bevy_editor_pls::default_windows::cameras::camera_3d_free::FlycamControls;

pub struct EditorPlsPlugin;

impl Plugin for EditorPlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(editor_controls()) // editor controls
            .add_startup_system(set_cam3d_controls);
    }
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
