use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::player::Player;

use super::game::Game;

#[derive(Debug)]
pub struct Games {
    next_game_id: Mutex<u32>,
    games: Mutex<HashMap<u32, Arc<Game>>>,
}

impl Games {
    pub fn new() -> Self {
        Self {
            next_game_id: Mutex::new(0),
            games: Mutex::new(HashMap::new()),
        }
    }

    pub fn create_game(&self, players: Vec<Arc<Player>>) -> Arc<Game> {
        let mut next_id = self.next_game_id.lock().unwrap();
        let game = Arc::new(Game::new(*next_id, players));
        self.games.lock().unwrap().insert(*next_id, game.clone());
        *next_id += 1;
        game
    }

    pub fn get_gamees(&self) -> Vec<Arc<Game>> {
        self.games.lock().unwrap().values().cloned().collect()
    }

    pub fn get_game(&self, id: u32) -> Option<Arc<Game>> {
        Some(self.games.lock().unwrap().get(&id)?.clone())
    }

    pub fn remove_game(&self, id: u32) -> Option<Arc<Game>> {
        self.games.lock().unwrap().remove(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game_with_test_player_should_create_game() -> Result<(), Box<dyn std::error::Error>> {
        let games = Games::new();
        games.create_game(vec![Arc::new(Player::new(0, String::from("test")))]);
        assert!(games.games.lock().unwrap().get(&0).is_some());
        Ok(())
    }

    #[test]
    fn get_game_with_test_game_should_return_game() -> Result<(), Box<dyn std::error::Error>> {
        let games = Games::new();
        games
            .games
            .lock()
            .unwrap()
            .insert(0, Arc::new(Game::new(0, Vec::new())));
        assert!(games.get_game(0).is_some());
        Ok(())
    }

    #[test]
    fn get_game_with_not_exist_game_should_return_none() -> Result<(), Box<dyn std::error::Error>> {
        let games = Games::new();
        assert!(games.get_game(0).is_none());
        Ok(())
    }

    #[test]
    fn remove_game_with_test_game_should_remove_game() -> Result<(), Box<dyn std::error::Error>> {
        let games = Games::new();
        games
            .games
            .lock()
            .unwrap()
            .insert(0, Arc::new(Game::new(0, Vec::new())));
        games.remove_game(0);
        assert_eq!(games.games.lock().unwrap().len(), 0);
        Ok(())
    }

    #[test]
    fn remove_game_with_not_exist_game_should_return_none() -> Result<(), Box<dyn std::error::Error>>
    {
        let games = Games::new();
        assert!(games.remove_game(0).is_none());
        Ok(())
    }
}
