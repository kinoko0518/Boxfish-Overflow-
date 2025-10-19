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
        match &self.direction {
            &Direction::X => IVec2::new(self.amount, 0),
            &Direction::Y => IVec2::new(0, self.amount),
        }
    }
    pub fn get_route(&self, origin: IVec2) -> Vec<IVec2> {
        let sign = self.amount.signum();
        (1..((self.amount.abs() as usize) + 1))
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

pub fn player_input(keyboard_input: &Res<ButtonInput<KeyCode>>) -> Travel {
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        Travel {
            direction: Direction::Y,
            amount: 1,
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyS) {
        Travel {
            direction: Direction::Y,
            amount: -1,
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyA) {
        Travel {
            direction: Direction::X,
            amount: -1,
        }
    } else if keyboard_input.just_pressed(KeyCode::KeyD) {
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
