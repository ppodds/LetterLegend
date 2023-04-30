use std::sync::Arc;

use std::sync::Mutex;

use crate::player::Player;

#[derive(Debug)]
pub struct LobbyPlayer {
    pub ready: Mutex<bool>,
    pub player: Arc<Player>,
}

impl PartialEq for LobbyPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.player.id == other.player.id
    }
}

impl LobbyPlayer {
    pub fn new(player: Arc<Player>) -> Self {
        Self {
            ready: Mutex::new(false),
            player,
        }
    }
}
