#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod boid;
mod boxfish;
mod tile;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use boxfish::PlayerPlugin;

use crate::boid::BoidPlugin;

const TILE_SIZE: usize = 16;

#[derive(Component)]
pub struct TileCoords {
    tile_pos: IVec2,
}

#[derive(Component)]
pub struct Bit {
    boolean: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(BoidPlugin)
        .add_systems(Startup, (tile::parse_aquarium, tile::tile_adjust).chain())
        .run();
}
