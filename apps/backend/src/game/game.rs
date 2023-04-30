use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::tile::Tile;
use crate::player::Player;

pub type Board = [[Option<Tile>; 26]; 26];

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub players: Mutex<HashMap<u32, Arc<Player>>>,
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
        // workaround
        const INIT: Option<Tile> = None;
        const ARR: [Option<Tile>; 26] = [INIT; 26];
        Self {
            id,
            players: Mutex::new(map),
            board: Arc::new(Mutex::new([ARR; 26])),
        }
    }

    pub fn get_board(&self) -> Arc<Mutex<Board>> {
        self.board.clone()
    }

    pub fn remove_player(&self, player: Arc<Player>) -> Option<Arc<Player>> {
        self.players.lock().unwrap().remove(&player.id)
    }
}
