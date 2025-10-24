mod game_clear;
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
            .add_systems(OnEnter(MacroStates::GameClear), game_clear::construction)
            .add_systems(
                Update,
                (reset_exit_hint::countup_duration,).run_if(in_state(MacroStates::GamePlay)),
            )
            .add_systems(
                Update,
                (
                    main_menu::end_game_button,
                    main_menu::start_button,
                    main_menu::button_sounds,
                )
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
    focused: Handle<AudioSource>,
    pressed: Handle<AudioSource>,
    menu_exit: Handle<AudioSource>,
    menu_enter: Handle<AudioSource>,
}

pub fn init_ucr(mut ucr: ResMut<UIResource>, asset_server: Res<AssetServer>) {
    ucr.font = asset_server.load("embedded://fonts/k8x12.ttf");
    ucr.text_font = TextFont {
        font: ucr.font.clone(),
        font_size: 32.,
        ..default()
    };
    ucr.wasd = asset_server.load("embedded://ui/wasd.png");
    ucr.shift = asset_server.load("embedded://ui/shift.png");
    ucr.focused = asset_server.load("embedded://sound_effects/ui_focused.mp3");
    ucr.pressed = asset_server.load("embedded://sound_effects/ui_accepted.mp3");
    ucr.menu_enter = asset_server.load("embedded://sound_effects/extend.mp3");
    ucr.menu_exit = asset_server.load("embedded://sound_effects/shrink.mp3");
}

pub fn toggle_menu(
    mut commands: Commands,
    mut state_mut: ResMut<NextState<MacroStates>>,
    state: ResMut<State<MacroStates>>,
    key_input: Res<ButtonInput<KeyCode>>,
    ucr: Res<UIResource>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            &MacroStates::GamePlay => {
                commands.spawn(AudioPlayer(ucr.menu_exit.clone()));
                state_mut.set(MacroStates::MainMenu);
            }
            &MacroStates::MainMenu => {
                commands.spawn(AudioPlayer(ucr.menu_enter.clone()));
                state_mut.set(MacroStates::GamePlay);
            }
            _ => (),
        }
    }
}
