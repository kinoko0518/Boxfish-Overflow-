use super::{
    super::resource::AquariumResource, Goal, IncorrectBit, LogiKind, LogiRegister, TILE_LAYER,
    Tiles, each_line::State,
};
use crate::{prelude::*, stage::SemiCollidable};
use bevy::prelude::*;

/// This is a support function to generating
/// logical gate with minimal boiler plate.
fn generate_logical_gate(
    coords: (TileCoords, Transform),
    tilemap_index: (usize, usize),
    logikind: LogiKind,
    state: &mut State,
    tile_resource: &Res<AquariumResource>,
) -> (Sprite, SemiCollidable, TileCoords, Transform, Tiles) {
    let gate_common_components = (coords.clone(), Tiles);
    let sprite: Sprite;
    let do_spawn_head = if let Some(bkind) = state.bitkind {
        bkind == logikind && state.tail_found
    } else {
        false
    };
    if do_spawn_head {
        sprite = generate_tile_from_index(tilemap_index.0 + 1, tilemap_index.1, tile_resource);
    } else {
        sprite = generate_tile_from_index(tilemap_index.0, tilemap_index.1, tile_resource);
        state.bitkind = Some(logikind);
    }
    state.tail_found = !do_spawn_head;
    let ((tile_coords, transform), tile) = gate_common_components;
    (sprite, SemiCollidable, tile_coords, transform, tile)
}

fn generate_tile_from_index(x: usize, y: usize, tile_resource: &Res<AquariumResource>) -> Sprite {
    // Generating a Sprite component from the tilemap index
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
    // Combining TileCoords and Transform for simplicity
    let coords = (
        TileCoords {
            tile_pos: IVec2::new(x as i32, y as i32),
        },
        Transform::from_xyz((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, TILE_LAYER),
    );
    // Common components for a Bit
    let bit_common_components = (
        Tiles,
        IncorrectBit { remaining: 0 },
        SemiCollidable,
        coords.clone(),
    );

    const LOGIKIND_UNDEFINED_MESSAGE: &str =
        "Parse Error: Expected a logigate's tail before any boolean";
    let logigate = match charactor {
        'A' => Some(((0, 1), LogiKind::And)),
        'O' => Some(((0, 2), LogiKind::Or)),
        'N' => Some(((0, 3), LogiKind::Not)),
        'X' => Some(((0, 4), LogiKind::Xor)),
        'G' => Some(((2, 0), LogiKind::Equal)),
        'U' => Some(((0, 5), LogiKind::Undo)),
        _ => None,
    };
    if let Some((index, logikind)) = logigate {
        commands.spawn(generate_logical_gate(
            coords,
            index,
            logikind,
            state,
            tile_resource,
        ));
        return;
    }

    let boolean = match charactor {
        '0' => Some(((1, 0), false)),
        '1' => Some(((0, 0), true)),
        _ => None,
    };
    if let Some((index, boolean)) = boolean {
        commands.spawn((
            generate_tile_from_index(index.0, index.1, tile_resource),
            LogiRegister {
                boolean: boolean,
                logikind: state.bitkind.expect(LOGIKIND_UNDEFINED_MESSAGE),
            },
            bit_common_components,
        ));
        return;
    }

    match charactor {
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
        _ => (),
    }
}
