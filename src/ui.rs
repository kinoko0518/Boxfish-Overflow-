mod main_menu;
mod operation_hint;
mod reset_exit_hint;

use crate::prelude::*;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UICommonResource>()
            .init_resource::<reset_exit_hint::LUMessageConstainer>()
            .init_resource::<operation_hint::OperationHintUI>()
            .add_systems(Startup, (init_ucr, operation_hint::init_resource).chain())
            .add_systems(Startup, reset_exit_hint::ui_construction)
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

// UIで汎用的に使われるデータを読み込み・保管
#[derive(Resource, Default)]
pub struct UICommonResource {
    font: Handle<Font>,
    text_font: TextFont,
}

pub fn init_ucr(mut ucr: ResMut<UICommonResource>, asset_server: Res<AssetServer>) {
    ucr.font = asset_server.load("fonts/k8x12.ttf");
    ucr.text_font = TextFont {
        font: ucr.font.clone(),
        font_size: 32.,
        ..default()
    };
}
