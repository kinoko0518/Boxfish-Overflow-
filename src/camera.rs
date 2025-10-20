use bevy::prelude::*;

use crate::{MacroStates, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StageCentre>()
            .add_systems(Startup, construct_camera)
            .add_systems(Update, get_stage_centre)
            .add_systems(Update, move_to_ideal);
    }
}

struct IdealCamStat {
    pos: Vec3,
    magnif: f32,
}

const MAIN_MENU_CAM_POS: Vec3 = Vec3::new(-300., 0., 0.);

#[derive(Resource, Default)]
pub struct StageCentre {
    centre: Vec3,
}

pub fn construct_camera(mut commands: Commands) {
    commands
        .spawn((Camera2d, Transform::from_xyz(0., 0., 0.)))
        .insert(Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }));
}

pub fn get_stage_centre(
    mut centre: ResMut<StageCentre>,
    mut event_reader: EventReader<ConstructAquarium>,
) {
    if let Some(event) = event_reader.read().next() {
        let lines = event.content.lines();
        let size = Vec3::new(
            lines.clone().map(|l| l.len()).max().unwrap_or(0) as f32,
            lines.collect::<Vec<&str>>().len() as f32,
            0.,
        ) * (TILE_SIZE as f32);
        centre.centre = size * 0.5;
    }
}

const CAM_SPEED: f32 = 200.;
const MAGNIF_CHANGE_SPEED: f32 = 2.;

pub fn move_to_ideal(
    time: Res<Time>,
    state: Res<State<MacroStates>>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
    centre: Res<StageCentre>,
) {
    let ideal = match state.get() {
        MacroStates::MainMenu => IdealCamStat {
            pos: MAIN_MENU_CAM_POS + centre.centre,
            magnif: 0.75,
        },
        MacroStates::GamePlay => IdealCamStat {
            pos: centre.centre,
            magnif: 0.5,
        },
    };
    for (mut transform, mut projection) in &mut query {
        let pos_difference = ideal.pos - transform.translation;
        let duration = pos_difference.normalize() * CAM_SPEED * time.delta_secs();

        if pos_difference.length() > duration.length() {
            // 通り過ぎないとき
            transform.translation += duration;
        } else {
            // 通り過ぎるとき
            transform.translation = ideal.pos;
        }
        // 拡大率を変更
        if let Projection::Orthographic(ref mut orth) = *projection {
            if orth.scale != ideal.magnif {
                let displacement =
                    (orth.scale - ideal.magnif).signum() * MAGNIF_CHANGE_SPEED * time.delta_secs();
                if (orth.scale - displacement).signum() == displacement.signum() {
                    // 通り過ぎないとき
                    orth.scale -= displacement;
                } else {
                    // 通り過ぎるとき
                    orth.scale = ideal.magnif;
                }
            }
        }
    }
}
