pub mod collision;
pub mod input;

use crate::boxfish::{BoxfishRegister, PLAYER_LAYER, movement::input::player_input};
use crate::prelude::*;
use bevy::prelude::*;
pub use collision::PlayerCollidedAnimation;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnMoved>().add_systems(
            Update,
            (
                collision::collided_animation,
                collision::goal_detection_system,
                body_system,
                boxfish_moving,
                regist_movement_history,
                undo,
            ),
        );
    }
}

#[derive(Event)]
pub struct OnMoved {
    pub travel: Travel,
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
    bit_query: Query<&mut BoxfishRegister>,
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
        for mut register in bit_query {
            if let Some(last) = register.history.pop() {
                register.boolean = last;
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
        let target_pos = TileCoords::ivec2_to_vec2(tile.tile_pos);
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
