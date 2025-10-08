pub mod movement;
pub mod register;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<movement::OnMoved>()
            .add_systems(Startup, aquarium_setup)
            .add_systems(Update, update_bits)
            .add_systems(Update, bit_visualise)
            .add_systems(Update, body_system)
            .add_systems(Update, face_system)
            .add_systems(Update, register::register_system)
            .add_systems(Update, movement::boxfish_moving);
    }
}

use crate::{
    Bit, TILE_SIZE, TileCoords,
    aquarium::{Collidable, ConstructAquarium},
};
use bevy::prelude::*;

// Resources
const HEAD_PATH: &str = "embedded://boxfish/head.png";
const EXPANDING_PATH: &str = "embedded://boxfish/head_expanding.png";
const SURPLIZING_PATH: &str = "embedded://boxfish/head_surplize.png";

const BODY_PATH: &str = "embedded://boxfish/body.png";
const TAIL_PATH: &str = "embedded://boxfish/tail.png";

const ZERO_PATH: &str = "embedded://boxfish/0.png";
const ONE_PATH: &str = "embedded://boxfish/1.png";

// Coefficients
const SHRINK_PER_TILE: f32 = 0.05;

#[derive(Component)]
pub enum FaceState {
    Normal,
    Expanding,
    Surplising,
}

#[derive(Component)]
pub struct Body {
    expanding: bool,
}

#[derive(Component)]
pub struct BitIter {
    pos: usize,
}

#[derive(Component)]
struct Tail;

impl Tail {
    fn get_position_on_the_length(bit_length: usize) -> Vec3 {
        Vec3::new(-(((bit_length + 1) * TILE_SIZE) as f32), 0., 0.)
    }
}

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Player;

/// ゲーム開始時に一度だけ呼び出され、プレイヤーの頭やカメラなどの
/// ゲームを通して削除されないものを配置し、最初のステージを読み込む。
fn aquarium_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<ConstructAquarium>,
) {
    commands.spawn((
        Sprite::from_image(asset_server.load(HEAD_PATH)),
        Transform::from_xyz(0., 0., 10.),
        FaceState::Normal,
        Head,
        Player,
        TileCoords {
            tile_pos: IVec2::new(0, 0),
        },
    ));
    commands.spawn(Camera2d);
    event_writer.write(ConstructAquarium::test_stage());
}

/// 新しく読み込まれたステージを適用
fn update_bits(
    mut head_query: Query<(Entity, Option<&Children>, &mut Transform, &mut TileCoords), With<Head>>,
    mut construct_aquarium: EventReader<ConstructAquarium>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
    head.2.translation = (aquarium.player_origin.as_vec2() * (TILE_SIZE as f32)).extend(10.);

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
                    Sprite::from_image(asset_server.load(BODY_PATH)),
                    Transform::from_xyz(0., 0., 10.),
                    Body { expanding: false },
                    BitIter { pos: iter },
                    Player,
                ))
                .with_child((
                    Sprite::from_image(asset_server.load(ZERO_PATH)),
                    Transform::from_xyz(0., 0., 10.),
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
        Sprite::from_image(asset_server.load(TAIL_PATH)),
        Transform::from_translation(Tail::get_position_on_the_length(body_length)),
        Body { expanding: false },
        BitIter { pos: body_length },
        Tail,
        Player,
    ));
}

/// プレイヤーのレジスタの見た目を真理値に合わせて更新
fn bit_visualise(
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

/// ハコフグくんの伸縮をコントール
fn body_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Body, &BitIter, Option<&Tail>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut body, bit_iter, tail) in &mut query {
        // 最終的にどの位置にいればいいかを計算する
        let ideal_x = if body.expanding {
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
            body.expanding = keyboard_input.pressed(KeyCode::ShiftLeft);
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

/// 表情をスプライトに反映させる
fn face_system(query: Query<(&mut Sprite, &FaceState)>, asset_server: Res<AssetServer>) {
    for (mut sprite, facestate) in query {
        sprite.image = asset_server.load(match facestate {
            &FaceState::Normal => HEAD_PATH,
            &FaceState::Expanding => EXPANDING_PATH,
            &FaceState::Surplising => SURPLIZING_PATH,
        })
    }
}
