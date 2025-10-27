pub use crate::{
    aquarium::Collidable,
    boxfish::{
        BitIter, Body, BoxfishRegister, Head, PLAYER_LAYER, Player, Tail,
        movement::{
            OnMoved,
            collision::{Collision, collide_with},
            input::{Direction, Travel},
        },
    },
    stage_manager::{ConstructAquarium, NewGame, StageManager},
};
use bevy::prelude::*;

/// The side length of a single square tile in pixels.
///
/// (This means one tile is 16x16 pixels).
///
/// Used to convert from tile coordinates (e.g., grid (2, 3))
/// to Bevy's world coordinates (e.g., pixel (32.0, 48.0)).
pub const TILE_SIZE: usize = 16;

#[derive(Component, Clone)]
/// Semantic coords for the systems.
///
/// This component itself does nothing,
/// but be used with other components
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

#[derive(States, Default, Debug, Clone, Hash, PartialEq, Eq)]
#[states(scoped_entities)]
/// This is the biggest states for the game.
pub enum MacroStates {
    #[default]
    /// In this state, camera zooming out,
    /// and showing main menu on the left of a screen.
    ///
    /// The ESCMenu state is the start screen
    /// of the game at the same time.
    ESCMenu,
    /// In this state, player can operate the boxfish.
    GamePlay,
    /// In this state, a result will be shown.
    GameClear,
}
