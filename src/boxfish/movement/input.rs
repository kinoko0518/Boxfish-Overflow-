use bevy::prelude::*;

#[derive(Clone)]
pub struct Travel {
    pub direction: Direction,
    pub amount: i32,
}

#[derive(Clone)]
pub enum Direction {
    X,
    Y,
}

impl Travel {
    pub fn into_ivec2(&self) -> IVec2 {
        match self.direction {
            Direction::X => IVec2::new(self.amount, 0),
            Direction::Y => IVec2::new(0, self.amount),
        }
    }
    pub fn get_route(&self, origin: IVec2) -> Vec<IVec2> {
        let sign = self.amount.signum();
        (1..((self.amount.unsigned_abs() as usize) + 1))
            .map(|i| {
                let i = sign * (i as i32);
                origin
                    + match self.direction {
                        Direction::X => IVec2::new(i, 0),
                        Direction::Y => IVec2::new(0, i),
                    }
            })
            .collect::<Vec<IVec2>>()
    }
}

pub fn player_input(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    gamepad_input: &Query<&Gamepad>,
) -> Travel {
    // ゲームパッド関連の判定
    if let Ok(gamepad) = gamepad_input.single() {
        const THREHOLD: f32 = 0.5;
        if let Some(x) = gamepad.get(GamepadAxis::LeftStickX)
            && x.abs() > THREHOLD
        {
            return Travel {
                direction: Direction::X,
                amount: x.signum() as i32,
            };
        }
        if let Some(y) = gamepad.get(GamepadAxis::LeftStickY)
            && y.abs() > THREHOLD
        {
            return Travel {
                direction: Direction::Y,
                amount: y.signum() as i32,
            };
        }
    }
    // キーボードの処理
    if keyboard_input.pressed(KeyCode::KeyW) {
        Travel {
            direction: Direction::Y,
            amount: 1,
        }
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        Travel {
            direction: Direction::Y,
            amount: -1,
        }
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        Travel {
            direction: Direction::X,
            amount: -1,
        }
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        Travel {
            direction: Direction::X,
            amount: 1,
        }
    } else {
        Travel {
            direction: Direction::X,
            amount: 0,
        }
    }
}
