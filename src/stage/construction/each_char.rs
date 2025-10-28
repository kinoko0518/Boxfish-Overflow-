use super::{
    super::resource::AquariumResource, Goal, IncorrectBit, LogiKind, LogiRegister, TILE_LAYER,
    Tiles, each_line::State,
};
use crate::{prelude::*, stage::SemiCollidable};
use bevy::prelude::*;

fn generate_logical_gate(
    coords: (TileCoords, Transform),
    tail_index: (usize, usize),
    logikind: LogiKind,
    state: &mut State,
    tile_resource: &Res<AquariumResource>,
) -> (
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
        sprite = generate_tile_from_index(tail_index.0 + 1, tail_index.1, tile_resource);
    } else {
        sprite = generate_tile_from_index(tail_index.0, tail_index.1, tile_resource);
        state.bitkind = Some(logikind);
    }
    state.tail_found = !do_spawn_head;
    (sprite, SemiCollidable, gate_common_components)
}

fn generate_tile_from_index(x: usize, y: usize, tile_resource: &Res<AquariumResource>) -> Sprite {
    // 画像のインデックスからSpriteコンポーネントを生成する
    Sprite::from_atlas_image(
        tile_resource.tile_sprite.clone(),
        TextureAtlas {
            layout: tile_resource.tile_layout.clone(),
            index: x + y * 16,
        },
    )
}

/// Generate an tile from a given charactor and contexts.
pub fn interprint_each_char_as_tile(
    commands: &mut Commands,
    charactor: char,
    x: usize,
    y: usize,
    tile_resource: &Res<AquariumResource>,
    state: &mut State,
) {
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

    const LOGIKIND_UNDEFINED: &str = "Parse Error: Expected a logigate's tail before any boolean";
    match charactor {
        'A' => {
            commands.spawn(generate_logical_gate(
                coords,
                (0, 1),
                LogiKind::And,
                state,
                tile_resource,
            ));
        }
        'O' => {
            commands.spawn(generate_logical_gate(
                coords,
                (0, 2),
                LogiKind::Or,
                state,
                tile_resource,
            ));
        }
        'N' => {
            commands.spawn(generate_logical_gate(
                coords,
                (0, 3),
                LogiKind::Not,
                state,
                tile_resource,
            ));
        }
        'X' => {
            commands.spawn(generate_logical_gate(
                coords,
                (0, 4),
                LogiKind::Xor,
                state,
                tile_resource,
            ));
        }
        'G' => {
            commands.spawn(generate_logical_gate(
                coords,
                (2, 0),
                LogiKind::Equal,
                state,
                tile_resource,
            ));
        }
        'U' => {
            commands.spawn(generate_logical_gate(
                coords,
                (0, 5),
                LogiKind::Undo,
                state,
                tile_resource,
            ));
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
                generate_tile_from_index(1, 0, tile_resource),
                LogiRegister {
                    boolean: false,
                    logikind: state.bitkind.expect(LOGIKIND_UNDEFINED),
                },
                bit_common_components,
            ));
        }
        '1' => {
            commands.spawn((
                generate_tile_from_index(0, 0, tile_resource),
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
