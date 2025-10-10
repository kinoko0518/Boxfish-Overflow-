pub mod construction;
pub mod movement;
pub mod register;
pub mod visual;

use crate::{TILE_SIZE, aquarium::Collidable, boxfish::movement::PlayerCollidedAnimation};
use bevy::prelude::*;
pub use visual::{BooleanImage, PlayerImage};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<movement::OnMoved>()
            .add_event::<register::GateCollidedAt>()
            .init_resource::<BooleanImage>()
            .init_resource::<PlayerImage>()
            .add_systems(
                Startup,
                (visual::assets_setup, construction::aquarium_setup).chain(),
            )
            .add_systems(
                Update,
                (
                    construction::update_bits,
                    movement::body_system,
                    visual::face_manager,
                    movement::collided_animation,
                    register::hightlight_collided_gate,
                ),
            )
            .add_systems(
                Update,
                (
                    movement::boxfish_moving,
                    register::register_system,
                    register::bit_visualise,
                )
                    .chain(),
            );
    }
}

const PLAYER_LAYER: f32 = 10.;

#[derive(Component)]
pub struct Body;

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
}

#[derive(Component)]
pub struct Player;
