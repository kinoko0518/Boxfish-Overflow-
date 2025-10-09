use crate::boxfish::{BitIter, Body, Collidable, Head, TILE_SIZE, TileCoords};
use bevy::prelude::*;

#[derive(Clone)]
pub struct Travel {
    direction: Direction,
    amount: i32,
}

#[derive(Clone)]
enum Direction {
    X,
    Y,
}

impl Travel {
    pub fn into_ivec2(&self) -> IVec2 {
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
    pub travel: Travel,
}

const SECONDS_PER_TILE: f32 = 0.2;

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

/// 単一の対象に対して衝突判定を行う
pub fn collide_with(original: &IVec2, travel: &Travel, target: &IVec2) -> bool {
    travel.get_route(*original).contains(target)
}
/// 複数の対象に対して衝突判定を行う
pub fn do_collide(original: &IVec2, travel: &Travel, target: &[IVec2]) -> bool {
    target.iter().any(|t| collide_with(original, travel, t))
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
