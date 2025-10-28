use super::{Goal, IncorrectBit, LogiRegister, TILE_LAYER};
use crate::prelude::*;
use bevy::prelude::*;

pub fn highlight_incorrect_bits(
    query: Query<(&mut Sprite, &mut IncorrectBit), With<LogiRegister>>,
) {
    for (mut sprite, mut incorrect_bit) in query {
        let not_red = 255 - incorrect_bit.remaining;
        sprite.color = Color::srgb_u8(255, not_red, not_red);
        incorrect_bit.remaining = std::cmp::max((incorrect_bit.remaining as i32) - 3, 0) as u8;
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
