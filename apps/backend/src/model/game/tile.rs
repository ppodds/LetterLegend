include!(concat!(env!("OUT_DIR"), "/game.tile.rs"));

impl From<&crate::game::tile::Tile> for Tile {
    fn from(tile: &crate::game::tile::Tile) -> Self {
        Self {
            char: tile.char.clone(),
            owner: tile.owner.id,
        }
    }
}
