use bevy::prelude::*;
use bevy::winit::WinitWindows;
use image;
use winit::window::Icon;

const WINDOW_ICON: &[u8] = include_bytes!("../assets/boxfish/head.png");

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(WINDOW_ICON)
            .expect("Failed to load icon from memory")
            .into_rgba8();
        let resized =
            image::imageops::resize(&image, 256, 256, image::imageops::FilterType::Nearest);
        let (width, height) = resized.dimensions();
        let rgba = resized.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

pub struct StylingPlugin;

impl Plugin for StylingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_icon)
            .insert_resource(ClearColor(Color::linear_rgb(0.0, 0.0, 0.0)));
    }
}
