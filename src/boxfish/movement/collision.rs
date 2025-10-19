use crate::prelude::*;
use crate::stage_manager::NextStage;
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::aquarium::{Goal, StageCompleted};

#[derive(Component)]
pub struct PlayerCollidedAnimation {
    pub travel: Travel,
    pub progress: f32,
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
/// 複数の対象に対し、どこで衝突するかを取得する
pub fn collide_at(original: &IVec2, travel: &Travel, target: &[IVec2]) -> Option<IVec2> {
    for route in travel.get_route(*original) {
        if target.contains(&route) {
            return Some(route);
        }
    }
    return None;
}

pub fn goal_detection_system(
    mut commands: Commands,
    head_query: Query<(&Head, &TileCoords)>,
    bits: Query<&BitIter>,
    goals: Query<(&Goal, &TileCoords, Entity), Without<StageCompleted>>,
    mut next_stage: EventWriter<NextStage>,
) {
    let (head, tile_coords) = if let Ok((head, tile_coords)) = head_query.single() {
        (head, tile_coords)
    } else {
        return;
    };
    let bit_iters: Vec<usize> = if head.is_expanding {
        bits.iter().map(|bit| bit.pos).collect()
    } else {
        (0..2).collect()
    };
    let player_coods = bit_iters
        .iter()
        .map(|i| tile_coords.tile_pos - IVec2::new(*i as i32, 0))
        .collect::<Vec<IVec2>>();
    for (_, pos, entity) in goals {
        if player_coods.contains(&pos.tile_pos) {
            commands.entity(entity).insert(StageCompleted);
            next_stage.write(NextStage);
        }
    }
}
