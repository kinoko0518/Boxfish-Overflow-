use bevy::{audio::Volume, prelude::*};
use serde::{Deserialize, Serialize};

pub struct StageManagerPlugin;

impl Plugin for StageManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConstructAquarium>()
            .add_event::<GameClear>()
            .add_event::<NextStage>()
            .init_resource::<StageManager>()
            .add_systems(Startup, setup_stage_manager)
            .add_systems(Update, (call_next_aquarium, soundeffect_on_stage_loaded));
    }
}

#[derive(Event, Clone, Serialize, Deserialize)]
pub struct ConstructAquarium {
    pub stage_name: String,
    pub content: String,
    pub player_origin: IVec2,
    pub player_defaultbits: Vec<bool>,
}

#[derive(Resource, Default)]
pub struct StageManager {
    pub stages: Vec<&'static str>,
    pub index: usize,
    pub on_loaded_soundeffect: Handle<AudioSource>,
}

const STAGE_0: &'static str = include_str!("../assets/stages/stage_0.toml");
const STAGE_1: &'static str = include_str!("../assets/stages/stage_1.toml");
const STAGE_2: &'static str = include_str!("../assets/stages/stage_2.toml");
const STAGE_3: &'static str = include_str!("../assets/stages/stage_3.toml");
const STAGE_4: &'static str = include_str!("../assets/stages/stage_4.toml");
const STAGE_5: &'static str = include_str!("../assets/stages/stage_5.toml");
const STAGE_6: &'static str = include_str!("../assets/stages/stage_6.toml");
const STAGE_7: &'static str = include_str!("../assets/stages/stage_7.toml");

pub fn setup_stage_manager(
    mut stage: ResMut<StageManager>,
    mut construct_stage: EventWriter<ConstructAquarium>,
    asset_server: Res<AssetServer>,
) {
    stage.stages = vec![
        STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6, STAGE_7,
    ];
    stage.on_loaded_soundeffect = asset_server.load("embedded://sound_effects/load_stage.ogg");
    construct_stage
        .write(toml::from_str(stage.stages.get(0).unwrap()).expect("Stage 0 is broken!"));
}

#[derive(Event)]
pub struct GameClear;

#[derive(Event)]
pub struct NextStage;

pub fn call_next_aquarium(
    mut stage_manager: ResMut<StageManager>,
    mut construct_aquarium: EventWriter<ConstructAquarium>,
    mut game_clear: EventWriter<GameClear>,
    mut next_stage: EventReader<NextStage>,
) {
    for _ in next_stage.read() {
        match stage_manager.stages.get(stage_manager.index + 1) {
            Some(next_stage) => {
                construct_aquarium.write(
                    toml::from_str(next_stage).expect("The format of a aquarium is not satisfied!"),
                );
                stage_manager.index += 1;
            }
            None => {
                game_clear.write(GameClear);
            }
        }
    }
}

pub fn soundeffect_on_stage_loaded(
    mut commands: Commands,
    stage_manager: Res<StageManager>,
    mut construct_aquarium: EventReader<ConstructAquarium>,
) {
    for _ in construct_aquarium.read() {
        commands.spawn((
            AudioPlayer::new(stage_manager.on_loaded_soundeffect.clone()),
            PlaybackSettings {
                volume: Volume::Linear(0.3),
                ..default()
            },
        ));
    }
}
