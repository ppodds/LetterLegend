use crate::player::Player;
use std::error::Error;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use super::lobby_player::LobbyPlayer;

#[derive(Debug, Clone)]
pub struct Lobby {
    id: u32,
    max_players: u32,
    players: Arc<Mutex<HashMap<u32, Arc<LobbyPlayer>>>>,
    pub leader: Arc<Player>,
}

impl PartialEq for Lobby {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Lobby {
    pub fn new(id: u32, max_players: u32, leader: Arc<Player>) -> Self {
        debug_assert!(max_players >= 4, "max_players must be greater than 4");
        debug_assert!(max_players <= 8, "max_players must be less than 8");
        Self {
            id,
            max_players,
            players: Arc::new(Mutex::new(HashMap::from([(
                leader.id,
                Arc::new(LobbyPlayer::new(leader.clone())),
            )]))),
            leader,
        }
    }

    pub fn add_player(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<LobbyPlayer>, Box<dyn Error + Send + Sync>> {
        if self.players.lock().unwrap().contains_key(&player.id) {
            return Err("Player already in lobby".into());
        }

        let lobby_player = Arc::new(LobbyPlayer::new(player.clone()));
        self.players
            .lock()
            .unwrap()
            .insert(player.id, lobby_player.clone());
        Ok(lobby_player)
    }

    pub fn get_player(&self, id: u32) -> Option<Arc<LobbyPlayer>> {
        Some(self.players.lock().unwrap().get(&id)?.clone())
    }

    pub fn get_players(&self) -> Vec<Arc<LobbyPlayer>> {
        self.players.lock().unwrap().values().cloned().collect()
    }

    pub fn remove_player(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<LobbyPlayer>, Box<dyn Error + Send + Sync>> {
        if !self.players.lock().unwrap().contains_key(&player.id) {
            return Err("Player is not in the lobby".into());
        }
        Ok(self.players.lock().unwrap().remove(&player.id).unwrap())
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_max_players(&self) -> u32 {
        self.max_players
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_id_with_id_0_returns_0() -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4, Arc::new(Player::new(0, "test".to_string())));
        assert_eq!(lobby.get_id(), 0);
        Ok(())
    }

    #[test]
    fn get_max_players_with_max_players_4_returns_4() -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4, Arc::new(Player::new(0, "test".to_string())));
        assert_eq!(lobby.get_max_players(), 4);
        Ok(())
    }

    #[test]
    fn add_player_with_test_player_should_be_added() -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4, Arc::new(Player::new(0, "test1".to_string())));
        let player = Arc::new(Player::new(1, "test2".to_string()));
        let result = lobby.add_player(player.clone());
        assert!(result.is_ok());
        assert!(lobby.players.lock().unwrap().get(&0).is_some());
        Ok(())
    }

    #[test]
    fn add_player_with_test_player_already_in_lobby_should_return_error(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let player = Arc::new(Player::new(0, "test1".to_string()));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        assert!(lobby.add_player(player).is_err());
        Ok(())
    }

    #[test]
    fn get_player_with_test_player_should_return_test_player(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4, Arc::new(Player::new(0, "test".to_string())));
        lobby.players.lock().unwrap().insert(
            0,
            Arc::new(LobbyPlayer::new(Arc::new(Player::new(
                0,
                "test".to_string(),
            )))),
        );
        let player = lobby.get_player(0).unwrap().player.clone();
        assert_eq!(player.id, 0);
        assert_eq!(player.name, "test");
        Ok(())
    }

    #[test]
    fn get_player_with_not_exist_player_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4, Arc::new(Player::new(0, "test".to_string())));
        assert!(lobby.get_player(1).is_none());
        Ok(())
    }

    #[test]
    fn get_players_with_three_players_should_return_players(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4, Arc::new(Player::new(0, "test".to_string())));
        for i in 0..3 {
            lobby.players.lock().unwrap().insert(
                i,
                Arc::new(LobbyPlayer::new(Arc::new(Player::new(
                    i,
                    format!("test{}", i),
                )))),
            );
        }

        for lobby_player in lobby.get_players() {
            assert_eq!(
                lobby_player.player.name,
                format!("test{}", lobby_player.player.id)
            );
        }
        Ok(())
    }

    #[test]
    fn remove_player_with_test_player_should_remove_player(
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let player = Arc::new(Player::new(0, "test".to_string()));
        let lobby = Arc::new(Lobby::new(0, 4, player.clone()));
        lobby.remove_player(player)?;
        assert_eq!(lobby.players.lock().unwrap().len(), 0);
        Ok(())
    }
}
