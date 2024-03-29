include!(concat!(env!("OUT_DIR"), "/game.tile.rs"));

impl From<&crate::game::tile::Tile> for Tile {
    fn from(tile: &crate::game::tile::Tile) -> Self {
        Self {
            char: String::from(tile.char),
            owner: tile.owner.id,
        }
    }
}
