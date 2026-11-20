mod bitboard;
mod board;

use bevy::{prelude::*};
use crate::board::SetupDefaultBoard;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupDefaultBoard)
        .run();
}

