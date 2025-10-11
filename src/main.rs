#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod aquarium;
mod boid;
mod boxfish;
mod stage_manager;
mod styling;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use boxfish::PlayerPlugin;

use crate::{
    aquarium::AquariumPlugin, boid::BoidPlugin, stage_manager::StageManagerPlugin,
    styling::StylingPlugin,
};

const TILE_SIZE: usize = 16;

#[derive(Component, Clone)]
pub struct TileCoords {
    tile_pos: IVec2,
}

impl TileCoords {
    pub fn into_vec2(&self) -> Vec2 {
        Self::ivec2_to_vec2(self.tile_pos)
    }
    pub fn from_ivec2(ivec2: IVec2) -> Self {
        Self { tile_pos: ivec2 }
    }
    pub fn ivec2_to_vec2(ivec2: IVec2) -> Vec2 {
        (ivec2 * (TILE_SIZE as i32)).as_vec2()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(StageManagerPlugin)
        .add_plugins(StylingPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BoidPlugin)
        .add_plugins(AquariumPlugin)
        .run();
}
