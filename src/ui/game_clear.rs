use crate::prelude::*;
use bevy::prelude::*;

use crate::{boxfish::ResultManager, ui::UIResource};

const S_RANK_MAX: u32 = 400;
const A_RANK_MAX: u32 = 450;
const B_RANK_MAX: u32 = 500;

#[derive(Component)]
/// Reference [return_to_main_menu_button]
/// how a button with this component works.
pub struct ReturnToMainMenuButton;

/// Constructing a result menu with those parts:
///
/// - Steps
/// - Rank
/// - Prize (Originally, it was a project for my school festival)
/// - Back to main screen
pub fn result_menu_construction(
    mut commands: Commands,
    result_manager: Res<ResultManager>,
    ucr: Res<UIResource>,
) {
    let steps = result_manager.steps;
    let (rank, prize) = if steps < S_RANK_MAX {
        ("S", "カントリーマアム2枚獲得！")
    } else if steps < A_RANK_MAX {
        ("A", "カントリーマアム1枚獲得！")
    } else if steps < B_RANK_MAX {
        ("B", "チョコ2個獲得！")
    } else {
        ("C", "チョコ1個獲得！")
    };
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::linear_rgba(0., 0., 0., 0.7)),
            StateScoped(MacroStates::GameClear),
        ))
        .with_child((
            Text::new(format!("手数：{}", steps)),
            TextColor::WHITE,
            TextFont {
                font: ucr.font.clone(),
                font_size: 48.,
                ..default()
            },
        ))
        .with_child((
            Text::new(format!("ランク：{}", rank)),
            TextColor::WHITE,
            TextFont {
                font: ucr.font.clone(),
                font_size: 80.,
                ..default()
            },
        ))
        .with_child((
            Text::new(prize.to_string()),
            TextColor::WHITE,
            TextFont {
                font: ucr.font.clone(),
                font_size: 48.,
                ..default()
            },
        ))
        .with_child((
            Text::new("クリックでタイトルに戻る"),
            TextColor::WHITE,
            TextFont {
                font: ucr.font.clone(),
                font_size: 48.,
                ..default()
            },
            Button,
            ReturnToMainMenuButton,
        ));
}

/// On a button which has a [ReturnToMainMenuButton] component clicked,
/// reset the game then back to the main menu.
pub fn return_to_main_menu_button(
    mut construct_stage: EventWriter<NewGame>,
    query: Query<&Interaction, (Changed<Interaction>, With<ReturnToMainMenuButton>)>,
    mut state: ResMut<NextState<MacroStates>>,
) {
    for i in query {
        if *i == Interaction::Pressed {
            construct_stage.write(NewGame);
            state.set(MacroStates::ESCMenu);
        }
    }
}
