use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use super::lobby::Lobby;

#[derive(Debug)]
pub struct Lobbies {
    next_lobby_id: Mutex<u32>,
    lobbies: Arc<Mutex<HashMap<u32, Arc<Lobby>>>>,
}

impl Lobbies {
    pub fn new() -> Self {
        Self {
            next_lobby_id: Mutex::new(0),
            lobbies: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_lobby(&self, max_players: u32) -> Arc<Lobby> {
        let mut next_lobby_id = self.next_lobby_id.lock().unwrap();
        let lobby = Arc::new(Lobby::new(*next_lobby_id, max_players));
        self.lobbies
            .lock()
            .unwrap()
            .insert(*next_lobby_id, lobby.clone());
        *next_lobby_id += 1;
        lobby
    }

    pub fn get_lobby(&self, id: u32) -> Option<Arc<Lobby>> {
        Some(self.lobbies.lock().unwrap().get(&id)?.clone())
    }

    pub fn remove_lobby(&self, id: u32) -> Option<Arc<Lobby>> {
        self.lobbies.lock().unwrap().remove(&id)
    }

    pub fn get_lobbies(&self) -> Vec<Arc<Lobby>> {
        self.lobbies.lock().unwrap().values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_lobby_should_create_lobby() -> Result<(), Box<dyn std::error::Error>> {
        let lobbies = Lobbies::new();
        lobbies.create_lobby(4);
        assert!(lobbies.lobbies.lock().unwrap().get(&0).is_some());
        Ok(())
    }

    #[test]
    fn get_lobby_with_test_lobby_should_return_lobby() -> Result<(), Box<dyn std::error::Error>> {
        let lobbies = Lobbies::new();
        lobbies
            .lobbies
            .lock()
            .unwrap()
            .insert(0, Arc::new(Lobby::new(0, 4)));
        assert!(lobbies.get_lobby(0).is_some());
        Ok(())
    }

    #[test]
    fn get_lobby_with_not_exist_lobby_should_return_none() -> Result<(), Box<dyn std::error::Error>>
    {
        let lobbies = Lobbies::new();
        assert!(lobbies.get_lobby(0).is_none());
        Ok(())
    }

    #[test]
    fn remove_lobby_with_test_lobby_should_remove_lobby() -> Result<(), Box<dyn std::error::Error>>
    {
        let lobbies = Lobbies::new();
        lobbies
            .lobbies
            .lock()
            .unwrap()
            .insert(0, Arc::new(Lobby::new(0, 4)));
        lobbies.remove_lobby(0);
        assert_eq!(lobbies.lobbies.lock().unwrap().len(), 0);
        Ok(())
    }

    #[test]
    fn remove_lobby_with_not_exist_lobby_should_return_none(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let lobbies = Lobbies::new();
        assert!(lobbies.remove_lobby(0).is_none());
        Ok(())
    }
}
