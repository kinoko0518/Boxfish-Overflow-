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
            .add_systems(Startup, init_ucr)
            .add_systems(PostStartup, reset_exit_hint::ui_construction)
            .add_systems(
                OnEnter(MacroStates::MainMenu),
                main_menu::construct_ui.after(init_ucr),
            )
            .add_systems(
                OnEnter(MacroStates::GamePlay),
                operation_hint::construct_ui.after(init_ucr),
            )
            .add_systems(
                Update,
                (reset_exit_hint::countup_duration,).run_if(in_state(MacroStates::GamePlay)),
            )
            .add_systems(
                Update,
                (main_menu::end_game_button, main_menu::start_button)
                    .run_if(in_state(MacroStates::MainMenu)),
            )
            .add_systems(Update, (reset_exit_hint::stage_index_display, toggle_menu));
    }
}

// UIで使われるデータを読み込み・保管
#[derive(Resource, Default)]
pub struct UIResource {
    font: Handle<Font>,
    text_font: TextFont,
    wasd: Handle<Image>,
    shift: Handle<Image>,
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
}

pub fn toggle_menu(
    mut state_mut: ResMut<NextState<MacroStates>>,
    state: ResMut<State<MacroStates>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            &MacroStates::GamePlay => state_mut.set(MacroStates::MainMenu),
            &MacroStates::MainMenu => state_mut.set(MacroStates::GamePlay),
        }
    }
}
