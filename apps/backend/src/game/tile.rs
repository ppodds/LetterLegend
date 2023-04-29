use std::sync::Arc;

use crate::player::Player;

#[derive(Debug, Clone)]
pub struct Tile {
    pub char: String,
    pub owner: Arc<Player>,
}

impl Tile {
    pub fn new(char: String, owner: Arc<Player>) -> Self {
        Self { char, owner }
    }
}
