use super::UIResource;

use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ResetDurationDisplay {
    reset_duration: f32,
}

#[derive(Component)]
pub struct StageIndexDisplay;

/// Constructing stage index display and
/// reset duration display on the upper left of a screen.
pub fn upper_left_menu_construction(mut commands: Commands, ucr: Res<UIResource>) {
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
            ResetDurationDisplay { reset_duration: 0. },
            StateScoped(MacroStates::GamePlay),
        ));
}

/// With setting this constant, how many secs
/// reset button must be keep pressed will be changed.
const RESET_EXPECTED_PRESSTIME: f32 = 5.;

/// Increament reset duration while reset button pressed.
pub fn countup_reset_duration(
    query: Query<(&mut Text, &mut ResetDurationDisplay)>,
    key_input: Res<ButtonInput<KeyCode>>,
    gamepad: Query<&Gamepad>,
    time: Res<Time>,
    stage_manager: Res<StageManager>,
    mut construct_aquarium: EventWriter<ConstructAquarium>,
) {
    // Detect that was R on keyboard or North button on gamepad pressed.
    let pressed = match gamepad.single() {
        Ok(gamepad) => gamepad.pressed(GamepadButton::North),
        Err(_) => false,
    } | key_input.pressed(KeyCode::KeyR);
    for (mut text, mut duration) in query {
        if pressed {
            duration.reset_duration += time.delta_secs();
            // If pressed time is greater than [RESET_EXPECTED_PRESSTIME],
            // call the same stage as now, and reset pressed duration.
            if duration.reset_duration > RESET_EXPECTED_PRESSTIME {
                construct_aquarium.write(
                    toml::from_str::<ConstructAquarium>(
                        stage_manager.stages.get(stage_manager.index).unwrap(),
                    )
                    .unwrap(),
                );
                duration.reset_duration = 0.;
            }
            // Display reset duration. The amount of "." is
            // corresponging to how many seconds passed.
            text.0 = format!(
                "{}秒後にステージをリセット",
                (RESET_EXPECTED_PRESSTIME - duration.reset_duration).ceil() as u32
            ) + &".".repeat(duration.reset_duration as usize);
        } else {
            duration.reset_duration = 0.;
            text.0 = String::new();
        }
    }
}

/// In this system, updating StageIndexDisplay with latest information
/// and hidding it on out of GamePlay state.
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
