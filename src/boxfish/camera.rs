use bevy::prelude::*;

use crate::{TILE_SIZE, stage_manager::ConstructAquarium};

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
