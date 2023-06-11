use std::{collections::HashSet, sync::Arc};

use super::{game::Game, tile::Tile};

pub const BOARD_SIZE: usize = 26;

#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: [[Option<Tile>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        // workaround
        const INIT: Option<Tile> = None;
        const ARR: [Option<Tile>; BOARD_SIZE] = [INIT; BOARD_SIZE];
        Self {
            tiles: [ARR; BOARD_SIZE],
        }
    }

    pub fn validate(&self, dict: &HashSet<String>, game: Arc<Game>) -> Option<Vec<String>> {
        let mut current_word: Option<String> = None;
        let mut is_horizontal_word_arr = [[false; BOARD_SIZE]; BOARD_SIZE];
        let mut words = Vec::new();
        let mut word_in_this_turn = false;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                match &self.tiles[row][col] {
                    Some(tile) => {
                        if current_word.is_some() {
                            let mut s = current_word.unwrap();
                            s.push(tile.char);
                            current_word = Some(s);
                        } else {
                            current_word = Some(tile.char.to_string())
                        }
                        if tile.owner == game.get_player_in_this_turn().player {
                            word_in_this_turn = true;
                        }
                    }
                    None => match current_word {
                        Some(word) => {
                            let len = word.len();
                            if dict.contains(&word) {
                                if word_in_this_turn {
                                    words.push(word);
                                }
                                for k in 1..len + 1 {
                                    is_horizontal_word_arr[row][col - k] = true;
                                }
                            }
                            word_in_this_turn = false;
                            current_word = None;
                        }
                        None => (),
                    },
                }
                if col == BOARD_SIZE - 1 && current_word.is_some() {
                    let word = current_word.unwrap();
                    if dict.contains(&word) {
                        let len = word.len();
                        if word_in_this_turn {
                            words.push(word);
                        }
                        for k in 0..len {
                            is_horizontal_word_arr[row][col - k] = true;
                        }
                    }
                    word_in_this_turn = false;
                    current_word = None;
                }
            }
        }
        word_in_this_turn = false;
        current_word = None;
        for col in 0..BOARD_SIZE {
            for row in 0..BOARD_SIZE {
                match &self.tiles[row][col] {
                    Some(tile) => {
                        if current_word.is_some() {
                            let mut s = current_word.unwrap();
                            s.push(tile.char);
                            current_word = Some(s);
                        } else {
                            current_word = Some(tile.char.to_string())
                        }
                        if tile.owner == game.get_player_in_this_turn().player {
                            word_in_this_turn = true;
                        }
                    }
                    None => match current_word {
                        Some(word) => {
                            let mut is_vertical_word = false;
                            let len = word.len();
                            if dict.contains(&word) {
                                if word_in_this_turn {
                                    words.push(word);
                                }
                                is_vertical_word = true;
                                for k in 1..len + 1 {
                                    is_horizontal_word_arr[row - k][col] = true;
                                }
                            }
                            let mut is_horizontal_word = true;
                            assert!(row > 0);
                            for k in 1..len + 1 {
                                if !is_horizontal_word_arr[row - k][col] {
                                    is_horizontal_word = false;
                                }
                            }
                            if !is_vertical_word && !is_horizontal_word {
                                return None;
                            }
                            word_in_this_turn = false;
                            current_word = None;
                        }
                        None => (),
                    },
                }
                if row == BOARD_SIZE - 1 && current_word.is_some() {
                    let word = current_word.unwrap();
                    if dict.contains(&word) {
                        if word_in_this_turn {
                            words.push(word);
                        }
                    } else if !is_horizontal_word_arr[row][col] {
                        return None;
                    }
                    word_in_this_turn = false;
                    current_word = None;
                }
            }
        }
        Some(words)
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, sync::Arc};

    use crate::player::Player;

    use super::*;

    #[test]
    fn validate_with_the_word_row_should_return_vector_with_string_the(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player, 1);
        board.tiles[0][0] = Some(t_tile);
        board.tiles[0][1] = Some(h_tile);
        board.tiles[0][2] = Some(e_tile);
        let list = board.validate(&wordlist, game).unwrap();
        assert!(list[0] == "the");
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
        let e_tile = Tile::new('e', player.clone(), 1);
        let game = Arc::new(Game::new(0, vec![player]));
        board.tiles[0][0] = Some(t_tile);
        board.tiles[1][0] = Some(h_tile);
        board.tiles[2][0] = Some(e_tile);
        let list = board.validate(&wordlist, game).unwrap();
        assert!(list[0] == "the");
        Ok(())
    }

    #[test]
    fn validate_with_tub_word_col_should_return_vector_with_string_tub(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("tub"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let u_tile = Tile::new('u', player.clone(), 1);
        let b_tile = Tile::new('b', player.clone(), 1);
        board.tiles[25][23] = Some(t_tile);
        board.tiles[25][24] = Some(u_tile);
        board.tiles[25][25] = Some(b_tile);
        let game = Arc::new(Game::new(0, vec![player]));
        let list = board.validate(&wordlist, game).unwrap();
        assert!(list[0] == "tub");
        Ok(())
    }

    #[test]
    fn validate_with_the_word_tilt_should_return_none() -> Result<(), Box<dyn Error + Sync + Send>>
    {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player.clone(), 1);
        board.tiles[0][0] = Some(t_tile);
        board.tiles[1][1] = Some(h_tile);
        board.tiles[2][2] = Some(e_tile);
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        let list = board.validate(&wordlist, game);
        assert!(list.is_none());
        Ok(())
    }

    #[test]
    fn validate_with_the_word_jump_to_next_row_should_return_vector_with_string_the(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player.clone(), 1);
        board.tiles[0][23] = Some(t_tile);
        board.tiles[0][24] = Some(h_tile);
        board.tiles[0][25] = Some(e_tile);
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        let list = board.validate(&wordlist, game).unwrap();
        assert!(list[0] == "the");
        Ok(())
    }

    #[test]
    fn validate_with_the_word_col_in_the_middle_should_return_vector_with_string_the(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player.clone(), 1);
        board.tiles[12][12] = Some(t_tile);
        board.tiles[13][12] = Some(h_tile);
        board.tiles[14][12] = Some(e_tile);
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        let list = board.validate(&wordlist, game).unwrap();
        assert!(list[0] == "the");
        Ok(())
    }

    #[test]
    fn validate_with_the_word_jump_to_next_col_should_return_vector_with_string_the(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player.clone(), 1);
        board.tiles[23][0] = Some(t_tile);
        board.tiles[24][0] = Some(h_tile);
        board.tiles[25][0] = Some(e_tile);
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        let list = board.validate(&wordlist, game).unwrap();
        assert!(list[0] == "the");
        Ok(())
    }

    #[test]
    fn validate_with_the_word_exceed_horizontal_border_should_return_none(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player.clone(), 1);
        board.tiles[0][25] = Some(t_tile);
        board.tiles[1][0] = Some(h_tile);
        board.tiles[1][1] = Some(e_tile);
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        let list = board.validate(&wordlist, game);
        assert!(list.is_none());
        Ok(())
    }

    #[test]
    fn validate_with_the_word_exceed_vertical_border_should_return_none(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let mut board = Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        let t_tile = Tile::new('t', player.clone(), 1);
        let h_tile = Tile::new('h', player.clone(), 1);
        let e_tile = Tile::new('e', player.clone(), 1);
        board.tiles[25][0] = Some(t_tile);
        board.tiles[0][1] = Some(h_tile);
        board.tiles[0][2] = Some(e_tile);
        let game = Arc::new(Game::new(0, vec![player.clone()]));
        let list = board.validate(&wordlist, game);
        assert!(list.is_none());
        Ok(())
    }

    #[test]
    fn validate_with_space_should_return_some_with_space_vector(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let mut wordlist = HashSet::new();
        wordlist.insert(String::from("the"));
        let board = Board::new();
        let game = Arc::new(Game::new(0, vec![]));
        let list = board.validate(&wordlist, game);
        assert!(list.is_some());
        assert!(list.unwrap().len() == 0);
        Ok(())
    }
}
