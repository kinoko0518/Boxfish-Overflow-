mod construction;
mod resource;
mod visual;

pub use crate::stage::resource::AquariumResource;

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
            .add_systems(Update, parse_stage);
    }
}

#[derive(Component)]
/// This is a component to detect tiles on stages.
///
/// With this component, a attached entity
/// will be disapper on a new stage loaded.
pub struct Tiles;

#[derive(Component)]
/// This is a component to detect that
/// does the tile collide with the boxfish.
pub struct Collidable;

#[derive(Component)]
/// This is a component to detect that
/// does the tile collide with the boxfish
/// when the boxfish isn't expanding.
pub struct SemiCollidable;

#[derive(Component, Debug)]
/// This is a component to highlight gates with red colour,
/// which was different from a collided player's register.
pub struct IncorrectBit {
    pub remaining: u8,
}

#[derive(Component)]
/// This is a component for logical gates' register.
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

pub fn parse_stage(
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
