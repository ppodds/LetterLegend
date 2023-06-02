use std::sync::Arc;

use crate::player::Player;

#[derive(Debug, Clone)]
pub struct Tile {
    pub char: char,
    pub owner: Arc<Player>,
    pub turn: u32,
}

impl Tile {
    pub fn new(char: char, owner: Arc<Player>, turn: u32) -> Self {
        Self { char, owner, turn }
    }
}
