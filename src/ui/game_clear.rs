use bevy::prelude::*;

use crate::{
    MacroStates,
    boxfish::ResultManager,
    prelude::{ConstructAquarium, StageManager},
    ui::UIResource,
};

#[derive(Component)]
pub struct ReturnToMainMenu;

pub fn construction(
    mut commands: Commands,
    result_manager: Res<ResultManager>,
    ucr: Res<UIResource>,
) {
    let steps = result_manager.steps;
    let (rank, prize) = if steps < 300 {
        ("S", "カントリーマアム2枚獲得！")
    } else if steps < 350 {
        ("A", "カントリーマアム1枚獲得！")
    } else if steps < 400 {
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
            Text::new(format!("{}", prize)),
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
            ReturnToMainMenu,
        ));
}

pub fn return_to_main_menu_button(
    stage: Res<StageManager>,
    mut construct_stage: EventWriter<ConstructAquarium>,
    query: Query<&Interaction, (Changed<Interaction>, With<ReturnToMainMenu>)>,
    mut state: ResMut<NextState<MacroStates>>,
) {
    for i in query {
        match i {
            Interaction::Pressed => {
                construct_stage.write(
                    toml::from_str::<ConstructAquarium>(stage.stages.first().unwrap()).unwrap(),
                );
                state.set(MacroStates::MainMenu);
            }
            _ => (),
        }
    }
}
