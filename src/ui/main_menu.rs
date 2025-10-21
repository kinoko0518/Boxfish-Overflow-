use super::UICommonResource;
use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct EndGameButton;

pub fn construct_ui(mut commands: Commands, ucr: Res<UICommonResource>) {
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
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            StateScoped(MacroStates::MainMenu),
        ))
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
}

pub fn start_button(
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut macro_state: ResMut<NextState<MacroStates>>,
) {
    for i in query {
        match &i {
            &Interaction::Pressed => {
                macro_state.set(MacroStates::GamePlay);
            }
            _ => (),
        }
    }
}

pub fn end_game_button(
    mut app_exit: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<EndGameButton>)>,
) {
    for i in query {
        match &i {
            &Interaction::Pressed => {
                app_exit.write(AppExit::Success);
            }
            _ => (),
        }
    }
}
