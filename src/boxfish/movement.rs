pub mod collision;
pub mod expansion;
pub mod input;

use crate::{
    boxfish::{
        BoxfishRegister, PLAYER_LAYER, ResultManager,
        movement::{collision::CollisionSoundEffect, input::player_input},
    },
    prelude::*,
    stage_manager::StageInfo,
};
use bevy::prelude::*;
pub use collision::PlayerCollidedAnimation;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CollisionSoundEffect>()
            .add_event::<OnMoved>()
            .add_systems(Startup, collision::init_collision_sound_effect)
            .add_systems(
                Update,
                (
                    expansion::on_expanding,
                    expansion::on_shrinking,
                    collision::collided_animation,
                ),
            )
            .add_systems(
                Update,
                (
                    step_counter,
                    collision::goal_detection_system,
                    expansion::get_expand_input,
                    boxfish_moving,
                    regist_movement_history,
                    undo,
                )
                    .run_if(in_state(MacroStates::GamePlay)),
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
    gamepad: Query<&Gamepad>,
) {
    let do_undo = match gamepad.single() {
        Ok(gamepad) => gamepad.just_pressed(GamepadButton::South),
        Err(_) => false,
    } | keyboard_input.pressed(KeyCode::ControlLeft)
        && keyboard_input.just_pressed(KeyCode::KeyZ);
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
    mut player_query: Query<
        (&mut Transform, &mut TileCoords, Entity, &Head),
        Without<PlayerCollidedAnimation>,
    >,
    stage_info: Res<StageInfo>,
    mut on_moved: EventWriter<OnMoved>,
    body_query: Query<&BitIter, With<Body>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepad_input: Query<&Gamepad>,
) {
    if let Ok((mut transform, mut tile, entity, head)) = player_query.single_mut() {
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

            let direction = player_input(&keyboard_input, &gamepad_input);

            if direction.amount != 0 {
                // 膨らんでいるか、縮んでいるかで衝突判定が異なるため
                let collision = if !head.is_expanding {
                    // 膨らんでいなければゲート、ビット(Semicollidable)
                    // と壁(Collidable)両方を対象にする
                    stage_info.collisions.clone() + stage_info.semicollisions.clone()
                } else {
                    // 膨らんでいれば壁(Collidable)のみを対象にする
                    stage_info.collisions.clone()
                };
                // 頭から尾まで衝突判定を行い、いずれかが衝突していれば衝突
                let was_collided = (0..(body_length + 1)).any(|iter| {
                    collision.do_collide(&(tile.tile_pos - IVec2::new(iter as i32, 0)), &direction)
                });
                if !was_collided {
                    // 衝突しなかったなら移動
                    tile.tile_pos += direction.into_ivec2();
                    on_moved.write(OnMoved { travel: direction });
                } else {
                    // 衝突したならぶつかったアニメーションを再生する
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

pub fn step_counter(mut on_moved: EventReader<OnMoved>, mut r_manager: ResMut<ResultManager>) {
    for _ in on_moved.read() {
        r_manager.steps += 1;
    }
}
