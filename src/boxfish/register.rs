use bevy::prelude::*;

use crate::{
    Bit, TileCoords,
    aquarium::LogiKind,
    boxfish::{
        BitIter, Head, ONE_PATH, Player, ZERO_PATH,
        movement::{OnMoved, collide_with},
    },
};

/// プレイヤーのレジスタの見た目を真理値に合わせて更新
pub fn bit_visualise(
    mut query: Query<(&mut Sprite, &Bit), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for (mut sprite, bit) in &mut query {
        if bit.boolean {
            sprite.image = asset_server.load(ONE_PATH)
        } else {
            sprite.image = asset_server.load(ZERO_PATH)
        }
    }
}

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
    for moved in on_moved.read() {
        let travel = moved.travel.clone();
        let gates: Vec<(IVec2, bool, LogiKind)> = queries
            .p0()
            .iter()
            .map(|g| (g.0.tile_pos, g.1.boolean, *g.2))
            .collect();
        let head_coord_before_move = &head.tile_pos - travel.into_ivec2();
        for (bit_iter, mut bit) in queries.p1() {
            let local_x_from_head = -(bit_iter.pos as i32) - 1;
            // 注・取得しているのは”移動後”の座標
            // 移動前の座標を逆算し、そこからルート上の座標を求める
            let from = head_coord_before_move + IVec2::new(local_x_from_head, 0);
            for (gate_coords, gate_bit, logikind) in &gates {
                if collide_with(&from, &travel, &gate_coords) {
                    match logikind {
                        LogiKind::And => bit.boolean &= *gate_bit,
                        LogiKind::Or => bit.boolean |= *gate_bit,
                        LogiKind::Not => bit.boolean = !bit.boolean,
                        LogiKind::Xor => bit.boolean ^= *gate_bit,
                        LogiKind::Gate => todo!(),
                    }
                }
            }
        }
    }
}
