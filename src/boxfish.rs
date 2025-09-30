use crate::{
    Bit, TILE_SIZE, TileCoords,
    tile::{Collidable, LogiKind},
};
use bevy::prelude::*;

const HEAD_PATH: &str = "embedded://boxfish/head.png";
const EXPANDING_PATH: &str = "embedded://boxfish/head_expanding.png";
const SURPLIZING_PATH: &str = "embedded://boxfish/head_surplize.png";

const BODY_PATH: &str = "embedded://boxfish/body.png";
const TAIL_PATH: &str = "embedded://boxfish/tail.png";

const ZERO_PATH: &str = "embedded://boxfish/0.png";
const ONE_PATH: &str = "embedded://boxfish/1.png";

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
pub struct Tail;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Player;

enum Direction {
    X,
    Y,
}

struct Travel {
    direction: Direction,
    amount: i32,
}

impl Travel {
    fn into_ivec2(&self) -> IVec2 {
        match &self.direction {
            &Direction::X => IVec2::new(self.amount, 0),
            &Direction::Y => IVec2::new(0, self.amount),
        }
    }
    fn get_route(&self, origin: IVec2) -> Vec<IVec2> {
        let sign = self.amount.signum();
        (1..((self.amount.abs() as usize) + 1))
            .map(|i| {
                let i = sign * (i as i32);
                origin
                    + match self.direction {
                        Direction::X => IVec2::new(i, 0),
                        Direction::Y => IVec2::new(0, i),
                    }
            })
            .collect::<Vec<IVec2>>()
    }
}

#[derive(Event)]
pub struct OnMoved {
    travel: Travel,
}

const SHRINK_PER_TILE: f32 = 0.05;
const SECONDS_PER_TILE: f32 = 0.2;

pub fn boxfish_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn bit_system(mut query: Query<(&mut Sprite, &Bit)>, asset_server: Res<AssetServer>) {
    for (mut sprite, bit) in &mut query {
        if bit.boolean {
            sprite.image = asset_server.load(ONE_PATH)
        } else {
            sprite.image = asset_server.load(ZERO_PATH)
        }
    }
}

pub fn body_system(
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

pub fn face_system(query: Query<(&mut Sprite, &FaceState)>, asset_server: Res<AssetServer>) {
    for (mut sprite, facestate) in query {
        sprite.image = asset_server.load(match facestate {
            &FaceState::Normal => HEAD_PATH,
            &FaceState::Expanding => EXPANDING_PATH,
            &FaceState::Surplising => SURPLIZING_PATH,
        })
    }
}

pub fn boxfish_moving(
    time: Res<Time>,
    mut queries: ParamSet<(
        Query<(&mut Transform, &mut TileCoords), With<Head>>,
        Query<&TileCoords, With<Collidable>>,
    )>,
    mut on_moved: EventWriter<OnMoved>,
    body_query: Query<(&Body, &BitIter)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let wall_positions: Vec<IVec2> = queries.p1().iter().map(|c| c.tile_pos).collect();
    let body_length: usize = body_query
        .iter()
        .map(|(body, bit_iter)| 1 + if body.expanding { bit_iter.pos } else { 1 })
        .max()
        .unwrap_or(1);
    if let Ok((mut transform, mut tile)) = queries.p0().single_mut() {
        let target_pos = Vec2::new(
            (tile.tile_pos.x * (TILE_SIZE as i32)) as f32,
            (tile.tile_pos.y * (TILE_SIZE as i32)) as f32,
        );
        let current_pos = transform.translation.xy();
        let difference = target_pos - current_pos;

        // Check as if ideal position and real position corresponding
        if difference.length() < 0.1 {
            // When corresponding, accept player input
            transform.translation.x = target_pos.x;
            transform.translation.y = target_pos.y;

            let direction = player_input(&keyboard_input);
            if direction.amount != 0 {
                let was_collided = (0..(body_length + 1)).any(|iter| {
                    do_collide(
                        &(tile.tile_pos - IVec2::new(iter as i32, 0)),
                        &direction,
                        &wall_positions,
                    )
                });
                if !was_collided {
                    tile.tile_pos += direction.into_ivec2();
                    on_moved.write(OnMoved { travel: direction });
                }
            }
        } else {
            // When not, move character to ideal position
            let move_speed = TILE_SIZE as f32 / SECONDS_PER_TILE;
            let direction_vec = difference.normalize();
            let travel_in_frame = direction_vec * move_speed * time.delta_secs();

            // Adjust when overred
            if travel_in_frame.length() >= difference.length() {
                transform.translation.x = target_pos.x;
                transform.translation.y = target_pos.y;
            } else {
                transform.translation += Vec3::new(travel_in_frame.x, travel_in_frame.y, 0.0);
            }
        }
    }
}

fn do_collide(original: &IVec2, travel: &Travel, target: &[IVec2]) -> bool {
    target.iter().any(|t| collide_with(original, travel, t))
}
fn collide_with(original: &IVec2, travel: &Travel, target: &IVec2) -> bool {
    travel.get_route(*original).contains(target)
}

fn player_input(keyboard_input: &Res<ButtonInput<KeyCode>>) -> Travel {
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        Travel {
            direction: Direction::Y,
            amount: 1,
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyS) {
        Travel {
            direction: Direction::Y,
            amount: -1,
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyA) {
        Travel {
            direction: Direction::X,
            amount: -1,
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyD) {
        Travel {
            direction: Direction::X,
            amount: 1,
        }
    } else {
        Travel {
            direction: Direction::X,
            amount: 0,
        }
    }
}
