use crate::game::GameState;
use crate::npc::spawn_npcs;
use crate::player::spawn_player;
use crate::world::spawn_world;
use bevy::asset::LoadState;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
// use bevy::gltf::GltfNode;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;
use derive_more::Display;
use iyes_progress::{ProgressCounter, ProgressPlugin};

#[derive(AssetCollection)]
pub struct Scenes {
    #[asset(path = "npc.glb#Scene0")]
    npc: Handle<Scene>,
}

#[derive(Display, Debug)]
pub enum AssetType {
    Npc,
}

pub fn get_asset(
    _ass: &Res<AssetServer>,
    scenes: &Res<Scenes>,
    ass_type: AssetType,
) -> Handle<Scene> {
    match ass_type {
        AssetType::Npc => {
            // ass.load(scenes.npc.lo.id());
            // ass.load(scenes.npc.as_weak().id.into());
            scenes.npc.clone()
        } /* e => {
              error!("AssetType not handled: {e}");
              ass.npc.clone()
          }, */
    }
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::InGame)
                .with_collection::<Scenes>(),
        )
        .add_state(GameState::AssetLoading)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // track progress during GameState::AssetLoading and
        // continue to GameState::InGame when complete
        .add_plugin(ProgressPlugin::new(GameState::AssetLoading).continue_to(GameState::InGame))
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(expect))
        /* .add_system_set(
            SystemSet::on_update(GameState::AssetLoading)
                .with_system(track_fake_long_task.before(print_progress)),
        ) */
        .add_system(print_progress);
    }
}

// Time in seconds to complete a custom long-running task.
// If assets are loaded earlier, the current state will not
// be changed until the 'fake long task' is completed (thanks to 'iyes_progress')
// const DURATION_LONG_TASK_IN_SECS: f64 = 2.0;

fn expect(
    mut commands: Commands,
    scenes: Res<Scenes>,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    assert_eq!(ass.get_load_state(scenes.npc.clone()), LoadState::Loaded);
    info!("spawning world");
    spawn_world(&mut commands, &mut meshes, &mut materials, &scenes);
    info!("spawning NPCs");
    spawn_npcs(&mut commands, &ass, &scenes);
    info!("spawning player");
    spawn_player(&mut commands, &mut meshes, &mut materials, &ass, &scenes);
}

/* fn track_fake_long_task(time: Res<Time>, progress: Res<ProgressCounter>) {
    if time.seconds_since_startup() > DURATION_LONG_TASK_IN_SECS {
        info!("Long task is completed");
        progress.manually_track(true.into());
    } else {
        progress.manually_track(false.into());
    }
} */

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<Diagnostics>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
            info!(
                "[Frame {}] Changed progress: {:?}",
                diagnostics
                    .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                    .unwrap_or(0.),
                progress
            );
        }
    }
}
