use crate::{MacroStates, ui::UICommonResource};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct OperationHintUI {
    wasd: Handle<Image>,
    shift: Handle<Image>,
}

pub fn init_resource(mut res: ResMut<OperationHintUI>, asset_server: Res<AssetServer>) {
    res.wasd = asset_server.load("embedded://ui/wasd.png");
    res.shift = asset_server.load("embedded://ui/shift.png");
}

const PERCENT_PER_PIXEL: f32 = 6. / 32.;
pub fn construct_ui(
    mut commands: Commands,
    ucr: Res<UICommonResource>,
    oper_hint_res: Res<OperationHintUI>,
) {
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
                .spawn((
                    Node {
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Vw(3.)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    StateScoped(MacroStates::GamePlay),
                ))
                .with_child((
                    Text::new("イドウ".to_string()),
                    TextColor::BLACK,
                    ucr.text_font.clone(),
                    StateScoped(MacroStates::GamePlay),
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
                    StateScoped(MacroStates::GamePlay),
                ))
                .with_child((
                    Text::new("ノビル".to_string()),
                    TextColor::BLACK,
                    ucr.text_font.clone(),
                    StateScoped(MacroStates::GamePlay),
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
                    StateScoped(MacroStates::GamePlay),
                ));
        });
}
