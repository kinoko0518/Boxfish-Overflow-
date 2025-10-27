mod construction;
mod resource;
mod visual;

pub use crate::aquarium::resource::AquariumResource;

use crate::prelude::*;
use bevy::prelude::*;

pub struct AquariumPlugin;

impl Plugin for AquariumPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConstructionCompleted>()
            .init_resource::<AquariumResource>()
            .add_systems(Startup, resource::init_aquarium_resource)
            .add_systems(Update, visual::highlight_incorrect_bits)
            .add_systems(Update, visual::goal_swaying)
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
/// 膨張しているときに通れないタイルを識別するコンポーネント
pub struct SemiCollidable;

#[derive(Component, Debug)]
/// ゲートのうち、プレイヤーと接触したときに異なるビットだった、
/// すなわち条件が満たされていないビットを赤くハイライトするためのコンポーネント
/// remainingは自然に減少し、255であるときに完全に赤くなる。
pub struct IncorrectBit {
    pub remaining: u8,
}

#[derive(Component)]
/// 論理ゲートのレジスタのコンポーネント
pub struct LogiRegister {
    pub boolean: bool,
    pub logikind: LogiKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogiKind {
    And,
    Or,
    Not,
    Xor,
    Undo,
    Equal,
}

const TILE_LAYER: f32 = 0.;

#[derive(Component)]
pub struct Goal;

#[derive(Component)]
pub struct StageCompleted;

#[derive(Event)]
pub struct ConstructionCompleted;

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
        construction::chars_into_tiles(&aq.content, commands, tile_resource);
        construction_completed.write(ConstructionCompleted);
    }
}
