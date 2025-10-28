mod each_char;
mod each_line;

use super::{
    Goal, IncorrectBit, LogiKind, LogiRegister, TILE_LAYER, Tiles, resource::AquariumResource,
};
use crate::prelude::*;
use bevy::prelude::*;

/// Constructing a stage with interprinting given string.
pub fn chars_into_tiles(
    aquarium: &str,
    mut commands: Commands,
    tile_resource: Res<AquariumResource>,
) {
    let aquarium_size = UVec2::new(
        aquarium.lines().map(|l| l.len()).max().unwrap_or(0) as u32,
        aquarium.lines().collect::<Vec<&str>>().len() as u32,
    );

    // Construct stages' inside
    for (y, s) in aquarium.lines().rev().enumerate() {
        each_line::interprint_each_line_as_tile(&mut commands, s, y, &tile_resource);
    }
    // Construct stages' outline
    construct_stage_outline(&mut commands, &tile_resource, aquarium_size);
}

/// Construct stages' outline.
pub fn construct_stage_outline(
    commands: &mut Commands,
    tile_resource: &Res<AquariumResource>,
    aquarium_size: UVec2,
) {
    let generate_a_outlines_tile = |index: (usize, usize), pos: IVec2| {
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
    // A upper left
    commands.spawn(generate_a_outlines_tile((0, 0), IVec2::new(-1, isize.y)));
    // Upper and downer sides
    for x in 0..isize.x {
        commands.spawn(generate_a_outlines_tile((1, 0), IVec2::new(x, isize.y)));
        commands.spawn(generate_a_outlines_tile((1, 2), IVec2::new(x, -1)));
    }
    // A upper right
    commands.spawn(generate_a_outlines_tile(
        (2, 0),
        IVec2::new(isize.x, isize.y),
    ));
    // Right and left sides
    for y in 0..isize.y {
        commands.spawn(generate_a_outlines_tile((0, 1), IVec2::new(-1, y)));
        commands.spawn(generate_a_outlines_tile((2, 1), IVec2::new(isize.x, y)));
    }
    // A downer left
    commands.spawn(generate_a_outlines_tile((0, 2), IVec2::new(-1, -1)));
    // A downer right
    commands.spawn(generate_a_outlines_tile((2, 2), IVec2::new(isize.x, -1)));
}
