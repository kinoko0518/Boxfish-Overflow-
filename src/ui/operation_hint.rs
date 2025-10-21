use super::PERCENT_PER_PIXEL;
use crate::{MacroStates, ui::UIResource};
use bevy::prelude::*;

pub fn construct_ui(mut commands: Commands, ucr: Res<UIResource>, oper_hint_res: Res<UIResource>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            StateScoped(MacroStates::GamePlay),
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Vw(3.)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },))
                .with_child((
                    Text::new("イドウ".to_string()),
                    TextColor::BLACK,
                    ucr.text_font.clone(),
                ))
                .with_child((
                    ImageNode {
                        image: oper_hint_res.wasd.clone(),
                        ..default()
                    },
                    Node {
                        width: Val::Vw(PERCENT_PER_PIXEL * 48.),
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("ノビル".to_string()),
                    TextColor::BLACK,
                    ucr.text_font.clone(),
                ))
                .with_child((
                    ImageNode {
                        image: oper_hint_res.shift.clone(),
                        ..default()
                    },
                    Node {
                        width: Val::Vw(PERCENT_PER_PIXEL * 32.),
                        ..default()
                    },
                ));
        });
}
