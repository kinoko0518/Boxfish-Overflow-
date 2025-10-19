use crate::prelude::*;
use bevy::prelude::*;

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
