use bevy::prelude::*;

use crate::{MacroStates, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamRes>()
            .add_systems(Startup, construct_camera)
            .add_systems(Update, (get_stage_centre, update_ideal, move_to_ideal));
    }
}

#[derive(PartialEq, Clone, Default)]
struct IdealCamStat {
    pos: Vec3,
    magnif: f32,
}

const MAIN_MENU_CAM_POS: Vec3 = Vec3::new(-300., 0., 0.);

#[derive(Resource, Default)]
pub struct CamRes {
    last: IdealCamStat,
    now: IdealCamStat,
    duration: f32,
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

pub fn update_ideal(state: Res<State<MacroStates>>, mut cam_res: ResMut<CamRes>) {
    let ideal = match state.get() {
        MacroStates::MainMenu => IdealCamStat {
            pos: MAIN_MENU_CAM_POS + cam_res.centre,
            magnif: 0.75,
        },
        MacroStates::GamePlay => IdealCamStat {
            pos: cam_res.centre,
            magnif: 0.5,
        },
    };
    if cam_res.now != ideal {
        cam_res.last = cam_res.now.clone();
        cam_res.now = ideal;
        cam_res.duration = 0.;
    }
}

pub fn move_to_ideal(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
    mut cam_res: ResMut<CamRes>,
) {
    for (mut transform, mut projection) in &mut query {
        if cam_res.duration < 1.0 {
            // 位置を変更
            transform.translation =
                cam_res.last.pos + (cam_res.now.pos - cam_res.last.pos) * cam_res.duration;
            // 拡大率を変更
            if let Projection::Orthographic(ref mut orth) = *projection {
                orth.scale = cam_res.last.magnif
                    + (cam_res.now.magnif - cam_res.last.magnif) * cam_res.duration;
            }
        }
        // durationを加算（1秒で完了）
        cam_res.duration += time.delta_secs();
    }
}
