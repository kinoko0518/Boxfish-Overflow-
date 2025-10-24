pub mod construction;
pub mod movement;
pub mod register;
pub mod visual;

use crate::boxfish::movement::{MovementPlugin, PlayerCollidedAnimation};
use crate::prelude::*;
use bevy::prelude::*;
pub use visual::{BooleanImage, PlayerImage};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<register::GateCollidedAt>()
            .init_resource::<BooleanImage>()
            .init_resource::<PlayerImage>()
            .init_resource::<ResultManager>()
            .add_plugins(MovementPlugin)
            .add_systems(
                Startup,
                (visual::assets_setup, construction::aquarium_setup).chain(),
            )
            .add_systems(
                Update,
                (
                    construction::update_bits,
                    visual::face_manager,
                    register::hightlight_collided_gate,
                    register::register_system,
                    register::bit_visualise,
                ),
            );
    }
}

pub const PLAYER_LAYER: f32 = 10.;

#[derive(Component)]
pub struct Body;

#[derive(Component, Clone)]
pub struct BoxfishRegister {
    boolean: bool,
    history: Vec<bool>,
}

#[derive(Component)]
pub struct BitIter {
    pos: usize,
}

impl BitIter {
    fn get_position_on_the_length(bit_length: usize) -> Vec3 {
        Vec3::new(-(((bit_length + 1) * TILE_SIZE) as f32), 0., PLAYER_LAYER)
    }
}

#[derive(Component)]
pub struct Tail;

#[derive(Component)]
pub struct Head {
    is_expanding: bool,
    history: Vec<IVec2>,
}

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct ResultManager {
    steps: usize,
}
