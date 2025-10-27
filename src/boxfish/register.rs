use bevy::prelude::*;

use crate::prelude::*;
use crate::{
    aquarium::{IncorrectBit, LogiKind, LogiRegister},
    boxfish::{BooleanImage, BoxfishRegister},
};

#[derive(Event)]
pub struct GateCollidedAt {
    collided_at: IVec2,
}

/// Updating player's register visual with BoxfishRegister component's data.
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

pub fn process_gate_effect(
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
        // The coords will be moved after the process.
        // Moving process won't be executed excluding when passing gate,
        // so it's None on default.
        let mut coords_after_process: Option<IVec2> = None;
        // How much player moved
        let travel = moved.travel.clone();
        // Correcting gates' informations on useful format
        let gates: Vec<(IVec2, bool, LogiKind)> = queries
            .p1()
            .iter()
            .map(|g| (g.0.tile_pos, g.1.boolean, g.1.logikind))
            .collect();
        // The coords of head before moving
        let head_coord_before_move = head - travel.into_ivec2();

        // Processing each bit
        for (bit_iter, mut bit) in queries.p3().iter_mut() {
            process_gate_effect_for_each_bit(
                bit_iter.pos as i32,
                head_coord_before_move,
                &gates,
                &travel,
                &mut bit,
                &mut coords_after_process,
                &mut gate_collided_at_writer,
            );
        }
        // If it won't correspond to equal gate,
        // get player back to before position
        if let (Ok((mut head_mut, _)), Some(coords)) =
            (queries.p2().single_mut(), coords_after_process)
        {
            head_mut.tile_pos = coords;
        }
    }
}

/// Processing gates' effect for each bit.
///
/// And gate(&) : Appling AND operation for the bit with passed gate's register.
///
/// Or gate(|) : Appling OR operation for the bit with passed gate's register.
///
/// Xor gate(|) : Appling XOR operation for the bit with passed gate's register.
///
/// Not gate(!) : Revert the bit if passed gate's register was 1.
///
/// Undo gate(↻) : Restorate before bit pattern from history.
///
/// Equal gate(=) : Impassable when the bit and gate's register isn't same.
pub fn process_gate_effect_for_each_bit(
    bit_iter: i32,
    head_coord_before_move: IVec2,
    gates: &[(IVec2, bool, LogiKind)],
    travel: &Travel,
    bit: &mut BoxfishRegister,
    coords_after_process: &mut Option<IVec2>,
    gate_collided_at_writer: &mut EventWriter<GateCollidedAt>,
) {
    let local_x_from_head = -bit_iter - 1;
    let from = head_coord_before_move + IVec2::new(local_x_from_head, 0);
    for (gate_coords, gate_bit, logikind) in gates {
        if !collide_with(&from, &travel, gate_coords) {
            continue;
        }
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
            LogiKind::Equal => {
                if bit.boolean != *gate_bit {
                    *coords_after_process = Some(head_coord_before_move);
                    gate_collided_at_writer.write(GateCollidedAt {
                        collided_at: *gate_coords,
                    });
                }
            }
        }
    }
}

/// Highlighting incorresponded gate's bit red.
pub fn hightlight_incorresponded_gate(
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
