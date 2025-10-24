use bevy::{audio::Volume, prelude::*};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bgm);
    }
}

pub fn spawn_bgm(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer(asset_server.load::<AudioSource>("embedded://musics/Boxfish-01.wav")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::Linear(0.3),
            ..default()
        },
    ));
}
