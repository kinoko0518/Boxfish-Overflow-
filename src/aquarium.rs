use crate::{Bit, TILE_SIZE, TileCoords};
use bevy::prelude::*;

pub struct AquariumPlugin;

impl Plugin for AquariumPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConstructAquarium>()
            .add_systems(Startup, setup_aquarium)
            .add_systems(Startup, call_default_aquarium)
            .add_systems(Update, tile_adjust)
            .add_systems(Update, parse_aquarium);
    }
}

#[derive(Component)]
pub struct Tiles;

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct LogiGate;

#[derive(Component)]
pub struct TileAdjust;

#[derive(Component, Clone, Copy)]
pub enum LogiKind {
    And,
    Or,
    Not,
    Xor,
}

#[derive(Event)]
pub struct ConstructAquarium {
    pub content: String,
    pub player_origin: IVec2,
    pub player_defaultbits: Vec<bool>,
}

impl ConstructAquarium {
    pub fn test_stage() -> Self {
        Self {
            content: "
WWWWWWWWWWWW
W    WWG11GW
W          W
W  O0110O  W
W          W
W          W
WWWWWWWWWWWW
            "
            .into(),
            player_origin: IVec2::new(4, 6),
            player_defaultbits: vec![true, true],
        }
    }
}

const WALL_TILESET: &str = "embedded://tile/wall.png";
const LOGIGATE_TILESET: &str = "embedded://tile/logical_gates.png";
const AQUARIUM_PATH: &str = "embedded://background/aquarium.png";

pub fn setup_aquarium(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite {
        image: asset_server.load(AQUARIUM_PATH),
        ..default()
    });
}

pub fn call_default_aquarium(mut construct_aquarium: EventWriter<ConstructAquarium>) {
    construct_aquarium.write(ConstructAquarium::test_stage());
}

pub fn parse_aquarium(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    old_tiles: Query<Entity, With<Tiles>>,
    mut on_loaded: EventReader<ConstructAquarium>,
) {
    if let Some(aq) = on_loaded.read().next() {
        for t in old_tiles {
            commands.entity(t).despawn();
        }
        chars_into_tiles(&aq.content, commands, asset_server, texture_atlas_layouts);
    }
}

fn chars_into_tiles(
    aquarium: &str,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (y, s) in aquarium.lines().rev().enumerate() {
        for (x, c) in s.chars().enumerate() {
            let tile_set_image: Handle<Image> = asset_server.load(LOGIGATE_TILESET);
            let tile_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 16, 16, None, None);
            let layout_handle = texture_atlas_layouts.add(tile_layout);

            let coords = TileCoords {
                tile_pos: IVec2::new(x as i32, y as i32),
            };
            match c {
                ' ' => (),
                'A' => {
                    commands.spawn((
                        Sprite::from_atlas_image(
                            tile_set_image,
                            TextureAtlas {
                                layout: layout_handle,
                                index: 16,
                            },
                        ),
                        Transform::from_xyz(0., 0., 0.),
                        coords,
                        Tiles,
                        LogiKind::And,
                        LogiGate,
                        TileAdjust,
                    ));
                }
                'N' => {
                    commands.spawn((
                        Sprite::from_atlas_image(
                            tile_set_image,
                            TextureAtlas {
                                layout: layout_handle,
                                index: 16 * 3,
                            },
                        ),
                        Transform::from_xyz(0., 0., 0.),
                        coords,
                        Tiles,
                        LogiKind::Not,
                        LogiGate,
                        TileAdjust,
                    ));
                }
                'W' => {
                    commands.spawn((
                        Sprite::from_image(asset_server.load(WALL_TILESET)),
                        Transform::from_xyz(0., 0., 0.),
                        Tiles,
                        Collidable,
                        TileAdjust,
                        coords,
                    ));
                }
                '0' => {
                    commands.spawn((
                        Sprite::from_atlas_image(
                            tile_set_image,
                            TextureAtlas {
                                layout: layout_handle,
                                index: 0,
                            },
                        ),
                        Transform::from_xyz(0., 0., 0.),
                        Bit { boolean: false },
                        Tiles,
                        LogiGate,
                        TileAdjust,
                        coords,
                    ));
                }
                '1' => {
                    commands.spawn((
                        Sprite::from_atlas_image(
                            tile_set_image,
                            TextureAtlas {
                                layout: layout_handle,
                                index: 1,
                            },
                        ),
                        Transform::from_xyz(0., 0., 0.),
                        Bit { boolean: true },
                        Tiles,
                        LogiGate,
                        TileAdjust,
                        coords,
                    ));
                }
                _ => (),
            }
        }
    }
}

pub fn tile_adjust(mut query: Query<(&TileCoords, &mut Transform), Changed<TileAdjust>>) {
    for (t_coords, mut transform) in &mut query {
        let t_pos = t_coords.tile_pos;
        transform.translation = Vec3::new(
            (t_pos.x * (TILE_SIZE as i32)) as f32,
            (t_pos.y * (TILE_SIZE as i32)) as f32,
            0.,
        );
    }
}
