use bevy::prelude::*;
use crate::{Enemy, new_player, Player};

/// Keeps track of game state and loads levels

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {

    }
}

/// End game and store state when enemies = 0 or players healths = 0
fn watch_state_sys(mut cmd: Commands, enemies: Query<&Enemy>, players: Query<&Player>){
    if enemies.is_empty() || players.is_empty() {
        new_player(&mut cmd);
        //todo create enemies

    }
}
