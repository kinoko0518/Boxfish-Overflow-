pub use crate::{
    aquarium::Collidable,
    boxfish::{
        BitIter, Body, BoxfishRegister, Head, PLAYER_LAYER, Player, Tail,
        movement::{
            OnMoved,
            collision::{collide_with, do_collide},
            input::{Direction, Travel},
        },
    },
    stage_manager::{ConstructAquarium, StageManager},
};
use bevy::prelude::*;

pub const TILE_SIZE: usize = 16;

#[derive(Component, Clone)]
pub struct TileCoords {
    pub tile_pos: IVec2,
}

impl TileCoords {
    pub fn into_vec2(&self) -> Vec2 {
        Self::ivec2_to_vec2(self.tile_pos)
    }
    pub fn from_ivec2(ivec2: IVec2) -> Self {
        Self { tile_pos: ivec2 }
    }
    pub fn ivec2_to_vec2(ivec2: IVec2) -> Vec2 {
        (ivec2 * (TILE_SIZE as i32)).as_vec2()
    }
}
