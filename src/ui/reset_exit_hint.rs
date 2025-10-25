use super::UIResource;

use crate::prelude::*;
use bevy::prelude::*;

// 左(Left)上(Up)に表示されるメッセージの状態管理用リソース
#[derive(Resource, Default)]
pub struct LUMessageConstainer {
    reset_duration: f32,
}

#[derive(Component)]
pub struct LUMessage;

#[derive(Component)]
pub struct StageIndexDisplay;

pub fn ui_construction(mut commands: Commands, ucr: Res<UIResource>) {
    commands
        .spawn((Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Baseline,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Vw(3.)),
            ..default()
        },))
        .with_child((
            Text::new(String::new()),
            TextColor::BLACK,
            ucr.text_font.clone(),
            StageIndexDisplay,
        ))
        .with_child((
            Text::new(String::new()),
            TextColor::BLACK,
            ucr.text_font.clone(),
            LUMessage,
            StateScoped(MacroStates::GamePlay),
        ));
}

const RESET_EXPECTED_PRESSTIME: f32 = 5.;
pub fn countup_duration(
    lu_text_query: Query<&mut Text, With<LUMessage>>,
    mut lmc: ResMut<LUMessageConstainer>,
    key_input: Res<ButtonInput<KeyCode>>,
    gamepad: Query<&Gamepad>,
    time: Res<Time>,
    stage_manager: Res<StageManager>,
    mut construct_aquarium: EventWriter<ConstructAquarium>,
) {
    let pressed = match gamepad.single() {
        Ok(gamepad) => gamepad.pressed(GamepadButton::North),
        Err(_) => false,
    } | key_input.pressed(KeyCode::KeyR);
    if pressed {
        lmc.reset_duration += time.delta_secs();
        if lmc.reset_duration > RESET_EXPECTED_PRESSTIME {
            construct_aquarium.write(
                toml::from_str::<ConstructAquarium>(
                    stage_manager.stages.get(stage_manager.index).unwrap(),
                )
                .unwrap(),
            );
            lmc.reset_duration = 0.;
        }
    } else {
        lmc.reset_duration = 0.;
    }
    for mut tex in lu_text_query {
        if lmc.reset_duration > 0. {
            tex.0 = format!(
                "{}秒後にステージをリセット",
                ((RESET_EXPECTED_PRESSTIME - lmc.reset_duration) as usize + 1)
            ) + &".".repeat(lmc.reset_duration as usize);
        } else {
            tex.0 = String::new();
        }
    }
}

pub fn stage_index_display(
    state: Res<State<MacroStates>>,
    mut text_query: Query<&mut Text, With<StageIndexDisplay>>,
    mut visibility_query: Query<&mut Visibility, With<StageIndexDisplay>>,
    stage_manager: Res<StageManager>,
    mut construct_aquarium: EventReader<ConstructAquarium>,
) {
    for ca in construct_aquarium.read() {
        for mut text in &mut text_query {
            text.0 = format!("ステージ{} - {}", stage_manager.index + 1, ca.stage_name);
        }
    }
    for mut visibility in &mut visibility_query {
        *visibility = match state.get() {
            MacroStates::GamePlay => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
