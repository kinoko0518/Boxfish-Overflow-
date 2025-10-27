use super::{PERCENT_PER_PIXEL, UIResource};
use crate::prelude::*;
use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct EndGameButton;

/// Constructs the menu shown when ESC key pressed on gameplayv, with those parts:
///
/// - The game logo
/// - Start Game
/// - Quit Game
pub fn construct_esc_menu(
    mut commands: Commands,
    ucr: Res<UIResource>,
    asset_server: Res<AssetServer>,
) {
    let menu_font = TextFont {
        font: ucr.font.clone(),
        font_size: 48.,
        ..default()
    };
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Vw(3.)),
                align_items: AlignItems::Baseline,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            StateScoped(MacroStates::ESCMenu),
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode {
                    image: asset_server.load("embedded://ui/logo.png"),
                    ..default()
                },
                Node {
                    width: Val::Vw(PERCENT_PER_PIXEL * 192.),
                    ..default()
                },
            ));
            parent
                .spawn(Node {
                    height: Val::Percent(100.),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_child((
                    Button,
                    StartButton,
                    TextColor::BLACK,
                    Text::new("ハジメル"),
                    menu_font.clone(),
                ))
                .with_child((
                    Button,
                    EndGameButton,
                    TextColor::BLACK,
                    Text::new("オワル"),
                    menu_font.clone(),
                ));
        });
}

pub fn on_start_button_clicked(
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut macro_state: ResMut<NextState<MacroStates>>,
) {
    for i in query {
        if *i == Interaction::Pressed {
            macro_state.set(MacroStates::GamePlay);
        }
    }
}

pub fn on_quit_button_clicked(
    mut app_exit: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<EndGameButton>)>,
) {
    for i in query {
        if i == &Interaction::Pressed {
            app_exit.write(AppExit::Success);
        }
    }
}

pub fn button_sounds(
    mut commands: Commands,
    query: Query<&Interaction, Changed<Interaction>>,
    resource: Res<UIResource>,
) {
    for i in query {
        let playback_style = PlaybackSettings {
            volume: Volume::Linear(0.3),
            mode: PlaybackMode::Despawn,
            ..default()
        };
        match *i {
            Interaction::Pressed => {
                commands.spawn((AudioPlayer(resource.pressed.clone()), playback_style));
            }
            Interaction::Hovered => {
                commands.spawn((AudioPlayer(resource.focused.clone()), playback_style));
            }
            _ => (),
        }
    }
}
