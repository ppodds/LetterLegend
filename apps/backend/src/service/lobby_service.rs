use std::{error::Error, sync::Arc};

use crate::{
    lobby::{lobbies::Lobbies, lobby::Lobby, lobby_player::LobbyPlayer},
    player::Player,
};

#[derive(Debug, Clone)]
pub struct LobbyService {
    lobbies: Arc<Lobbies>,
}

impl LobbyService {
    pub fn new() -> Self {
        Self {
            lobbies: Arc::new(Lobbies::new()),
        }
    }

    pub fn create_lobby(
        &self,
        leader: Arc<Player>,
        max_players: u32,
    ) -> Result<Arc<Lobby>, Box<dyn Error + Send + Sync + Send + Sync>> {
        if max_players < 4 || max_players > 8 {
            return Err("Invalid max players".into());
        }
        let lobby = self.lobbies.create_lobby(max_players, leader.clone());
        leader.set_lobby(Some(lobby.clone()));
        Ok(lobby)
    }

    pub fn add_player_to_lobby(
        &self,
        player: Arc<Player>,
        lobby: Arc<Lobby>,
    ) -> Result<Arc<LobbyPlayer>, Box<dyn Error + Send + Sync>> {
        let lobby_player = lobby.add_player(player.clone())?;
        player.set_lobby(Some(lobby));
        Ok(lobby_player)
    }

    pub fn get_lobbies(&self) -> Vec<Arc<Lobby>> {
        self.lobbies.get_lobbies()
    }

    pub fn get_lobby(&self, id: u32) -> Option<Arc<Lobby>> {
        self.lobbies.get_lobby(id)
    }

    pub fn remove_player_from_lobby(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<LobbyPlayer>, Box<dyn Error + Send + Sync>> {
        let lobby_player = match player.clone().get_lobby() {
            Some(lobby) => lobby,
            None => return Err("Player is not in a lobby".into()),
        }
        .remove_player(player.clone())?;
        player.set_lobby(None);
        Ok(lobby_player)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_lobby_with_test_user_should_create_lobby() -> Result<(), Box<dyn Error + Send + Sync>>
    {
        let service = LobbyService::new();
        service.create_lobby(Arc::new(Player::new(0, String::from("test"))), 4)?;
        assert!(service.lobbies.get_lobby(0).is_some());
        Ok(())
    }

    #[test]
    fn create_lobby_with_test_user_and_invaild_max_players_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        let leader = Arc::new(Player::new(0, String::from("test")));
        assert!(service.create_lobby(leader.clone(), 3).is_err());
        assert!(service.create_lobby(leader, 9).is_err());
        Ok(())
    }

    #[test]
    fn create_lobby_with_test_user_should_contains_test_user(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        let leader = Arc::new(Player::new(0, String::from("test")));
        service.create_lobby(leader, 4)?;
        assert!(service
            .lobbies
            .get_lobby(0)
            .unwrap()
            .get_player(0)
            .is_some());
        Ok(())
    }

    #[test]
    fn add_player_to_lobby_with_test_user_and_test_lobby_should_add_player_to_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        let leader = Arc::new(Player::new(0, String::from("test")));
        let lobby = service.lobbies.create_lobby(4, leader.clone());
        service.add_player_to_lobby(Arc::new(Player::new(1, String::from("test2"))), lobby)?;
        assert!(service
            .lobbies
            .get_lobby(0)
            .unwrap()
            .get_player(1)
            .is_some());
        Ok(())
    }

    #[test]
    fn add_player_to_lobby_with_test_user_and_test_lobby_player_lobby_should_be_test_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        let leader = Arc::new(Player::new(0, String::from("test")));
        let lobby = service.lobbies.create_lobby(4, leader.clone());
        let player = Arc::new(Player::new(1, String::from("test2")));
        service.add_player_to_lobby(player.clone(), lobby.clone())?;
        assert_eq!(player.get_lobby().unwrap(), lobby);
        Ok(())
    }

    #[test]
    fn remove_player_from_lobby_with_test_user_in_test_lobby_should_quit_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        let leader = Arc::new(Player::new(0, String::from("test")));
        service.create_lobby(leader.clone(), 4)?;
        service.remove_player_from_lobby(leader)?;
        assert!(service
            .lobbies
            .get_lobby(0)
            .unwrap()
            .get_player(0)
            .is_none());
        Ok(())
    }

    #[test]
    fn get_lobbies_with_test_looby_should_return_test_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        service
            .lobbies
            .create_lobby(4, Arc::new(Player::new(0, String::from("test"))));
        let lobbies = service.get_lobbies();
        assert_eq!(lobbies.len(), 1);
        Ok(())
    }
}
