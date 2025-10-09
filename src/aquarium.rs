use crate::{Bit, TILE_SIZE, TileCoords};
use bevy::prelude::*;

pub struct AquariumPlugin;

impl Plugin for AquariumPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConstructAquarium>()
            .add_systems(Startup, setup_aquarium)
            .add_systems(Startup, call_default_aquarium)
            .add_systems(Update, tile_adjust)
            .add_systems(Update, parse_aquarium);
    }
}

#[derive(Component)]
pub struct Tiles;

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct LogiGate;

#[derive(Component)]
pub struct TileAdjust;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogiKind {
    And,
    Or,
    Not,
    Xor,
    Gate,
}

#[derive(Event)]
pub struct ConstructAquarium {
    pub content: String,
    pub player_origin: IVec2,
    pub player_defaultbits: Vec<bool>,
}

impl ConstructAquarium {
    pub fn test_stage() -> Self {
        Self {
            content: "
WWWWWWWWWWWW
W          W
W          W
W  N0110N  W
W          W
W          W
WWWWWWWG11GW
      W    W
      W    W
      WWWWWW
            "
            .into(),
            player_origin: IVec2::new(4, 6),
            player_defaultbits: vec![false, false],
        }
    }
}

const WALL_TILESET: &str = "embedded://tile/wall.png";
const LOGIGATE_TILESET: &str = "embedded://tile/logical_gates.png";
const AQUARIUM_PATH: &str = "embedded://background/aquarium.png";

pub fn setup_aquarium(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite {
        image: asset_server.load(AQUARIUM_PATH),
        ..default()
    });
}

pub fn call_default_aquarium(mut construct_aquarium: EventWriter<ConstructAquarium>) {
    construct_aquarium.write(ConstructAquarium::test_stage());
}

pub fn parse_aquarium(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    old_tiles: Query<Entity, With<Tiles>>,
    mut on_loaded: EventReader<ConstructAquarium>,
) {
    if let Some(aq) = on_loaded.read().next() {
        for t in old_tiles {
            commands.entity(t).despawn();
        }
        chars_into_tiles(&aq.content, commands, asset_server, texture_atlas_layouts);
    }
}

fn chars_into_tiles(
    aquarium: &str,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tile_set_image: Handle<Image> = asset_server.load(LOGIGATE_TILESET);
    let tile_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 16, 16, None, None);
    let layout_handle = texture_atlas_layouts.add(tile_layout);

    for (y, s) in aquarium.lines().rev().enumerate() {
        struct State {
            bitkind: Option<LogiKind>,
            tail_found: bool,
        }
        type LogigateBundle = (
            Sprite,
            LogiKind,
            (Transform, TileCoords, Tiles, LogiGate, TileAdjust),
        );
        let mut state = State {
            bitkind: None,
            tail_found: false,
        };
        for (x, c) in s.chars().enumerate() {
            let coords = TileCoords {
                tile_pos: IVec2::new(x as i32, y as i32),
            };
            let bit_common_components = (
                Transform::from_xyz(0., 0., 0.),
                Tiles,
                LogiGate,
                TileAdjust,
                coords.clone(),
            );
            let from_index = |x: usize, y: usize| -> Sprite {
                Sprite::from_atlas_image(
                    tile_set_image.clone(),
                    TextureAtlas {
                        layout: layout_handle.clone(),
                        index: x + y * 16,
                    },
                )
            };
            let mut get_logigate =
                |tail_index: (usize, usize), logikind: LogiKind| -> LogigateBundle {
                    let gate_common_components = (
                        Transform::from_xyz(0., 0., 0.),
                        coords.clone(),
                        Tiles,
                        LogiGate,
                        TileAdjust,
                    );
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
                    (sprite, logikind, gate_common_components)
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
                'W' => {
                    commands.spawn((
                        Sprite::from_image(asset_server.load(WALL_TILESET)),
                        Transform::from_xyz(0., 0., 0.),
                        Tiles,
                        Collidable,
                        TileAdjust,
                        coords,
                    ));
                }
                '0' => {
                    commands.spawn((
                        from_index(1, 0),
                        Bit { boolean: false },
                        bit_common_components,
                        state.bitkind.expect(LOGIKIND_UNDEFINED),
                    ));
                }
                '1' => {
                    commands.spawn((
                        from_index(0, 0),
                        Bit { boolean: true },
                        bit_common_components,
                        state.bitkind.expect(LOGIKIND_UNDEFINED),
                    ));
                }
                _ => (),
            }
        }
    }
}

pub fn tile_adjust(mut query: Query<(&TileCoords, &mut Transform), Changed<TileAdjust>>) {
    for (t_coords, mut transform) in &mut query {
        let t_pos = t_coords.tile_pos;
        transform.translation = Vec3::new(
            (t_pos.x * (TILE_SIZE as i32)) as f32,
            (t_pos.y * (TILE_SIZE as i32)) as f32,
            0.,
        );
    }
}
