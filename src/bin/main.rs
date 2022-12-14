use bevy::prelude::*;
// use bevy_editor_pls::default_windows::cameras::EditorCamKind; // set default to flycam?
use bevy_editor_pls::prelude::*;
// use bevy_editor_pls_default_windows::hierarchy::picking::EditorRayCastSource;
use bevy_rapier3d::prelude::*;
use darkest::assets::AssetsPlugin;
use darkest::editor_pls::EditorPlsPlugin;
use darkest::player::{FpsCameraPlugin, PlayerPlugin};
use smooth_bevy_cameras::LookTransformPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // hasn't worked for me
        // .add_plugin(RapierDebugRenderPlugin::default()) // shows collision debug boundaries?
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetsPlugin)
        .add_plugin(EditorPlugin) // live editor, inspect (esc)
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_plugin(EditorPlsPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
