use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::lobby::Lobby;

#[derive(Debug, Clone)]
pub struct Lobbies {
    next_lobby_id: u32,
    lobbies: Arc<Mutex<HashMap<u32, Arc<Mutex<Lobby>>>>>,
}

impl Lobbies {
    pub fn new() -> Self {
        Self {
            next_lobby_id: 0,
            lobbies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_lobby(&mut self, max_players: u32) -> Arc<Mutex<Lobby>> {
        let lobby = Arc::new(Mutex::new(Lobby::new(self.next_lobby_id, max_players)));
        let ret = lobby.clone();
        self.lobbies.lock().await.insert(self.next_lobby_id, lobby);
        self.next_lobby_id += 1;
        ret
    }

    pub async fn get_lobby(&self, id: u32) -> Option<Arc<Mutex<Lobby>>> {
        Some(self.lobbies.lock().await.get(&id)?.clone())
    }

    pub async fn remove_lobby(&self, id: u32) -> Option<Arc<Mutex<Lobby>>> {
        self.lobbies.lock().await.remove(&id)
    }

    pub async fn get_lobbies(&self) -> Vec<Arc<Mutex<Lobby>>> {
        self.lobbies.lock().await.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_lobby_should_create_lobby() -> Result<(), Box<dyn std::error::Error>> {
        let mut lobbies = Lobbies::new();
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
            .insert(0, Arc::new(Mutex::new(Lobby::new(0, 4))));
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
            .insert(0, Arc::new(Mutex::new(Lobby::new(0, 4))));
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
