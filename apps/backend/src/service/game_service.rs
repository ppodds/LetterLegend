use std::{error::Error, sync::Arc};

use crate::{
    game::{game::Game, games::Games},
    player::Player,
};

#[derive(Debug, Clone)]
pub struct GameService {
    games: Arc<Games>,
}

impl GameService {
    pub fn new() -> Self {
        Self {
            games: Arc::new(Games::new()),
        }
    }

    pub fn start_game(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<Game>, Box<dyn Error + Send + Sync>> {
        let lobby = match player.get_lobby() {
            Some(lobby) => lobby,
            None => return Err("Player not in lobby".into()),
        };

        if player != lobby.leader {
            return Err("Only leader can start game".into());
        }

        Ok(self.games.create_game(
            lobby
                .get_players()
                .iter()
                .map(|x| x.player.clone())
                .collect(),
        ))
    }

    pub fn get_game(&self, id: u32) -> Option<Arc<Game>> {
        self.games.get_game(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::service::lobby_service::LobbyService;

    use super::*;

    #[test]
    fn start_game_with_test_player_in_test_lobby_should_start_game(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new();
        let lobby_service = LobbyService::new();
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = lobby_service.create_lobby(leader.clone(), 4)?;
        lobby.get_player(leader.id).unwrap().set_ready(true);
        assert!(game_service.start_game(leader.clone()).is_ok());
        Ok(())
    }

    #[test]
    fn start_game_with_test_player_not_in_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new();
        assert!(game_service
            .start_game(Arc::new(Player::new(1, "test2".to_string())))
            .is_err());
        Ok(())
    }

    #[test]
    fn start_game_with_not_leader_should_return_error() -> Result<(), Box<dyn Error + Send + Sync>>
    {
        let game_service = GameService::new();
        let lobby_service = LobbyService::new();
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = lobby_service.create_lobby(leader.clone(), 4)?;
        let player = Arc::new(Player::new(1, "test2".to_string()));
        lobby_service.add_player_to_lobby(player.clone(), lobby)?;
        assert!(game_service.start_game(player).is_err());
        Ok(())
    }

    #[test]
    fn get_game_with_game_id_should_return_game() -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = GameService::new();
        let lobby_service = LobbyService::new();
        let leader = Arc::new(Player::new(0, "test".to_string()));
        let lobby = lobby_service.create_lobby(leader.clone(), 4)?;
        lobby.get_player(leader.id).unwrap().set_ready(true);
        game_service.start_game(leader.clone())?;
        assert!(game_service.get_game(0).is_some());
        Ok(())
    }
}
