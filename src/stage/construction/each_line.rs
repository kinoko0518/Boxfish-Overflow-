use super::{super::resource::AquariumResource, LogiKind, each_char::interprint_each_char_as_tile};
use bevy::prelude::*;

/// LogiKindを類，真理値を真としたとき，
///
/// 類真真...真真類
///
/// という並びなら挟まれた真は類のLogiKindを持つ．
/// そのためにデータを保持する構造体．
pub struct LineContextContainer {
    pub bitkind: Option<LogiKind>,
    pub tail_found: bool,
}

pub fn interprint_each_line_as_tile(
    commands: &mut Commands,
    line: &str,
    y: usize,
    tile_resource: &Res<AquariumResource>,
) {
    let mut state = LineContextContainer {
        bitkind: None,
        tail_found: false,
    };
    // ここからタイルそれぞれについての処理
    for (x, c) in line.chars().enumerate() {
        interprint_each_char_as_tile(commands, c, x, y, tile_resource, &mut state);
    }
}
