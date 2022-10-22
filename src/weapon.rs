use crate::config::{AXE_ASSET_PATH, WEAPON_Z_OFFSET};
use bevy::prelude::*;

pub enum WeaponType {
    Axe,
}

pub fn spawn_weapon(_weapon_type: WeaponType, ass: &Res<AssetServer>) -> SceneBundle {
    let axe = ass.load(AXE_ASSET_PATH);
    SceneBundle {
        scene: axe,
        transform: Transform::from_xyz(0.0, 0.0, WEAPON_Z_OFFSET),
        ..Default::default()
    }
}
