mod main_menu;
mod operation_hint;
mod reset_exit_hint;

use crate::prelude::*;
use bevy::prelude::*;

pub struct UIPlugin;

const PERCENT_PER_PIXEL: f32 = 6. / 32.;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIResource>()
            .init_resource::<reset_exit_hint::LUMessageConstainer>()
            .add_systems(
                Startup,
                (init_ucr, reset_exit_hint::ui_construction).chain(),
            )
            .add_systems(OnEnter(MacroStates::MainMenu), main_menu::construct_ui)
            .add_systems(OnEnter(MacroStates::GamePlay), operation_hint::construct_ui)
            .add_systems(
                Update,
                (reset_exit_hint::countup_duration,).run_if(in_state(MacroStates::GamePlay)),
            )
            .add_systems(
                Update,
                (main_menu::end_game_button, main_menu::start_button)
                    .run_if(in_state(MacroStates::MainMenu)),
            )
            .add_systems(Update, reset_exit_hint::stage_index_display);
    }
}

// UIで使われるデータを読み込み・保管
#[derive(Resource, Default)]
pub struct UIResource {
    font: Handle<Font>,
    text_font: TextFont,
    wasd: Handle<Image>,
    shift: Handle<Image>,
    logo: Handle<Image>,
}

pub fn init_ucr(mut ucr: ResMut<UIResource>, asset_server: Res<AssetServer>) {
    ucr.font = asset_server.load("fonts/k8x12.ttf");
    ucr.text_font = TextFont {
        font: ucr.font.clone(),
        font_size: 32.,
        ..default()
    };
    ucr.wasd = asset_server.load("embedded://ui/wasd.png");
    ucr.shift = asset_server.load("embedded://ui/shift.png");
    ucr.logo = asset_server.load("embedded://ui/logo.png");
}
