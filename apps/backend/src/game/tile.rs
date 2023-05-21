use std::sync::Arc;

use crate::player::Player;

#[derive(Debug, Clone)]
pub struct Tile {
    pub char: char,
    pub owner: Arc<Player>,
}

impl Tile {
    pub fn new(char: char, owner: Arc<Player>) -> Self {
        Self { char, owner }
    }
}
