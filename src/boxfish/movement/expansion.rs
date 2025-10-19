use crate::{
    boxfish::movement::{PlayerCollidedAnimation, collision::collide_at},
    prelude::*,
    stage_manager::StageInfo,
};
use bevy::prelude::*;

/// Bodyにつけられるコンポーネント
/// ついてると膨らみ中
#[derive(Component)]
pub struct Expanding {
    // BitIter
    collided_at: Option<usize>,
}

/// 伸び縮みのキー入力を受け取る
pub fn get_expand_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_query: Query<(&mut Head, &TileCoords)>,
    stage_info: Res<StageInfo>,
    body_query: Query<(&Body, &BitIter, Option<&Tail>, Entity)>,
) {
    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        for (_, tile_coords) in &head_query {
            // 尻尾含めたBodyの最大のBitIter、すなわち体の長さを取得
            let body_len = body_query.iter().map(|b| b.1.pos).max().unwrap_or(0);
            // 衝突位置を取得
            let collided_at = match collide_at(
                &tile_coords.tile_pos,
                &Travel {
                    direction: Direction::X,
                    amount: -((body_len as i32) + 1),
                },
                &stage_info.collisions,
            ) {
                Some(at) => Some((tile_coords.tile_pos - at).x as usize),
                None => None,
            };
            // BodyにExpandingコンポーネントを追加
            for (_, _, _, entity) in body_query {
                commands.entity(entity).insert(Expanding {
                    collided_at: collided_at,
                });
            }
        }
        // 頭のフラグを更新
        for (mut head, _) in &mut head_query {
            head.is_expanding = true;
        }
    }
    // Shiftが離されたらExpandingコンポーネントを削除
    if keyboard_input.just_released(KeyCode::ShiftLeft) {
        for (_, _, _, entity) in body_query {
            commands.entity(entity).remove::<Expanding>();
        }
        // 頭のフラグを更新
        for (mut head, _) in &mut head_query {
            head.is_expanding = false;
        }
    }
}

const EXPAND_SHRINK_DURATION: f32 = 0.1;

/// ハコフグくんが伸びる処理
pub fn on_expanding(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(&BitIter, &Expanding, &mut Transform, Option<&Tail>), With<Body>>,
    exp_query: Query<Entity, With<Expanding>>,
    mut head_query: Query<(&mut Head, Entity)>,
) {
    let max_iter = query.iter().map(|q| q.0.pos).max().unwrap_or(0);
    for (bit_iter, expanding, mut transform, tail) in query {
        let iter = match expanding.collided_at {
            Some(col_at) => std::cmp::min(
                match tail {
                    Some(_) => col_at - 1,
                    None => col_at - 2,
                },
                bit_iter.pos + 1,
            ),
            None => bit_iter.pos + 1,
        };
        let ideal_x = -((iter * TILE_SIZE) as f32);
        let duration = time.delta_secs() / EXPAND_SHRINK_DURATION * (TILE_SIZE as f32);
        if transform.translation.x - duration < ideal_x {
            transform.translation.x = ideal_x;
            // 壁にぶつかって伸びきったなら、伸びたのと逆方向に驚き収縮する
            if expanding.collided_at.is_some() & (bit_iter.pos == max_iter) {
                for exp_entity in exp_query {
                    commands.entity(exp_entity).remove::<Expanding>();
                }
                for (mut head, head_entity) in &mut head_query {
                    commands
                        .entity(head_entity)
                        .insert(PlayerCollidedAnimation {
                            travel: Travel {
                                direction: Direction::X,
                                amount: 1,
                            },
                            progress: 0.,
                        });
                    head.is_expanding = false;
                }
            }
        } else {
            transform.translation.x -= duration;
        }
    }
}

/// ハコフグくんが縮む処理
pub fn on_shrinking(
    time: Res<Time>,
    query: Query<(&mut Transform, Option<&Tail>), (With<Body>, Without<Expanding>)>,
) {
    for (mut transform, tail) in query {
        // 理想位置はしっぽ以外なら頭の隣
        // しっぽなら頭の二個隣
        let ideal_x = match tail {
            Some(_) => 2.,
            None => 1.,
        } * -(TILE_SIZE as f32);
        let duration = time.delta_secs() / EXPAND_SHRINK_DURATION * (TILE_SIZE as f32);
        if transform.translation.x + duration > ideal_x {
            transform.translation.x = ideal_x;
        } else {
            transform.translation.x += duration;
        }
    }
}
