pub mod movement;
pub mod register;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<movement::OnMoved>()
            .add_systems(Startup, boxfish_setup)
            .add_systems(Update, bit_visualise)
            .add_systems(Update, body_system)
            .add_systems(Update, face_system)
            .add_systems(Update, register::register_system)
            .add_systems(Update, movement::boxfish_moving);
    }
}

use crate::{Bit, TILE_SIZE, TileCoords, tile::Collidable};
use bevy::prelude::*;

// Resources
const HEAD_PATH: &str = "embedded://boxfish/head.png";
const EXPANDING_PATH: &str = "embedded://boxfish/head_expanding.png";
const SURPLIZING_PATH: &str = "embedded://boxfish/head_surplize.png";

const BODY_PATH: &str = "embedded://boxfish/body.png";
const TAIL_PATH: &str = "embedded://boxfish/tail.png";

const ZERO_PATH: &str = "embedded://boxfish/0.png";
const ONE_PATH: &str = "embedded://boxfish/1.png";

// Coefficients
const SHRINK_PER_TILE: f32 = 0.05;

#[derive(Component)]
pub enum FaceState {
    Normal,
    Expanding,
    Surplising,
}

#[derive(Component)]
pub struct Body {
    expanding: bool,
}

#[derive(Component)]
pub struct BitIter {
    pos: usize,
}

#[derive(Component)]
struct Tail;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Player;

fn boxfish_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Sprite::from_image(asset_server.load(HEAD_PATH)),
            Transform::from_xyz(0., 0., 0.),
            FaceState::Normal,
            Head,
            Player,
            TileCoords {
                tile_pos: IVec2::new(0, 0),
            },
        ))
        .with_children(|parent| {
            const BIT_LENGTH: usize = 4;
            for iter in 0..BIT_LENGTH {
                parent
                    .spawn((
                        Sprite::from_image(asset_server.load(BODY_PATH)),
                        Transform::from_xyz(0., 0., 0.),
                        Body { expanding: false },
                        BitIter { pos: iter },
                        Player,
                    ))
                    .with_child((
                        Sprite::from_image(asset_server.load(ZERO_PATH)),
                        Transform::from_xyz(0., 0., 0.),
                        Bit { boolean: false },
                        BitIter { pos: iter },
                        Player,
                    ));
            }
            parent.spawn((
                Sprite::from_image(asset_server.load(TAIL_PATH)),
                Transform::from_xyz(-(((BIT_LENGTH + 1) * TILE_SIZE) as f32), 0., 0.),
                Body { expanding: false },
                BitIter { pos: BIT_LENGTH },
                Tail,
                Player,
            ));
        });
    commands.spawn(Camera2d);
}

fn bit_visualise(
    mut query: Query<(&mut Sprite, &Bit), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, bit) in &mut query {
        if bit.boolean {
            sprite.image = asset_server.load(ONE_PATH)
        } else {
            sprite.image = asset_server.load(ZERO_PATH)
        }
    }
}

fn body_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Body, &BitIter, Option<&Tail>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut body, bit_iter, tail) in &mut query {
        let ideal_x = if body.expanding {
            -(((bit_iter.pos + 1) * TILE_SIZE) as f32)
        } else {
            match tail {
                Some(_) => -2. * (TILE_SIZE as f32),
                None => -(TILE_SIZE as f32),
            }
        };
        let difference = transform.translation.x - ideal_x;
        if difference.abs() <= 0.01 {
            body.expanding = keyboard_input.pressed(KeyCode::ShiftLeft);
        } else {
            let shrink_speed = (TILE_SIZE as f32) / SHRINK_PER_TILE * time.delta_secs();
            let travel_in_frame = if difference.is_sign_positive() {
                -shrink_speed
            } else {
                shrink_speed
            };
            if difference.is_sign_positive() != (difference + travel_in_frame).is_sign_positive() {
                transform.translation.x = ideal_x;
            } else {
                transform.translation.x += travel_in_frame;
            }
        }
    }
}

fn face_system(query: Query<(&mut Sprite, &FaceState)>, asset_server: Res<AssetServer>) {
    for (mut sprite, facestate) in query {
        sprite.image = asset_server.load(match facestate {
            &FaceState::Normal => HEAD_PATH,
            &FaceState::Expanding => EXPANDING_PATH,
            &FaceState::Surplising => SURPLIZING_PATH,
        })
    }
}
