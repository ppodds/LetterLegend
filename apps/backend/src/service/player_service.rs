#[cfg(not(test))]
use crate::frame::Frame;
use crate::player::Player;

use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};
#[cfg(not(test))]
use tokio::sync::mpsc::Sender;

use super::{game_service::GameService, lobby_service::LobbyService};

type ClientMap = Arc<Mutex<HashMap<u32, Arc<Player>>>>;

#[derive(Debug, Clone)]
pub struct PlayerService {
    online_player_map: ClientMap,
    lobby_service: Arc<LobbyService>,
    game_service: Arc<GameService>,
}

impl PlayerService {
    pub fn new(lobby_service: Arc<LobbyService>, game_service: Arc<GameService>) -> Self {
        Self {
            online_player_map: Arc::new(Mutex::new(HashMap::new())),
            lobby_service,
            game_service,
        }
    }

    pub fn get_player(&self, client_id: u32) -> Option<Arc<Player>> {
        match self.online_player_map.lock().unwrap().get(&client_id) {
            Some(player) => Some(player.clone()),
            None => None,
        }
    }

    pub fn add_player(
        &self,
        client_id: u32,
        name: String,
        #[cfg(not(test))] sender: Sender<Frame>,
    ) -> Arc<Player> {
        let player = Arc::new(Player::new(
            client_id,
            name,
            #[cfg(not(test))]
            sender,
        ));
        self.online_player_map
            .lock()
            .unwrap()
            .insert(client_id, player.clone());
        player
    }

    pub fn get_players(&self) -> Vec<Arc<Player>> {
        self.online_player_map
            .lock()
            .unwrap()
            .iter()
            .map(|x| x.1.clone())
            .collect()
    }

    pub fn remove_player(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<Player>, Box<dyn Error + Send + Sync>> {
        match self.online_player_map.lock().unwrap().remove(&player.id) {
            Some(player) => {
                if player.clone().get_lobby().is_some() {
                    println!("cleaning: remove player from lobby");
                    self.lobby_service
                        .remove_player_from_lobby(player.clone())?;
                };
                if player.clone().get_game().is_some() {
                    println!("cleaning: remove player from game");
                    self.game_service.remove_player_from_game(player.clone())?;
                };
                Ok(player)
            }
            None => Err("Player not found".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, error::Error};

    use super::*;

    #[tokio::test]
    async fn add_player_with_test_user_online_player_map_should_include_test_user(
    ) -> Result<(), Box<dyn Error>> {
        let service = PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new(HashSet::new())),
        );
        service.add_player(0, String::from("test"));
        assert!(service.online_player_map.lock().unwrap().get(&0).is_some());
        Ok(())
    }

    #[tokio::test]
    async fn get_players_with_a_player_in_online_player_map_should_return_a_vec_with_that_player(
    ) -> Result<(), Box<dyn Error>> {
        let service = PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new(HashSet::new())),
        );
        service
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        assert_eq!(service.get_players().len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn remove_player_with_a_player_in_lobby_should_remove_player_from_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let lobby_service = Arc::new(LobbyService::new());
        let service = PlayerService::new(
            lobby_service.clone(),
            Arc::new(GameService::new(HashSet::new())),
        );
        let player = service.add_player(0, String::from("test"));
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        service.remove_player(player.clone())?;
        assert_eq!(lobby.get_players().len(), 0);
        assert!(player.get_lobby().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn remove_player_with_a_player_in_game_should_remove_player_from_game(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let game_service = Arc::new(GameService::new(HashSet::new()));
        let lobby_service = Arc::new(LobbyService::new());
        let service = PlayerService::new(lobby_service.clone(), game_service.clone());
        let player = service.add_player(0, String::from("test"));
        let lobby = lobby_service.create_lobby(player.clone(), 4)?;
        lobby.get_player(player.id).unwrap().set_ready(true);
        let game = GameService::start_game(game_service, player.clone(), lobby)?;
        service.remove_player(player.clone())?;
        assert_eq!(game.get_players().len(), 0);
        assert!(player.get_game().is_none());
        Ok(())
    }

    #[tokio::test]
    async fn remove_player_with_a_player_not_existing_should_return_error(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = PlayerService::new(
            Arc::new(LobbyService::new()),
            Arc::new(GameService::new(HashSet::new())),
        );
        assert!(service
            .remove_player(Arc::new(Player::new(0, String::from("test"))))
            .is_err());
        Ok(())
    }
}
