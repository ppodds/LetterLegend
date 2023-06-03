use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{
    lobby::{lobby::Lobby, lobby_player::LobbyPlayer},
    player::Player,
};

#[cfg(not(test))]
use crate::frame::{Response, ResponseData};
#[cfg(not(test))]
use crate::model::lobby::broadcast::{LobbyBroadcast, LobbyEvent};
#[cfg(not(test))]
use crate::model::state::State;

#[derive(Debug)]
pub struct LobbyService {
    next_lobby_id: Mutex<u32>,
    lobbies: Mutex<HashMap<u32, Arc<Lobby>>>,
}

impl LobbyService {
    pub fn new() -> Self {
        Self {
            next_lobby_id: Mutex::new(0),
            lobbies: Mutex::new(HashMap::new()),
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
        let mut next_lobby_id = self.next_lobby_id.lock().unwrap();
        let lobby = Arc::new(Lobby::new(*next_lobby_id, max_players, leader.clone()));
        self.lobbies
            .lock()
            .unwrap()
            .insert(*next_lobby_id, lobby.clone());
        *next_lobby_id += 1;
        leader.set_lobby(Some(lobby.clone()));
        Ok(lobby)
    }

    pub fn add_player_to_lobby(
        &self,
        player: Arc<Player>,
        lobby: Arc<Lobby>,
    ) -> Result<Arc<LobbyPlayer>, Box<dyn Error + Send + Sync>> {
        if player.get_lobby().is_some() {
            return Err("player already in a lobby".into());
        }
        let lobby_player = lobby.add_player(player.clone())?;
        #[cfg(not(test))]
        {
            for lobby_player in lobby.get_players() {
                if lobby_player.player == player {
                    continue;
                }
                let lobby = lobby.clone();
                tokio::spawn(async move {
                    if let Err(e) = lobby_player
                        .player
                        .send_message(Response::new(
                            State::LobbyBroadcast as u32,
                            Arc::new(ResponseData::LobbyBroadcast(LobbyBroadcast {
                                event: LobbyEvent::Join as i32,
                                lobby: Some(crate::model::lobby::lobby::Lobby::from(lobby.clone())),
                                cards: None,
                                current_player: None,
                                next_player: None,
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending lobby broadcast: {}", e);
                    }
                });
            }
        }
        player.set_lobby(Some(lobby));
        Ok(lobby_player)
    }

    pub fn get_lobbies(&self) -> Vec<Arc<Lobby>> {
        self.lobbies.lock().unwrap().values().cloned().collect()
    }

    pub fn get_lobby(&self, id: u32) -> Option<Arc<Lobby>> {
        Some(self.lobbies.lock().unwrap().get(&id)?.clone())
    }

    pub fn remove_player_from_lobby(
        &self,
        player: Arc<Player>,
    ) -> Result<Arc<LobbyPlayer>, Box<dyn Error + Send + Sync>> {
        let lobby = match player.clone().get_lobby() {
            Some(lobby) => lobby,
            None => return Err("Player is not in a lobby".into()),
        };
        let lobby_player = lobby.remove_player(player.clone())?;
        let is_lobby_destroy = player == lobby.leader;
        #[cfg(not(test))]
        {
            for lobby_player in lobby.get_players() {
                let lobby = lobby.clone();
                tokio::spawn(async move {
                    if let Err(e) = lobby_player
                        .player
                        .send_message(Response::new(
                            State::LobbyBroadcast as u32,
                            Arc::new(ResponseData::LobbyBroadcast(LobbyBroadcast {
                                event: match is_lobby_destroy {
                                    true => LobbyEvent::Destroy as i32,
                                    false => LobbyEvent::Leave as i32,
                                },
                                lobby: Some(crate::model::lobby::lobby::Lobby::from(lobby)),
                                cards: None,
                                current_player: None,
                                next_player: None,
                            })),
                        ))
                        .await
                    {
                        eprintln!("Error sending lobby broadcast: {}", e);
                    }
                });
            }
        }
        player.set_lobby(None);
        if is_lobby_destroy {
            self.remove_lobby(lobby)?;
        }
        Ok(lobby_player)
    }

    pub fn remove_lobby(
        &self,
        lobby: Arc<Lobby>,
    ) -> Result<Arc<Lobby>, Box<dyn Error + Send + Sync>> {
        let mut lobbies = self.lobbies.lock().unwrap();
        if !lobbies.contains_key(&lobby.get_id()) {
            return Err("Lobby does not exist".into());
        }
        Ok(lobbies.remove(&lobby.get_id()).unwrap())
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
        assert!(service.lobbies.lock().unwrap().get(&0).is_some());
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
            .lock()
            .unwrap()
            .get(&0)
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
        let lobby = Arc::new(Lobby::new(0, 4, leader.clone()));
        service.lobbies.lock().unwrap().insert(0, lobby.clone());
        service.add_player_to_lobby(Arc::new(Player::new(1, String::from("test2"))), lobby)?;
        assert!(service
            .lobbies
            .lock()
            .unwrap()
            .get(&0)
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
        let lobby = Arc::new(Lobby::new(0, 4, leader.clone()));
        service.lobbies.lock().unwrap().insert(0, lobby.clone());
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
        let lobby = service.create_lobby(leader.clone(), 4)?;
        let other_player = Arc::new(Player::new(1, String::from("test1")));
        service.add_player_to_lobby(other_player.clone(), lobby)?;
        service.remove_player_from_lobby(other_player)?;
        assert!(service
            .lobbies
            .lock()
            .unwrap()
            .get(&0)
            .unwrap()
            .get_player(1)
            .is_none());
        Ok(())
    }

    #[test]
    fn remove_player_from_lobby_with_leader_in_test_lobby_should_destroy_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        let leader = Arc::new(Player::new(0, String::from("test")));
        service.create_lobby(leader.clone(), 4)?;
        service.remove_player_from_lobby(leader)?;
        assert!(service.lobbies.lock().unwrap().get(&0).is_none());
        Ok(())
    }

    #[test]
    fn remove_player_from_lobby_with_leader_should_destroy_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        let leader = Arc::new(Player::new(0, String::from("test")));
        service.create_lobby(leader.clone(), 4)?;
        service.remove_player_from_lobby(leader)?;
        assert!(service.lobbies.lock().unwrap().is_empty());
        Ok(())
    }

    #[test]
    fn get_lobbies_with_test_looby_should_return_test_lobby(
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let service = LobbyService::new();
        service.lobbies.lock().unwrap().insert(
            0,
            Arc::new(Lobby::new(
                0,
                4,
                Arc::new(Player::new(0, String::from("test"))),
            )),
        );
        let lobbies = service.get_lobbies();
        assert_eq!(lobbies.len(), 1);
        Ok(())
    }

    #[test]
    fn remove_lobby_with_test_lobby_should_remove_lobby() -> Result<(), Box<dyn Error + Send + Sync>>
    {
        let service = LobbyService::new();
        let lobby = Arc::new(Lobby::new(
            0,
            4,
            Arc::new(Player::new(0, String::from("test"))),
        ));
        service.lobbies.lock().unwrap().insert(0, lobby.clone());
        service.remove_lobby(lobby)?;
        assert_eq!(service.lobbies.lock().unwrap().len(), 0);
        Ok(())
    }
}
