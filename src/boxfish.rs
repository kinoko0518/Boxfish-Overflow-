pub mod movement;
pub mod register;

use crate::{
    Bit, TILE_SIZE, TileCoords,
    aquarium::{Collidable, ConstructAquarium},
    boxfish::movement::PlayerCollidedAnimation,
};
use bevy::prelude::*;

// Resources
const BOXFISH_PATH: &str = "embedded://boxfish/boxfish.png";
const BOOLEAN_PATH: &str = "embedded://boxfish/0_to_1_to_0.png";

// Coefficients
const SHRINK_PER_TILE: f32 = 0.05;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<movement::OnMoved>()
            .add_event::<register::GateCollidedAt>()
            .init_resource::<BooleanImage>()
            .init_resource::<PlayerImage>()
            .add_systems(Startup, (assets_setup, aquarium_setup).chain())
            .add_systems(
                Update,
                (
                    update_bits,
                    body_system,
                    face_manager,
                    movement::collided_animation,
                    register::hightlight_collided_gate,
                ),
            )
            .add_systems(
                Update,
                (
                    movement::boxfish_moving,
                    register::register_system,
                    register::bit_visualise,
                )
                    .chain(),
            );
    }
}

#[derive(Component)]
pub struct Body;

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
pub struct Head {
    is_expanding: bool,
}

#[derive(Component)]
pub struct Player;

#[derive(Resource, Default)]
pub struct PlayerImage {
    image: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl PlayerImage {
    fn from_index(&self, x: usize, y: usize) -> Sprite {
        Sprite::from_atlas_image(self.image.clone(), self.index_to_atlas(x, y))
    }
    fn index_to_atlas(&self, x: usize, y: usize) -> TextureAtlas {
        TextureAtlas {
            layout: self.atlas_layout.clone(),
            index: x + y * 4,
        }
    }
}

#[derive(Resource, Default)]
pub struct BooleanImage {
    image: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
}

impl BooleanImage {
    fn from_y_to_sprite(&self, y: usize) -> Sprite {
        Sprite::from_atlas_image(self.image.clone(), self.from_y_to_atlas(y))
    }
    fn from_y_to_atlas(&self, y: usize) -> TextureAtlas {
        TextureAtlas {
            layout: self.atlas_layout.clone(),
            index: y,
        }
    }
    fn zero(&self) -> TextureAtlas {
        self.from_y_to_atlas(0)
    }
    fn one(&self) -> TextureAtlas {
        self.from_y_to_atlas(10)
    }
}

pub fn assets_setup(
    mut player_image: ResMut<PlayerImage>,
    mut boolean_image: ResMut<BooleanImage>,
    asset_server: Res<AssetServer>,
) {
    player_image.image = asset_server.load(BOXFISH_PATH);
    player_image.atlas_layout = asset_server.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        4,
        4,
        None,
        None,
    ));
    boolean_image.image = asset_server.load(BOOLEAN_PATH);
    boolean_image.atlas_layout = asset_server.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        1,
        20,
        None,
        None,
    ));
}

/// ゲーム開始時に一度だけ呼び出され、プレイヤーの頭やカメラなどの
/// ゲームを通して削除されないものを配置し、最初のステージを読み込む。
fn aquarium_setup(
    mut commands: Commands,
    mut event_writer: EventWriter<ConstructAquarium>,
    player_image: Res<PlayerImage>,
) {
    commands.spawn((
        player_image.from_index(2, 0),
        Transform::from_xyz(0., 0., 10.),
        Head {
            is_expanding: false,
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
fn update_bits(
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
                    player_image.from_index(1, 0),
                    Transform::from_xyz(0., 0., 10.),
                    Body,
                    BitIter { pos: iter },
                    Player,
                ))
                .with_child((
                    boolean_image.from_y_to_sprite(0),
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
        player_image.from_index(0, 0),
        Transform::from_translation(Tail::get_position_on_the_length(body_length)),
        Body,
        BitIter { pos: body_length },
        Tail,
        Player,
    ));
}

/// ハコフグくんの伸縮をコントール
fn body_system(
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

fn face_manager(
    mut query: Query<(&mut Sprite, Option<&PlayerCollidedAnimation>, &Head)>,
    player_image: Res<PlayerImage>,
) {
    if let Ok((mut sprite, col_anim, head)) = query.single_mut() {
        sprite.texture_atlas = Some(player_image.index_to_atlas(
            2,
            match col_anim {
                Some(_) => 2,
                None => {
                    if head.is_expanding {
                        1
                    } else {
                        0
                    }
                }
            },
        ));
    }
}
