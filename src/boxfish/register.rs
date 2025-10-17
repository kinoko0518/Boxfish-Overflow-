use bevy::prelude::*;

use crate::{
    TileCoords,
    aquarium::{IncorrectBit, LogiKind, LogiRegister},
    boxfish::{
        BitIter, BooleanImage, BoxfishRegister, Head, Player,
        movement::{OnMoved, collide_with},
    },
};

#[derive(Event)]
pub struct GateCollidedAt {
    collided_at: IVec2,
}

/// プレイヤーのレジスタの見た目を真理値に合わせて更新
pub fn bit_visualise(
    mut query: Query<(&mut Sprite, &BoxfishRegister), With<Player>>,
    boolean_image: Res<BooleanImage>,
) {
    for (mut sprite, bit) in &mut query {
        sprite.texture_atlas = Some(if bit.boolean {
            boolean_image.one()
        } else {
            boolean_image.zero()
        })
    }
}

pub fn register_system(
    mut on_moved: EventReader<OnMoved>,
    mut queries: ParamSet<(
        Query<&TileCoords, With<Player>>,
        Query<(&TileCoords, &LogiRegister)>,
        Query<(&mut TileCoords, &Head)>,
        Query<(&BitIter, &mut BoxfishRegister), With<Player>>,
    )>,
    mut gate_collided_at_writer: EventWriter<GateCollidedAt>,
) {
    let head = if let Ok(head) = queries.p0().single() {
        head.tile_pos
    } else {
        println!("WARN: The head of the boxfish not found");
        return;
    };
    for moved in on_moved.read() {
        // 処理終了後に移動する座標、ゲート以外では移動処理は行われないためNone
        let mut coords_after_process: Option<IVec2> = None;
        // 移動量
        let travel = moved.travel.clone();
        // ゲートの情報を使いやすい形で保存しておく
        let gates: Vec<(IVec2, bool, LogiKind)> = queries
            .p1()
            .iter()
            .map(|g| (g.0.tile_pos, g.1.boolean, g.1.logikind))
            .collect();
        // 移動前の頭の位置
        let head_coord_before_move = &head - travel.into_ivec2();

        for (bit_iter, mut bit) in queries.p3().iter_mut() {
            let local_x_from_head = -(bit_iter.pos as i32) - 1;
            // 注・取得しているのは”移動後”の座標
            // 移動前の座標を逆算し、そこからルート上の座標を求める
            let from = head_coord_before_move + IVec2::new(local_x_from_head, 0);
            for (gate_coords, gate_bit, logikind) in &gates {
                if collide_with(&from, &travel, &gate_coords) {
                    let now = bit.boolean;
                    match logikind {
                        LogiKind::And => {
                            bit.history.push(now);
                            bit.boolean &= *gate_bit;
                        }
                        LogiKind::Or => {
                            bit.history.push(now);
                            bit.boolean |= *gate_bit;
                        }
                        LogiKind::Not => {
                            if *gate_bit {
                                bit.history.push(now);
                                bit.boolean = !bit.boolean
                            }
                        }
                        LogiKind::Xor => {
                            bit.history.push(now);
                            bit.boolean ^= *gate_bit;
                        }
                        LogiKind::Undo => {
                            if let Some(last) = bit.history.pop() {
                                bit.boolean = last;
                            }
                        }
                        LogiKind::Gate => {
                            if bit.boolean != *gate_bit {
                                coords_after_process = Some(head_coord_before_move);
                                gate_collided_at_writer.write(GateCollidedAt {
                                    collided_at: *gate_coords,
                                });
                            }
                        }
                    }
                }
            }
        }
        // ゲートと一致しなければ通れないため元の位置に戻す
        // 一致した，しなかったに関わらず伸びていなければ弾く
        if let (Ok((mut head_mut, head)), Some(coords)) =
            (queries.p2().single_mut(), coords_after_process)
        {
            if head.is_expanding {
                head_mut.tile_pos = coords;
            }
        }
    }
}

pub fn hightlight_collided_gate(
    mut event_reader: EventReader<GateCollidedAt>,
    mut query: Query<(&mut IncorrectBit, &TileCoords)>,
) {
    for event in event_reader.read() {
        for mut target_tile in query
            .iter_mut()
            .filter(|q| q.1.tile_pos == event.collided_at)
        {
            target_tile.0.remaining = 255;
        }
    }
}
