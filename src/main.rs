use bevy::prelude::*;
mod boxfish;
mod tile;

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
        .add_systems(Startup, boxfish::boxfish_setup)
        .add_systems(Startup, (tile::parse_aquarium, tile::tile_adjust).chain())
        .add_systems(Update, boxfish::bit_system)
        .add_systems(Update, boxfish::body_system)
        .add_systems(Update, boxfish::face_system)
        .add_systems(Update, boxfish::boxfish_moving)
        .run();
}
