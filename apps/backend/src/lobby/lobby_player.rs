use std::sync::Arc;

use tokio::sync::Mutex;

use crate::player::Player;

#[derive(Clone, Debug)]
pub struct LobbyPlayer {
    pub ready: bool,
    pub player: Arc<Mutex<Player>>,
}

impl LobbyPlayer {
    pub fn new(player: Arc<Mutex<Player>>) -> Self {
        Self {
            ready: false,
            player,
        }
    }
}
