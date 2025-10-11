use crate::{TILE_SIZE, TileCoords, stage_manager::ConstructAquarium};
use bevy::prelude::*;

pub struct AquariumPlugin;

impl Plugin for AquariumPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, highlight_incorrect_bits)
            .add_systems(Update, goal_swaying)
            .add_systems(Update, parse_aquarium);
    }
}

#[derive(Component)]
/// アクアリウム上のタイルを識別するコンポーネント
pub struct Tiles;

#[derive(Component)]
/// プレイヤーが衝突するタイルを識別するコンポーネント
pub struct Collidable;

#[derive(Component)]
/// 論理ゲートのレジスタのコンポーネント
pub struct LogiRegister {
    pub boolean: bool,
    pub logikind: LogiKind,
}

#[derive(Component, Debug)]
/// ゲートのうち、プレイヤーと接触したときに異なるビットだった、
/// すなわち条件が満たされていないビットを赤くハイライトするためのコンポーネント
/// remainingは自然に減少し、255であるときに完全に赤くなる。
pub struct IncorrectBit {
    pub remaining: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogiKind {
    And,
    Or,
    Not,
    Xor,
    Undo,
    Gate,
}

const WALL_TILESET: &str = "embedded://tile/wall.png";
const LOGIGATE_TILESET: &str = "embedded://tile/logical_gates.png";
const GOAL_TILESET: &str = "embedded://tile/goal.png";

const TILE_LAYER: f32 = 0.;

pub fn highlight_incorrect_bits(
    query: Query<(&mut Sprite, &mut IncorrectBit), With<LogiRegister>>,
) {
    for (mut sprite, mut incorrect_bit) in query {
        let not_red = 255 - incorrect_bit.remaining;
        sprite.color = Color::srgb_u8(255, not_red, not_red);
        incorrect_bit.remaining = std::cmp::max((incorrect_bit.remaining as i32) - 3, 0) as u8;
    }
}

#[derive(Component)]
pub struct Goal;

#[derive(Component)]
pub struct StageCompleted;

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
    let tile_set_image: Handle<Image> = asset_server.load(LOGIGATE_TILESET);
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 16, 16, None, None);
    let layout_handle = texture_atlas_layouts.add(tile_layout);

    for (y, s) in aquarium.lines().rev().enumerate() {
        struct State {
            bitkind: Option<LogiKind>,
            tail_found: bool,
        }
        let mut state = State {
            bitkind: None,
            tail_found: false,
        };
        for (x, c) in s.chars().enumerate() {
            let coords = (
                TileCoords {
                    tile_pos: IVec2::new(x as i32, y as i32),
                },
                Transform::from_xyz((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, TILE_LAYER),
            );
            let bit_common_components = (Tiles, IncorrectBit { remaining: 0 }, coords.clone());
            let from_index = |x: usize, y: usize| -> Sprite {
                Sprite::from_atlas_image(
                    tile_set_image.clone(),
                    TextureAtlas {
                        layout: layout_handle.clone(),
                        index: x + y * 16,
                    },
                )
            };
            let mut get_logigate =
                |tail_index: (usize, usize),
                 logikind: LogiKind|
                 -> (Sprite, ((TileCoords, bevy::prelude::Transform), Tiles)) {
                    let gate_common_components = (coords.clone(), Tiles);
                    let sprite: Sprite;
                    let do_spawn_head = if let Some(bkind) = state.bitkind {
                        bkind == logikind && state.tail_found
                    } else {
                        false
                    };
                    if do_spawn_head {
                        sprite = from_index(tail_index.0 + 1, tail_index.1);
                    } else {
                        sprite = from_index(tail_index.0, tail_index.1);
                        state.bitkind = Some(logikind);
                    }
                    state.tail_found = !do_spawn_head;
                    (sprite, gate_common_components)
                };
            const LOGIKIND_UNDEFINED: &str =
                "Parse Error: Expected a logigate's tail before any boolean";
            match c {
                'A' => {
                    commands.spawn(get_logigate((0, 1), LogiKind::And));
                }
                'O' => {
                    commands.spawn(get_logigate((0, 2), LogiKind::Or));
                }
                'N' => {
                    commands.spawn(get_logigate((0, 3), LogiKind::Not));
                }
                'X' => {
                    commands.spawn(get_logigate((0, 4), LogiKind::Xor));
                }
                'G' => {
                    commands.spawn(get_logigate((2, 0), LogiKind::Gate));
                }
                'U' => {
                    commands.spawn(get_logigate((0, 5), LogiKind::Undo));
                }
                'W' => {
                    commands.spawn((
                        Sprite::from_image(asset_server.load(WALL_TILESET)),
                        Tiles,
                        Collidable,
                        coords,
                    ));
                }
                'E' => {
                    commands.spawn((
                        Sprite::from_image(asset_server.load(GOAL_TILESET)),
                        Tiles,
                        Goal,
                        coords,
                    ));
                }
                '0' => {
                    commands.spawn((
                        from_index(1, 0),
                        LogiRegister {
                            boolean: false,
                            logikind: state.bitkind.expect(LOGIKIND_UNDEFINED),
                        },
                        bit_common_components,
                    ));
                }
                '1' => {
                    commands.spawn((
                        from_index(0, 0),
                        LogiRegister {
                            boolean: true,
                            logikind: state.bitkind.expect(LOGIKIND_UNDEFINED),
                        },
                        bit_common_components,
                    ));
                }
                _ => (),
            }
        }
    }
}

pub fn goal_swaying(query: Query<(&mut Transform, &TileCoords), With<Goal>>, time: Res<Time>) {
    for (mut transform, tile_coords) in query {
        let swayness = Vec2::new((time.elapsed_secs() * 3.).sin(), time.elapsed_secs().sin())
            * (TILE_SIZE as f32)
            / 4.;
        transform.translation = (tile_coords.into_vec2() + swayness).extend(TILE_LAYER);
    }
}
