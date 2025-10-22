use bevy::prelude::*;

use crate::{MacroStates, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamRes>()
            .add_systems(Startup, construct_camera)
            .add_systems(Update, (get_stage_centre, move_to_ideal));
    }
}

const MAIN_MENU_CAM_POS: Vec3 = Vec3::new(-300., 0., 0.);

#[derive(Resource, Default)]
pub struct CamRes {
    progress: f32,
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
    mut centre: ResMut<CamRes>,
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

const OPEN_CLOSE_DURATION: f32 = 0.3;

pub fn move_to_ideal(
    time: Res<Time>,
    state: Res<State<MacroStates>>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
    mut cam_res: ResMut<CamRes>,
) {
    for (mut transform, mut projection) in &mut query {
        let t = cam_res.progress;
        let eased_progress = t * t * (3.0 - 2.0 * t);

        if cam_res.progress < 1.0 {
            // 位置を変更
            transform.translation = cam_res.centre + MAIN_MENU_CAM_POS * eased_progress;
            // 拡大率を変更
            if let Projection::Orthographic(ref mut orth) = *projection {
                orth.scale = 0.5 + 0.25 * eased_progress;
            }
        }
        // このフレームで変更されるprogressの絶対値
        let travel = time.delta_secs() / OPEN_CLOSE_DURATION;
        // 0.~1.の範囲でprogressを再代入
        cam_res.progress = match state.get() {
            &MacroStates::GamePlay => (0. as f32).max(cam_res.progress - travel),
            &MacroStates::MainMenu => (1. as f32).min(cam_res.progress + travel),
        }
    }
}
