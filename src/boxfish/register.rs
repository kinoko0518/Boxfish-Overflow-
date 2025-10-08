use bevy::prelude::*;

use crate::{
    Bit, TileCoords,
    aquarium::LogiKind,
    boxfish::{
        BitIter, Head, Player,
        movement::{OnMoved, collide_with},
    },
};

pub fn register_system(
    mut on_moved: EventReader<OnMoved>,
    head_query: Query<&TileCoords, With<Head>>,
    mut queries: ParamSet<(
        Query<(&TileCoords, &Bit, &LogiKind)>,
        Query<(&BitIter, &mut Bit), With<Player>>,
    )>,
) {
    let head = if let Ok(head) = head_query.single() {
        head
    } else {
        println!("WARN: The head of the boxfish not found");
        return;
    };
    let gates: Vec<(IVec2, bool, LogiKind)> = queries
        .p0()
        .iter()
        .map(|g| (g.0.tile_pos, g.1.boolean, *g.2))
        .collect();
    if let Some(moved) = on_moved.read().next() {
        for (bit_iter, mut bit) in queries.p1() {
            for (gate_coords, gate_bit, logikind) in &gates {
                if collide_with(
                    &(&head.tile_pos + IVec2::new(-(bit_iter.pos as i32), 0)),
                    &moved.travel,
                    &gate_coords,
                ) {
                    match logikind {
                        LogiKind::And => bit.boolean &= gate_bit,
                        LogiKind::Or => bit.boolean |= gate_bit,
                        LogiKind::Not => {
                            if bit.boolean {
                                bit.boolean = !bit.boolean
                            }
                        }
                        LogiKind::Xor => {
                            bit.boolean = !bit.boolean && *gate_bit || bit.boolean && !gate_bit
                        }
                    }
                }
            }
        }
    }
}
