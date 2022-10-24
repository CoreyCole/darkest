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

const LOADING_PRINT_INTERVAL: usize = 250;

#[derive(AssetCollection)]
pub struct Scenes {
    #[asset(path = "npc.glb#Scene0")]
    npc: Handle<Scene>,
    #[asset(path = "npc.glb#walk")]
    npc_walk: Handle<Scene>,
    #[asset(path = "battleaxe.glb#Scene0")]
    axe: Handle<Scene>,
}

#[derive(Display, Debug)]
pub enum AssetType {
    Axe,
    Npc,
    NpcWalk,
}

pub fn get_asset(scenes: &Res<Scenes>, ass_type: AssetType) -> Handle<Scene> {
    match ass_type {
        AssetType::Axe => scenes.axe.clone(),
        AssetType::Npc => scenes.npc.clone(),
        AssetType::NpcWalk => scenes.npc_walk.clone(),
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
        .add_system(print_progress);
    }
}

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
    spawn_npcs(&mut commands, &scenes);
    info!("spawning player");
    spawn_player(&mut commands, &mut meshes, &mut materials, &scenes);
}

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    diagnostics: Res<Diagnostics>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        let frame = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
            .map(|diagnostic| diagnostic.value().unwrap_or(0.))
            .unwrap_or(0.);
        if frame % LOADING_PRINT_INTERVAL as f64 == 0. {
            info!(
                "[Frame {}] loading...",
                diagnostics
                    .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                    .map(|diagnostic| diagnostic.value().unwrap_or(0.))
                    .unwrap_or(0.),
            );
        }
        if progress.done > *last_done {
            *last_done = progress.done;
            info!("[Frame {}] Changed progress: {:?}", frame, progress);
        }
    }
}
