use super::tile::Tile;

include!(concat!(env!("OUT_DIR"), "/game.board.rs"));

impl From<&crate::game::board::Board> for Board {
    fn from(board: &crate::game::board::Board) -> Self {
        let mut rows = Vec::new();
        for row in &board.tiles {
            let mut cols = Vec::new();
            for col in row {
                cols.push(Column {
                    tile: match col {
                        Some(tile) => Some(Tile::from(tile)),
                        None => None,
                    },
                });
            }
            rows.push(Row { columns: cols });
        }

        Self { rows }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::player::Player;

    use super::*;

    #[test]
    fn from_board_return_board() {
        let mut board = crate::game::board::Board::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        board.tiles[0][0] = Some(crate::game::tile::Tile::new('a', player.clone(), 0));
        board.tiles[0][25] = Some(crate::game::tile::Tile::new('b', player.clone(), 0));
        board.tiles[25][0] = Some(crate::game::tile::Tile::new('c', player.clone(), 0));
        board.tiles[25][25] = Some(crate::game::tile::Tile::new('d', player.clone(), 0));
        let board = Board::from(&board);
        assert_eq!(
            board.rows[0].columns[0].tile.clone().unwrap().char,
            String::from("a")
        );
        assert_eq!(
            board.rows[0].columns[25].tile.clone().unwrap().char,
            String::from("b")
        );
        assert_eq!(
            board.rows[25].columns[0].tile.clone().unwrap().char,
            String::from("c")
        );
        assert_eq!(
            board.rows[25].columns[25].tile.clone().unwrap().char,
            String::from("d")
        );
        for i in 1..24 {
            assert_eq!(board.rows[0].columns[i].tile, None);
            assert_eq!(board.rows[25].columns[i].tile, None);
            assert_eq!(board.rows[i].columns[0].tile, None);
            assert_eq!(board.rows[i].columns[25].tile, None);
        }
    }
}
