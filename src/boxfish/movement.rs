use std::f32::consts::PI;

use crate::TileCoords;
use crate::boxfish::{BitIter, Body, Collidable, Head, PLAYER_LAYER, TILE_SIZE, Tail};
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

#[derive(Component)]
pub struct PlayerCollidedAnimation {
    travel: Travel,
    progress: f32,
}

const SECONDS_PER_TILE: f32 = 0.2;

pub fn regist_movement_history(
    mut query: Query<(&mut Head, &TileCoords)>,
    mut travel: EventReader<OnMoved>,
) {
    for read in travel.read() {
        if let Ok((mut head, coords)) = query.single_mut() {
            head.history
                .push(coords.tile_pos - read.travel.into_ivec2());
        }
    }
}

pub fn undo(
    head_query: Query<(&mut TileCoords, &mut Transform, &mut Head)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let do_undo = keyboard_input
        .get_pressed()
        .any(|pressed| matches!(pressed, KeyCode::ControlLeft))
        && keyboard_input
            .get_just_pressed()
            .any(|pressed| matches!(pressed, KeyCode::KeyZ));
    if do_undo {
        for (mut t_coords, mut transform, mut head) in head_query {
            if let Some(last) = head.history.pop() {
                t_coords.tile_pos = last;
                transform.translation = TileCoords::ivec2_to_vec2(last).extend(PLAYER_LAYER);
            }
        }
    }
}

pub fn boxfish_moving(
    mut commands: Commands,
    time: Res<Time>,
    mut queries: ParamSet<(
        Query<(&mut Transform, &mut TileCoords, Entity, &Head), Without<PlayerCollidedAnimation>>,
        Query<&TileCoords, With<Collidable>>,
    )>,
    mut on_moved: EventWriter<OnMoved>,
    body_query: Query<&BitIter, With<Body>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let wall_positions: Vec<IVec2> = queries.p1().iter().map(|c| c.tile_pos).collect();
    if let Ok((mut transform, mut tile, entity, head)) = queries.p0().single_mut() {
        let body_length: usize = body_query
            .iter()
            .map(|bit_iter| 1 + if head.is_expanding { bit_iter.pos } else { 1 })
            .max()
            .unwrap_or(1);
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
                } else {
                    commands.entity(entity).insert(PlayerCollidedAnimation {
                        progress: 0.,
                        travel: direction,
                    });
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

/// プレイヤーの衝突アニメーション
pub fn collided_animation(
    mut commands: Commands,
    mut query: Query<
        (
            &mut Transform,
            &mut PlayerCollidedAnimation,
            &TileCoords,
            Entity,
        ),
        With<Head>,
    >,
) {
    if let Ok((mut transform, mut collided_anim, tcoords, entity)) = query.single_mut() {
        let halfed_travel = collided_anim.travel.into_ivec2().as_vec2() / 2.0 * (TILE_SIZE as f32);
        let anim = halfed_travel * collided_anim.progress.sin();
        transform.translation = (tcoords.into_vec2() + anim).extend(0.);

        if collided_anim.progress > PI {
            commands.entity(entity).remove::<PlayerCollidedAnimation>();
        } else {
            collided_anim.progress += 0.1;
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

// Coefficients
const SHRINK_PER_TILE: f32 = 0.05;

/// ハコフグくんの伸縮をコントール
pub fn body_system(
    time: Res<Time>,
    mut head_query: Query<&mut Head>,
    mut body_query: Query<(&mut Transform, &BitIter, Option<&Tail>), With<Body>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut head = match head_query.single_mut() {
        Ok(o) => o,
        Err(_) => panic!("Head not found"),
    };
    for (mut transform, bit_iter, tail) in &mut body_query {
        // 最終的にどの位置にいればいいかを計算する
        let ideal_x = if head.is_expanding {
            -(((bit_iter.pos + 1) * TILE_SIZE) as f32)
        } else {
            match tail {
                Some(_) => -2. * (TILE_SIZE as f32),
                None => -(TILE_SIZE as f32),
            }
        };
        // 理想の位置と今の位置の差
        let difference = transform.translation.x - ideal_x;
        if difference.abs() <= 0.01 {
            // 差が一定以下のときのみ伸縮の入力を受け付ける
            head.is_expanding = keyboard_input.pressed(KeyCode::ShiftLeft);
        } else {
            // このフレームに移動する量
            let shrink_speed = (TILE_SIZE as f32) / SHRINK_PER_TILE * time.delta_secs();
            let travel_in_frame = if difference.is_sign_positive() {
                -shrink_speed
            } else {
                shrink_speed
            };
            if difference.is_sign_positive() != (difference + travel_in_frame).is_sign_positive() {
                // 行き過ぎた場合に修正
                transform.translation.x = ideal_x;
            } else {
                // 座標に移動量を加算
                transform.translation.x += travel_in_frame;
            }
        }
    }
}
