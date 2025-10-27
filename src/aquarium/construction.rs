use super::{
    Goal, IncorrectBit, LogiKind, LogiRegister, TILE_LAYER, Tiles, resource::AquariumResource,
};
use crate::{aquarium::SemiCollidable, prelude::*};
use bevy::prelude::*;

/// 文字列を解釈してステージを組み立てる
pub fn chars_into_tiles(
    aquarium: &str,
    mut commands: Commands,
    tile_resource: Res<AquariumResource>,
) {
    let aquarium_size = UVec2::new(
        aquarium.lines().map(|l| l.len()).max().unwrap_or(0) as u32,
        aquarium.lines().collect::<Vec<&str>>().len() as u32,
    );

    // 水槽の中身を構築
    for (y, s) in aquarium.lines().rev().enumerate() {
        // ここから行それぞれについての処理

        /// LogiKindを類，真理値を真としたとき，
        ///
        /// 類真真...真真類
        ///
        /// という並びなら挟まれた真は類のLogiKindを持つ．
        /// そのためにデータを保持する構造体．
        struct State {
            bitkind: Option<LogiKind>,
            tail_found: bool,
        }
        let mut state = State {
            bitkind: None,
            tail_found: false,
        };
        // ここからタイルそれぞれについての処理
        for (x, c) in s.chars().enumerate() {
            // 大抵まとめて付与されるTileCoordsと
            // Transformを1つのタプルにまとめておく
            let coords = (
                TileCoords {
                    tile_pos: IVec2::new(x as i32, y as i32),
                },
                Transform::from_xyz((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, TILE_LAYER),
            );
            // Bitに共通のコンポーネント
            let bit_common_components = (
                Tiles,
                IncorrectBit { remaining: 0 },
                SemiCollidable,
                coords.clone(),
            );
            // 画像のインデックスからSpriteコンポーネントを生成する
            let from_index = |x: usize, y: usize| -> Sprite {
                Sprite::from_atlas_image(
                    tile_resource.tile_sprite.clone(),
                    TextureAtlas {
                        layout: tile_resource.tile_layout.clone(),
                        index: x + y * 16,
                    },
                )
            };
            // 論理ゲートを生成する処理をクロージャとして共通化する
            let mut get_logigate = |tail_index: (usize, usize),
                                    logikind: LogiKind|
             -> (
                Sprite,
                SemiCollidable,
                ((TileCoords, bevy::prelude::Transform), Tiles),
            ) {
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
                (sprite, SemiCollidable, gate_common_components)
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
                    commands.spawn(get_logigate((2, 0), LogiKind::Equal));
                }
                'U' => {
                    commands.spawn(get_logigate((0, 5), LogiKind::Undo));
                }
                'W' => {
                    commands.spawn((
                        Sprite::from_image(tile_resource.wall_sprite.clone()),
                        Tiles,
                        Collidable,
                        coords,
                    ));
                }
                'E' => {
                    commands.spawn((
                        Sprite::from_image(tile_resource.goal_sprite.clone()),
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
    // 水槽の外側を構築
    let construct_outline = |index: (usize, usize), pos: IVec2| {
        (
            Collidable,
            TileCoords::from_ivec2(pos),
            Transform::from_xyz(
                (pos.x * (TILE_SIZE as i32)) as f32,
                (pos.y * (TILE_SIZE as i32)) as f32,
                TILE_LAYER,
            ),
            Sprite::from_atlas_image(
                tile_resource.outline_sprite.clone(),
                TextureAtlas {
                    layout: tile_resource.outline_layout.clone(),
                    index: index.1 * 4 + index.0,
                },
            ),
            Tiles,
        )
    };
    let isize = IVec2::new(aquarium_size.x as i32, aquarium_size.y as i32);
    // 水槽の左上
    commands.spawn(construct_outline((0, 0), IVec2::new(-1, isize.y)));
    // 上 & 下辺
    for x in 0..isize.x {
        commands.spawn(construct_outline((1, 0), IVec2::new(x, isize.y)));
        commands.spawn(construct_outline((1, 2), IVec2::new(x, -1)));
    }
    // 右上
    commands.spawn(construct_outline((2, 0), IVec2::new(isize.x, isize.y)));
    // 右 & 左辺
    for y in 0..isize.y {
        commands.spawn(construct_outline((0, 1), IVec2::new(-1, y)));
        commands.spawn(construct_outline((2, 1), IVec2::new(isize.x, y)));
    }
    // 左下
    commands.spawn(construct_outline((0, 2), IVec2::new(-1, -1)));
    // 右下
    commands.spawn(construct_outline((2, 2), IVec2::new(isize.x, -1)));
}
