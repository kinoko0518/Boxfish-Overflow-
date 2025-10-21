use crate::prelude::*;
use crate::stage_manager::NextStage;
use bevy::{audio::Volume, prelude::*};
use itertools::Itertools;
use std::{f32::consts::PI, ops::Add};

use crate::aquarium::{Goal, StageCompleted};

/// 単一の対象に対して衝突判定を行う
pub fn collide_with(original: &IVec2, travel: &Travel, target: &IVec2) -> bool {
    travel.get_route(*original).contains(target)
}

#[derive(Default, Clone)]
pub struct Collision {
    collision: Vec<IVec2>,
}

impl Add for Collision {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            collision: self
                .collision
                .clone()
                .into_iter()
                .chain(rhs.collision.clone().into_iter())
                .unique()
                .collect::<Vec<IVec2>>(),
        }
    }
}

impl From<Vec<IVec2>> for Collision {
    fn from(value: Vec<IVec2>) -> Self {
        Self { collision: value }
    }
}

impl Collision {
    /// 複数の対象に対して衝突判定を行う
    pub fn do_collide(&self, original: &IVec2, travel: &Travel) -> bool {
        self.collision
            .iter()
            .any(|t| collide_with(original, travel, t))
    }
    /// 複数の対象に対し、どこで衝突するかを取得する
    pub fn collide_at(&self, original: &IVec2, travel: &Travel) -> Option<IVec2> {
        for route in travel.get_route(*original) {
            if self.collision.contains(&route) {
                return Some(route);
            }
        }
        return None;
    }
}

impl std::fmt::Display for Collision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_iter = self.collision.iter().map(|c| c.x);
        let y_iter = self.collision.iter().map(|c| c.y);

        let max = IVec2::new(
            x_iter.clone().max().unwrap_or(0),
            y_iter.clone().max().unwrap_or(0),
        );
        let min = IVec2::new(
            x_iter.clone().min().unwrap_or(0),
            y_iter.clone().min().unwrap_or(0),
        );

        let mut result = String::new();
        for y in min.y..(max.y + 1) {
            // 上下さかさまにレンダリングされないようyに変更を行う
            let y = (max.y - 1) - y;
            for x in min.x..(max.x + 1) {
                let coords = IVec2::new(x, y);
                if self.collision.contains(&coords) {
                    result.push('#');
                } else {
                    result.push(' ');
                }
                if x == max.x {
                    result.push('\n');
                }
            }
        }
        write!(f, "{}", result)
    }
}

#[derive(Component)]
pub struct PlayerCollidedAnimation {
    pub travel: Travel,
    pub progress: f32,
}

#[derive(Resource, Default)]
pub struct CollisionSoundEffect {
    collided: Handle<AudioSource>,
}

pub fn init_collision_sound_effect(
    mut sound_effect: ResMut<CollisionSoundEffect>,
    asset_server: Res<AssetServer>,
) {
    sound_effect.collided = asset_server.load("embedded://sound_effects/collided.ogg");
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
    sound_effect: Res<CollisionSoundEffect>,
) {
    if let Ok((mut transform, mut collided_anim, tcoords, entity)) = query.single_mut() {
        let halfed_travel = collided_anim.travel.into_ivec2().as_vec2() / 2.0 * (TILE_SIZE as f32);
        let anim = halfed_travel * collided_anim.progress.sin();
        transform.translation = (tcoords.into_vec2() + anim).extend(0.);

        if collided_anim.progress == 0. {
            commands.spawn((
                AudioPlayer::new(sound_effect.collided.clone()),
                PlaybackSettings {
                    volume: Volume::Linear(0.3),
                    ..default()
                },
            ));
        }
        if collided_anim.progress > PI {
            commands.entity(entity).remove::<PlayerCollidedAnimation>();
        } else {
            collided_anim.progress += 0.1;
        }
    }
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
