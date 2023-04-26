use std::sync::Arc;

use tokio::sync::Mutex;

use crate::player::Player;

#[derive(Clone, Debug)]
pub struct LobbyPlayer {
    pub ready: Arc<Mutex<bool>>,
    pub player: Arc<Player>,
}

impl LobbyPlayer {
    pub fn new(player: Arc<Player>) -> Self {
        Self {
            ready: Arc::new(Mutex::new(false)),
            player,
        }
    }
}
