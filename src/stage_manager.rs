use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct StageManagerPlugin;

impl Plugin for StageManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConstructAquarium>()
            .add_event::<GameClear>()
            .add_event::<NextStage>()
            .init_resource::<StageManager>()
            .add_systems(Startup, setup_stage_manager)
            .add_systems(Update, call_next_aquarium);
    }
}

#[derive(Event, Clone, Serialize, Deserialize)]
pub struct ConstructAquarium {
    pub content: String,
    pub player_origin: IVec2,
    pub player_defaultbits: Vec<bool>,
}

#[derive(Resource, Default)]
pub struct StageManager {
    pub stages: Vec<&'static str>,
    pub index: usize,
}

const STAGE_1: &'static str = include_str!("../assets/stages/stage_1.toml");
const STAGE_2: &'static str = include_str!("../assets/stages/stage_2.toml");
const STAGE_7: &'static str = include_str!("../assets/stages/stage_7.toml");

pub fn setup_stage_manager(
    mut stage: ResMut<StageManager>,
    mut next_stage: EventWriter<NextStage>,
) {
    stage.stages = vec![STAGE_1, STAGE_2, STAGE_7];
    next_stage.write(NextStage);
}

#[derive(Event)]
pub struct GameClear;

#[derive(Event)]
pub struct NextStage;

pub fn call_next_aquarium(
    mut construct_aquarium: EventWriter<ConstructAquarium>,
    mut game_clear: EventWriter<GameClear>,
    mut next_stage: EventReader<NextStage>,
    mut stage_manager: ResMut<StageManager>,
) {
    for _ in next_stage.read() {
        match stage_manager.stages.get(stage_manager.index) {
            Some(next_stage) => {
                construct_aquarium.write(
                    toml::from_str(next_stage).expect("The format of a aquarium is not satisfied!"),
                );
                println!("Stage {} was loaded!", stage_manager.index);
                stage_manager.index += 1;
            }
            None => {
                game_clear.write(GameClear);
            }
        }
    }
}
