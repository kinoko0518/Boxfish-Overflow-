use crate::{boxfish::movement::collision::collide_at, prelude::*, stage_manager::StageInfo};
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
    head_query: Query<(&mut Head, &TileCoords)>,
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
                    amount: -(body_len as i32),
                },
                &stage_info.collisions,
            ) {
                Some(at) => Some((tile_coords.tile_pos - at).x as usize),
                None => None,
            };
            // BodyにExpandingコンポーネントを追加
            for (_, _, tail, entity) in body_query {
                commands.entity(entity).insert(Expanding {
                    collided_at: match collided_at {
                        Some(at) => match tail {
                            Some(_) => Some(at - 1),
                            None => Some(at - 2),
                        },
                        None => collided_at,
                    },
                });
            }
        }
    }
    // Shiftが離されたらExpandingコンポーネントを削除
    if keyboard_input.just_released(KeyCode::ShiftLeft) {
        for (_, _, _, entity) in body_query {
            commands.entity(entity).remove::<Expanding>();
        }
    }
    // 頭のフラグを更新
    for (mut head, _) in head_query {
        head.is_expanding = keyboard_input.pressed(KeyCode::ShiftLeft);
    }
}

const EXPAND_SHRINK_DURATION: f32 = 0.1;

/// ハコフグくんが伸びる処理
pub fn on_expanding(
    time: Res<Time>,
    query: Query<(&BitIter, &Expanding, &mut Transform), With<Body>>,
) {
    for (bit_iter, expanding, mut transform) in query {
        let iter = match expanding.collided_at {
            Some(col_at) => std::cmp::min(col_at, bit_iter.pos + 1),
            None => bit_iter.pos + 1,
        };
        let ideal_x = -((iter * TILE_SIZE) as f32);
        let duration = time.delta_secs() / EXPAND_SHRINK_DURATION * (TILE_SIZE as f32);
        if transform.translation.x - duration < ideal_x {
            transform.translation.x = ideal_x;
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
