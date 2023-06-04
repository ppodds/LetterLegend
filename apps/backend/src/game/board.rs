use std::collections::HashSet;

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

    pub fn validate(&self, dict: &HashSet<String>) -> bool {
        let mut current_word: Option<String> = None;
        let mut is_horizontal_word_arr: [[bool; 26]; 26] = [[false; 26]; 26];
        for row in 0..26 {
            for col in 0..26 {
                match &self.tiles[row][col] {
                    Some(tile) => {
                        if current_word.is_some() {
                            let mut s = current_word.unwrap();
                            s.push(tile.char);
                            current_word = Some(s);
                        } else {
                            current_word = Some(tile.char.to_string())
                        }
                    }
                    None => match current_word {
                        Some(word) => {
                            if dict.contains(&word) {
                                for k in 1..word.len() + 1 {
                                    is_horizontal_word_arr[row][col - k] = true;
                                }
                            }
                            current_word = None;
                        }
                        None => (),
                    },
                }
                if col == 25 {
                    current_word = None;
                }
            }
        }
        current_word = None;
        for col in 0..26 {
            for row in 0..26 {
                match &self.tiles[row][col] {
                    Some(tile) => {
                        if current_word.is_some() {
                            let mut s = current_word.unwrap();
                            s.push(tile.char);
                            current_word = Some(s);
                        } else {
                            current_word = Some(tile.char.to_string())
                        }
                    }
                    None => match current_word {
                        Some(word) => {
                            let mut is_vertical_word = false;
                            if dict.contains(&word) {
                                is_vertical_word = true;
                                for k in 1..word.len() + 1 {
                                    is_horizontal_word_arr[row - k][col] = true;
                                }
                            }
                            let mut is_horizontal_word = true;
                            assert!(row > 0);
                            for k in 1..word.len() + 1 {
                                if !is_horizontal_word_arr[row - k][col] {
                                    is_horizontal_word = false;
                                }
                            }
                            if !is_vertical_word && !is_horizontal_word {
                                return false;
                            }
                            current_word = None;
                        }
                        None => (),
                    },
                }
                if row == 25 {
                    current_word = None;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, sync::Arc};

    use crate::player::Player;

    use super::*;

    #[test]
    fn validate_with_the_word_row_should_return_true() -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player, 1);
        board.tiles[0][0] = Some(t_tile);
        board.tiles[0][1] = Some(h_tile);
        board.tiles[0][2] = Some(e_tile);
        assert!(board.validate(&wordlist));
        Ok(())
    }

    #[test]
    fn validate_with_the_word_col_should_return_true() -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player, 1);
        board.tiles[0][0] = Some(t_tile);
        board.tiles[1][0] = Some(h_tile);
        board.tiles[2][0] = Some(e_tile);
        assert!(board.validate(&wordlist));
        Ok(())
    }

    #[test]
    fn validate_with_the_word_tilt_should_return_false() -> Result<(), Box<dyn Error + Sync + Send>>
    {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player, 1);
        board.tiles[0][0] = Some(t_tile);
        board.tiles[1][1] = Some(h_tile);
        board.tiles[2][2] = Some(e_tile);
        assert!(!board.validate(&wordlist));
        Ok(())
    }

    #[test]
    fn validate_with_the_word_exceed_horizontal_border_should_return_false(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player, 1);
        board.tiles[0][25] = Some(t_tile);
        board.tiles[1][0] = Some(h_tile);
        board.tiles[1][1] = Some(e_tile);
        assert!(!board.validate(&wordlist));
        Ok(())
    }

    #[test]
    fn validate_with_the_word_exceed_vertical_border_should_return_false(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player, 1);
        board.tiles[25][0] = Some(t_tile);
        board.tiles[0][1] = Some(h_tile);
        board.tiles[0][2] = Some(e_tile);
        assert!(!board.validate(&wordlist));
        Ok(())
    }

    #[test]
    fn validate_with_space_should_return_true() -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let board = Board::new();
        assert!(board.validate(&wordlist));
        Ok(())
    }
}
