use crate::player::Player;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Lobby {
    id: u32,
    players: Arc<Mutex<HashMap<u32, Arc<Mutex<Player>>>>>,
}

impl Lobby {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            players: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_player(&self, player: Arc<Mutex<Player>>) {
        self.players
            .lock()
            .await
            .insert(player.lock().await.id, player.clone());
    }

    pub async fn get_player(&self, id: u32) -> Option<Arc<Mutex<Player>>> {
        Some(self.players.lock().await.get(&id)?.clone())
    }

    pub async fn get_players(&self) -> Vec<Arc<Mutex<Player>>> {
        self.players.lock().await.values().cloned().collect()
    }

    pub async fn remove_player(&self, id: u32) -> Option<Arc<Mutex<Player>>> {
        self.players.lock().await.remove(&id)
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_id_with_id_0_returns_0() -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0);
        assert_eq!(lobby.get_id(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn add_player_with_test_player_should_be_added() -> Result<(), Box<dyn std::error::Error>>
    {
        let lobby = Lobby::new(0);
        lobby
            .add_player(Arc::new(Mutex::new(Player::new(0, "test".to_string()))))
            .await;
        assert!(lobby.players.lock().await.get(&0).is_some());
        Ok(())
    }

    #[tokio::test]
    async fn get_player_with_test_player_should_return_test_player(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0);
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
        let lobby = Lobby::new(0);
        assert!(lobby.get_player(0).await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn get_players_with_three_players_should_return_players(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0);
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
        let lobby = Lobby::new(0);
        lobby.players.lock().await.insert(
            0,
            Arc::new(Mutex::new(Player::new(0, format!("test{}", 0)))),
        );

        lobby.remove_player(0).await;
        assert_eq!(lobby.players.lock().await.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn remove_player_with_not_exist_player_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobby = Lobby::new(0);
        assert!(lobby.remove_player(0).await.is_none());
        Ok(())
    }
}
