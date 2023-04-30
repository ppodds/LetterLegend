use std::sync::Arc;

use std::sync::Mutex;

use crate::player::Player;

#[derive(Debug)]
pub struct LobbyPlayer {
    ready: Mutex<bool>,
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

    pub fn get_ready(&self) -> bool {
        *self.ready.lock().unwrap()
    }

    pub fn set_ready(&self, ready: bool) {
        *self.ready.lock().unwrap() = ready;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_ready_should_set_ready() {
        let player = Arc::new(Player::new(0, "test".to_string()));
        let lobby_player = LobbyPlayer::new(player.clone());
        lobby_player.set_ready(true);
        assert_eq!(lobby_player.get_ready(), true);
    }

    #[test]
    fn get_ready_should_get_ready() {
        let player = Arc::new(Player::new(0, "test".to_string()));
        let lobby_player = LobbyPlayer::new(player.clone());
        lobby_player.set_ready(true);
        assert_eq!(lobby_player.get_ready(), true);
    }
}
