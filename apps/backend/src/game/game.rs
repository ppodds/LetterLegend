use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::board::Board;
use crate::player::Player;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    players: Mutex<HashMap<u32, Arc<Player>>>,
    board: Arc<Mutex<Board>>,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Game {
    pub fn new(id: u32, players: Vec<Arc<Player>>) -> Self {
        let mut map = HashMap::new();
        for player in players {
            map.insert(player.id, player.clone());
        }
        Self {
            id,
            players: Mutex::new(map),
            board: Arc::new(Mutex::new(Board::new())),
        }
    }

    pub fn get_board(&self) -> Arc<Mutex<Board>> {
        self.board.clone()
    }

    pub fn remove_player(&self, player: Arc<Player>) -> Option<Arc<Player>> {
        self.players.lock().unwrap().remove(&player.id)
    }

    pub fn get_players(&self) -> Vec<Arc<Player>> {
        self.players
            .lock()
            .unwrap()
            .values()
            .map(|player| player.clone())
            .collect()
    }
}
