use crate::{
    Bit, TILE_SIZE, TileCoords,
    aquarium::ConstructAquarium,
    boxfish::{BitIter, Body, BooleanImage, Head, PLAYER_LAYER, Player, Tail, visual::PlayerImage},
};
use bevy::prelude::*;

/// ゲーム開始時に一度だけ呼び出され、プレイヤーの頭やカメラなどの
/// ゲームを通して削除されないものを配置し、最初のステージを読み込む。
pub fn aquarium_setup(
    mut commands: Commands,
    mut event_writer: EventWriter<ConstructAquarium>,
    player_image: Res<PlayerImage>,
) {
    commands.spawn((
        player_image.from_index(2, 0),
        Transform::from_xyz(0., 0., PLAYER_LAYER),
        Head {
            is_expanding: false,
            history: Vec::new(),
        },
        Player,
        TileCoords {
            tile_pos: IVec2::new(0, 0),
        },
    ));
    commands.spawn(Camera2d);
    event_writer.write(ConstructAquarium::test_stage());
}

/// 新しく読み込まれたステージを適用
pub fn update_bits(
    mut head_query: Query<(Entity, Option<&Children>, &mut Transform, &mut TileCoords), With<Head>>,
    mut construct_aquarium: EventReader<ConstructAquarium>,
    mut commands: Commands,
    player_image: Res<PlayerImage>,
    boolean_image: Res<BooleanImage>,
) {
    // ステージを読み込み
    let aquarium = match construct_aquarium.read().next() {
        Some(aq) => aq,
        None => return,
    };
    // ハコフグくんの頭を取得
    let mut head = match head_query.single_mut() {
        Ok(h) => h,
        Err(_) => panic!("The head of the boxfish not found"),
    };
    // 座標を更新
    head.3.tile_pos = aquarium.player_origin;
    head.2.translation =
        (aquarium.player_origin.as_vec2() * (TILE_SIZE as f32)).extend(PLAYER_LAYER);

    // 古いビットとしっぽを削除
    if let Some(children) = head.1 {
        for child in children {
            commands.entity(*child).despawn();
        }
    }
    // 新しいビットを生成、idを取得してVec<Entity>にする
    let bits = {
        let mut ids = vec![];
        for (iter, bit) in aquarium.player_defaultbits.iter().enumerate() {
            let id = commands
                .spawn((
                    player_image.from_index(1, 0),
                    Transform::from_xyz(0., 0., PLAYER_LAYER),
                    Body,
                    BitIter { pos: iter },
                    Player,
                ))
                .with_child((
                    boolean_image.from_y_to_sprite(0),
                    Transform::from_xyz(0., 0., PLAYER_LAYER),
                    Bit { boolean: *bit },
                    BitIter { pos: iter },
                    Player,
                ))
                .id();
            ids.push(id);
        }
        ids
    };
    // 生成されたビットのIDをもとにハコフグの頭の子にする
    let mut head_command = commands.entity(head.0);
    head_command.add_children(&bits);

    // しっぽを追加
    let body_length = aquarium.player_defaultbits.len();
    head_command.with_child((
        player_image.from_index(0, 0),
        Transform::from_translation(BitIter::get_position_on_the_length(body_length)),
        Body,
        BitIter { pos: body_length },
        Tail,
        Player,
    ));
}
