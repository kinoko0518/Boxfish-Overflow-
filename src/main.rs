#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod boxfish;
mod camera;
mod music;
pub mod prelude;
mod stage;
mod stage_manager;
mod styling;
mod ui;

use bevy::{image::ImageSamplerDescriptor, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use boxfish::PlayerPlugin;

use crate::{
    camera::CameraPlugin, music::MusicPlugin, prelude::*, stage::AquariumPlugin,
    stage_manager::StageManagerPlugin, styling::StylingPlugin, ui::UIPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .init_state::<MacroStates>()
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(StageManagerPlugin)
        .add_plugins(StylingPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(AquariumPlugin)
        .add_plugins(MusicPlugin)
        .run();
}
