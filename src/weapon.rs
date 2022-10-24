use crate::assets::{get_asset, AssetType, Scenes};
use crate::config::WEAPON_Z_OFFSET;
use bevy::prelude::*;
pub const AXE_ASSET_PATH: &str = "battleaxe.glb#Scene0";

pub enum WeaponType {
    Axe,
}

pub fn spawn_weapon(weapon_type: WeaponType, scenes: &Res<Scenes>) -> SceneBundle {
    let asset_type = match weapon_type {
        WeaponType::Axe => AssetType::Axe,
        // _ => AssetType::Axe,
    };
    SceneBundle {
        scene: get_asset(scenes, asset_type),
        transform: Transform::from_xyz(0.0, 0.0, WEAPON_Z_OFFSET),
        ..Default::default()
    }
}
