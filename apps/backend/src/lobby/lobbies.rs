use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::lobby::Lobby;

#[derive(Debug, Clone)]
pub struct Lobbies {
    next_lobby_id: Arc<Mutex<u32>>,
    lobbies: Arc<Mutex<HashMap<u32, Arc<Lobby>>>>,
}

impl Lobbies {
    pub fn new() -> Self {
        Self {
            next_lobby_id: Arc::new(Mutex::new(0)),
            lobbies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_lobby(&self, max_players: u32) -> Arc<Lobby> {
        let mut next_lobby_id = self.next_lobby_id.lock().await;
        let lobby = Arc::new(Lobby::new(*next_lobby_id, max_players));
        self.lobbies
            .lock()
            .await
            .insert(*next_lobby_id, lobby.clone());
        *next_lobby_id += 1;
        lobby
    }

    pub async fn get_lobby(&self, id: u32) -> Option<Arc<Lobby>> {
        Some(self.lobbies.lock().await.get(&id)?.clone())
    }

    pub async fn remove_lobby(&self, id: u32) -> Option<Arc<Lobby>> {
        self.lobbies.lock().await.remove(&id)
    }

    pub async fn get_lobbies(&self) -> Vec<Arc<Lobby>> {
        self.lobbies.lock().await.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_lobby_should_create_lobby() -> Result<(), Box<dyn std::error::Error>> {
        let lobbies = Lobbies::new();
        lobbies.create_lobby(4).await;
        assert!(lobbies.lobbies.lock().await.get(&0).is_some());
        Ok(())
    }

    #[tokio::test]
    async fn get_lobby_with_test_lobby_should_return_lobby(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobbies = Lobbies::new();
        lobbies
            .lobbies
            .lock()
            .await
            .insert(0, Arc::new(Lobby::new(0, 4)));
        assert!(lobbies.get_lobby(0).await.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn get_lobby_with_not_exist_lobby_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobbies = Lobbies::new();
        assert!(lobbies.get_lobby(0).await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn remove_lobby_with_test_lobby_should_remove_lobby(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobbies = Lobbies::new();
        lobbies
            .lobbies
            .lock()
            .await
            .insert(0, Arc::new(Lobby::new(0, 4)));
        lobbies.remove_lobby(0).await;
        assert_eq!(lobbies.lobbies.lock().await.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn remove_lobby_with_not_exist_lobby_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobbies = Lobbies::new();
        assert!(lobbies.remove_lobby(0).await.is_none());
        Ok(())
    }
}
