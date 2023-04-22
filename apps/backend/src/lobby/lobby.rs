use crate::player::Player;
use std::error::Error;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Lobby {
    id: u32,
    max_players: u32,
    players: Arc<Mutex<HashMap<u32, Arc<Mutex<Player>>>>>,
}

impl Lobby {
    pub fn new(id: u32, max_players: u32) -> Self {
        debug_assert!(max_players >= 4, "max_players must be greater than 4");
        debug_assert!(max_players <= 8, "max_players must be less than 8");
        Self {
            id,
            max_players,
            players: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_player(
        &self,
        player: Arc<Mutex<Player>>,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        if player.lock().await.lobby_id.is_some() {
            return Err("player already in a lobby".into());
        }

        player.lock().await.lobby_id = Some(self.id);
        self.players
            .lock()
            .await
            .insert(player.lock().await.id, player.clone());
        Ok(())
    }

    pub async fn get_player(&self, id: u32) -> Option<Arc<Mutex<Player>>> {
        Some(self.players.lock().await.get(&id)?.clone())
    }

    pub async fn get_players(&self) -> Vec<Arc<Mutex<Player>>> {
        self.players.lock().await.values().cloned().collect()
    }

    pub async fn remove_player(&self, id: u32) -> Option<Arc<Mutex<Player>>> {
        let player = self.players.lock().await.remove(&id);

        if let Some(player) = player.clone() {
            player.lock().await.lobby_id = None;
        }

        player
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
        let lobby = Lobby::new(0, 4);
        assert_eq!(lobby.get_id(), 0);
        Ok(())
    }

    #[test]
    fn get_max_players_with_max_players_4_returns_4() -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        assert_eq!(lobby.get_max_players(), 4);
        Ok(())
    }

    #[tokio::test]
    async fn add_player_with_test_player_should_be_added() -> Result<(), Box<dyn std::error::Error>>
    {
        let lobby = Lobby::new(0, 4);
        let player = Arc::new(Mutex::new(Player::new(0, "test".to_string())));
        let result = lobby.add_player(player.clone()).await;
        assert!(result.is_ok());
        assert!(lobby.players.lock().await.get(&0).is_some());
        assert_eq!(player.lock().await.lobby_id.unwrap(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn add_player_with_test_player_already_in_lobby_should_return_error(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        let player = Arc::new(Mutex::new(Player::new(0, "test".to_string())));
        player.lock().await.lobby_id = Some(1);
        assert!(lobby.add_player(player).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn get_player_with_test_player_should_return_test_player(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        lobby
            .players
            .lock()
            .await
            .insert(0, Arc::new(Mutex::new(Player::new(0, "test".to_string()))));
        let t = lobby.get_player(0).await.unwrap();
        let player = t.lock().await;
        assert_eq!(player.id, 0);
        assert_eq!(player.name, "test");
        Ok(())
    }

    #[tokio::test]
    async fn get_player_with_not_exist_player_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        assert!(lobby.get_player(0).await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn get_players_with_three_players_should_return_players(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        for i in 0..3 {
            lobby.players.lock().await.insert(
                i,
                Arc::new(Mutex::new(Player::new(i, format!("test{}", i)))),
            );
        }

        for player in lobby.get_players().await {
            let player = player.lock().await;
            assert_eq!(player.name, format!("test{}", player.id));
        }
        Ok(())
    }

    #[tokio::test]
    async fn remove_player_with_test_player_should_remove_player(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        lobby.players.lock().await.insert(
            0,
            Arc::new(Mutex::new(Player::new(0, format!("test{}", 0)))),
        );

        let player = lobby.remove_player(0).await;
        assert_eq!(lobby.players.lock().await.len(), 0);
        assert!(player.unwrap().lock().await.lobby_id.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn remove_player_with_not_exist_player_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0, 4);
        assert!(lobby.remove_player(0).await.is_none());
        Ok(())
    }
}
