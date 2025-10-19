use crate::prelude::*;
use bevy::prelude::*;

pub struct AquariumPlugin;

impl Plugin for AquariumPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConstructionCompleted>()
            .init_resource::<AquariumResource>()
            .add_systems(Startup, init_aquarium_resource)
            .add_systems(Update, highlight_incorrect_bits)
            .add_systems(Update, goal_swaying)
            .add_systems(Update, parse_aquarium);
    }
}

#[derive(Component)]
/// アクアリウム上のタイルを識別するコンポーネント
/// このコンポーネントが付与されていれば新しいシーンが読み込まれたときに消滅する
pub struct Tiles;

#[derive(Component)]
/// プレイヤーが衝突するタイルを識別するコンポーネント
pub struct Collidable;

#[derive(Component)]
/// 論理ゲートのレジスタのコンポーネント
pub struct LogiRegister {
    pub boolean: bool,
    pub logikind: LogiKind,
}

#[derive(Component, Debug)]
/// ゲートのうち、プレイヤーと接触したときに異なるビットだった、
/// すなわち条件が満たされていないビットを赤くハイライトするためのコンポーネント
/// remainingは自然に減少し、255であるときに完全に赤くなる。
pub struct IncorrectBit {
    pub remaining: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogiKind {
    And,
    Or,
    Not,
    Xor,
    Undo,
    Gate,
}

const LOGIGATE_TILESET: &str = "embedded://tile/logical_gates.png";
const OUTLINE_TILESET: &str = "embedded://tile/aquarium.png";
const WALL_SPRITE: &str = "embedded://tile/wall.png";
const GOAL_SPRITE: &str = "embedded://tile/goal.png";

const TILE_LAYER: f32 = 0.;

pub fn highlight_incorrect_bits(
    query: Query<(&mut Sprite, &mut IncorrectBit), With<LogiRegister>>,
) {
    for (mut sprite, mut incorrect_bit) in query {
        let not_red = 255 - incorrect_bit.remaining;
        sprite.color = Color::srgb_u8(255, not_red, not_red);
        incorrect_bit.remaining = std::cmp::max((incorrect_bit.remaining as i32) - 3, 0) as u8;
    }
}

#[derive(Component)]
pub struct Goal;

#[derive(Component)]
pub struct StageCompleted;

#[derive(Event)]
pub struct ConstructionCompleted;

#[derive(Resource, Default)]
pub struct AquariumResource {
    tile_sprite: Handle<Image>,
    tile_layout: Handle<TextureAtlasLayout>,
    outline_sprite: Handle<Image>,
    outline_layout: Handle<TextureAtlasLayout>,
    wall_sprite: Handle<Image>,
    goal_sprite: Handle<Image>,
}

pub fn init_aquarium_resource(
    mut resource: ResMut<AquariumResource>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    resource.tile_sprite = asset_server.load(LOGIGATE_TILESET);
    resource.tile_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        16,
        16,
        None,
        None,
    ));
    resource.outline_sprite = asset_server.load(OUTLINE_TILESET);
    resource.outline_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        4,
        4,
        None,
        None,
    ));
    resource.wall_sprite = asset_server.load(WALL_SPRITE);
    resource.goal_sprite = asset_server.load(GOAL_SPRITE);
}

pub fn parse_aquarium(
    mut commands: Commands,
    tile_resource: Res<AquariumResource>,
    old_tiles: Query<Entity, With<Tiles>>,
    mut on_loaded: EventReader<ConstructAquarium>,
    mut construction_completed: EventWriter<ConstructionCompleted>,
) {
    if let Some(aq) = on_loaded.read().next() {
        for t in old_tiles {
            commands.entity(t).despawn();
        }
        chars_into_tiles(&aq.content, commands, tile_resource);
        construction_completed.write(ConstructionCompleted);
    }
}

