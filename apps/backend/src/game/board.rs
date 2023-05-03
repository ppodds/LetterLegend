use super::tile::Tile;

#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: [[Option<Tile>; 26]; 26],
}

impl Board {
    pub fn new() -> Self {
        // workaround
        const INIT: Option<Tile> = None;
        const ARR: [Option<Tile>; 26] = [INIT; 26];
        Self { tiles: [ARR; 26] }
    }

    pub fn validate() -> bool {
        todo!()
    }
}
