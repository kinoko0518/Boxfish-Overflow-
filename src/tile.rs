use crate::{Bit, TILE_SIZE, TileCoords};
use bevy::prelude::*;

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct LogiGate;

#[derive(Component)]
pub struct TileAdjust;

#[derive(Component)]
pub enum LogiKind {
    And,
    Or,
    Not,
    Xor,
}

const PARSE_TARGET: &str = "
WWWWWWWWWWWWWW
W            W
W            W
W            W
W            W
WWWWWWWWWWWWWW
";

pub fn parse_aquarium(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (y, s) in PARSE_TARGET.lines().enumerate() {
        for (x, c) in s.chars().enumerate() {
            let tile_set_image: Handle<Image> = asset_server.load("tile/logical_gates.png");
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
                                index: 16 + 1,
                            },
                        ),
                        Transform::from_xyz(0., 0., 0.),
                        coords,
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
                                index: (16 * 3) + 1,
                            },
                        ),
                        Transform::from_xyz(0., 0., 0.),
                        coords,
                        LogiKind::Not,
                        LogiGate,
                        TileAdjust,
                    ));
                }
                'W' => {
                    commands.spawn((
                        Sprite::from_image(asset_server.load("tile/wall.png")),
                        Transform::from_xyz(0., 0., 0.),
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

pub fn tile_adjust(mut query: Query<(&TileCoords, &mut Transform), With<TileAdjust>>) {
    for (t_coords, mut transform) in &mut query {
        let t_pos = t_coords.tile_pos;
        println!("{}", t_pos);
        transform.translation = Vec3::new(
            (t_pos.x * (TILE_SIZE as i32)) as f32,
            (t_pos.y * (TILE_SIZE as i32)) as f32,
            0.,
        );
    }
}
