use bevy::prelude::*;
mod boxfish;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, boxfish::boxfish_setup)
        .add_systems(Update, boxfish::bit_system)
        .add_systems(Update, boxfish::body_system)
        .add_systems(Update, boxfish::face_system)
        .add_systems(Update, boxfish::boxfish_moving)
        .run();
}
