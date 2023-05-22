use std::sync::Arc;

use crate::game::game_player::GamePlayer;

include!(concat!(env!("OUT_DIR"), "/player.players.rs"));

impl From<&Vec<Arc<GamePlayer>>> for Players {
    fn from(players: &Vec<Arc<GamePlayer>>) -> Self {
        Self {
            players: players
                .iter()
                .map(|x| super::player::Player::from(x.clone()))
                .collect(),
        }
    }
}
