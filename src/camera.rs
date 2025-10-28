use bevy::prelude::*;

use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamRes>()
            .add_systems(Startup, construct_camera)
            .add_systems(
                Update,
                (get_centre_coords_of_stage, move_to_camera_to_ideal_pos),
            );
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

pub fn get_centre_coords_of_stage(
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

/// How many sec will be took to open or close esc menu.
const ESC_MENU_OPEN_CLOSE_DURATION: f32 = 0.3;

pub fn move_to_camera_to_ideal_pos(
    time: Res<Time>,
    state: Res<State<MacroStates>>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
    mut cam_res: ResMut<CamRes>,
) {
    for (mut transform, mut projection) in &mut query {
        let t = cam_res.progress;
        let eased_progress = t * t * (3.0 - 2.0 * t);

        if cam_res.progress < 1.0 {
            // Change position smoothly
            transform.translation = cam_res.centre + MAIN_MENU_CAM_POS * eased_progress;
            // Change scale smoothly
            if let Projection::Orthographic(ref mut orth) = *projection {
                orth.scale = 0.5 + 0.25 * eased_progress;
            }
        }
        // The absolute value of progress which is modified on this frame
        let travel = time.delta_secs() / ESC_MENU_OPEN_CLOSE_DURATION;
        // Re-assign progress in the range of 0.~1.
        cam_res.progress = match state.get() {
            &MacroStates::ESCMenu => (1_f32).min(cam_res.progress + travel),
            _ => (0_f32).max(cam_res.progress - travel),
        }
    }
}
