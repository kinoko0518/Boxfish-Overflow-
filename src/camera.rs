use bevy::prelude::*;

use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, construct_camera)
            .add_systems(Update, camera_adjust);
    }
}

pub fn construct_camera(mut commands: Commands) {
    commands
        .spawn((Camera2d, Transform::from_xyz(0., 0., 0.)))
        .insert(Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }));
}

pub fn camera_adjust(
    query: Query<&mut Transform, With<Camera2d>>,
    mut event_reader: EventReader<ConstructAquarium>,
) {
    if let Some(event) = event_reader.read().next() {
        for mut transform in query {
            let lines = event.content.lines();
            let size = Vec3::new(
                lines.clone().map(|l| l.len()).max().unwrap_or(0) as f32,
                lines.collect::<Vec<&str>>().len() as f32,
                0.,
            ) * (TILE_SIZE as f32);
            transform.translation = size * 0.5;
        }
    }
}