fn chars_into_tiles(aquarium: &str, mut commands: Commands, tile_resource: Res<AquariumResource>) {
    let aquarium_size = UVec2::new(
        aquarium.lines().map(|l| l.len()).max().unwrap_or(0) as u32,
        aquarium.lines().collect::<Vec<&str>>().len() as u32,
    );

    // 水槽の中身を構築
    for (y, s) in aquarium.lines().rev().enumerate() {
        struct State {
            bitkind: Option<LogiKind>,
            tail_found: bool,
        }
        let mut state = State {
            bitkind: None,
            tail_found: false,
        };
        for (x, c) in s.chars().enumerate() {
            let coords = (
                TileCoords {
                    tile_pos: IVec2::new(x as i32, y as i32),
                },
                Transform::from_xyz((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, TILE_LAYER),
            );
            let bit_common_components = (Tiles, IncorrectBit { remaining: 0 }, coords.clone());
            let from_index = |x: usize, y: usize| -> Sprite {
                Sprite::from_atlas_image(
                    tile_resource.tile_sprite.clone(),
                    TextureAtlas {
                        layout: tile_resource.tile_layout.clone(),
                        index: x + y * 16,
                    },
                )
            };
            let mut get_logigate =
                |tail_index: (usize, usize),
                 logikind: LogiKind|
                 -> (Sprite, ((TileCoords, bevy::prelude::Transform), Tiles)) {
                    let gate_common_components = (coords.clone(), Tiles);
                    let sprite: Sprite;
                    let do_spawn_head = if let Some(bkind) = state.bitkind {
                        bkind == logikind && state.tail_found
                    } else {
                        false
                    };
                    if do_spawn_head {
                        sprite = from_index(tail_index.0 + 1, tail_index.1);
                    } else {
                        sprite = from_index(tail_index.0, tail_index.1);
                        state.bitkind = Some(logikind);
                    }
                    state.tail_found = !do_spawn_head;
                    (sprite, gate_common_components)
                };
            const LOGIKIND_UNDEFINED: &str =
                "Parse Error: Expected a logigate's tail before any boolean";
            match c {
                'A' => {
                    commands.spawn(get_logigate((0, 1), LogiKind::And));
                }
                'O' => {
                    commands.spawn(get_logigate((0, 2), LogiKind::Or));
                }
                'N' => {
                    commands.spawn(get_logigate((0, 3), LogiKind::Not));
                }
                'X' => {
                    commands.spawn(get_logigate((0, 4), LogiKind::Xor));
                }
                'G' => {
                    commands.spawn(get_logigate((2, 0), LogiKind::Gate));
                }
                'U' => {
                    commands.spawn(get_logigate((0, 5), LogiKind::Undo));
                }
                'W' => {
                    commands.spawn((
                        Sprite::from_image(tile_resource.wall_sprite.clone()),
                        Tiles,
                        Collidable,
                        coords,
                    ));
                }
                'E' => {
                    commands.spawn((
                        Sprite::from_image(tile_resource.goal_sprite.clone()),
                        Tiles,
                        Goal,
                        coords,
                    ));
                }
                '0' => {
                    commands.spawn((
                        from_index(1, 0),
                        LogiRegister {
                            boolean: false,
                            logikind: state.bitkind.expect(LOGIKIND_UNDEFINED),
                        },
                        bit_common_components,
                    ));
                }
                '1' => {
                    commands.spawn((
                        from_index(0, 0),
                        LogiRegister {
                            boolean: true,
                            logikind: state.bitkind.expect(LOGIKIND_UNDEFINED),
                        },
                        bit_common_components,
                    ));
                }
                _ => (),
            }
        }
    }
    // 水槽の外側を構築
    let construct_outline = |index: (usize, usize), pos: IVec2| {
        (
            Collidable,
            TileCoords::from_ivec2(pos),
            Transform::from_xyz(
                (pos.x * (TILE_SIZE as i32)) as f32,
                (pos.y * (TILE_SIZE as i32)) as f32,
                TILE_LAYER,
            ),
            Sprite::from_atlas_image(
                tile_resource.outline_sprite.clone(),
                TextureAtlas {
                    layout: tile_resource.outline_layout.clone(),
                    index: index.1 * 4 + index.0,
                },
            ),
            Tiles,
        )
    };
    let isize = IVec2::new(aquarium_size.x as i32, aquarium_size.y as i32);
    // 水槽の左上
    commands.spawn(construct_outline((0, 0), IVec2::new(-1, isize.y)));
    // 上 & 下辺
    for x in 0..isize.x {
        commands.spawn(construct_outline((1, 0), IVec2::new(x, isize.y)));
        commands.spawn(construct_outline((1, 2), IVec2::new(x, -1)));
    }
    // 右上
    commands.spawn(construct_outline((2, 0), IVec2::new(isize.x, isize.y)));
    // 右 & 左辺
    for y in 0..isize.y {
        commands.spawn(construct_outline((0, 1), IVec2::new(-1, y)));
        commands.spawn(construct_outline((2, 1), IVec2::new(isize.x, y)));
    }
    // 左下
    commands.spawn(construct_outline((0, 2), IVec2::new(-1, -1)));
    // 右下
    commands.spawn(construct_outline((2, 2), IVec2::new(isize.x, -1)));
}

pub fn goal_swaying(query: Query<(&mut Transform, &TileCoords), With<Goal>>, time: Res<Time>) {
    for (mut transform, tile_coords) in query {
        let swayness = Vec2::new((time.elapsed_secs() * 3.).sin(), time.elapsed_secs().sin())
            * (TILE_SIZE as f32)
            / 4.;
        transform.translation = (tile_coords.into_vec2() + swayness).extend(TILE_LAYER);
    }
}
