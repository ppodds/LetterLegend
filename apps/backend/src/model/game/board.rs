use super::tile::Tile;

include!(concat!(env!("OUT_DIR"), "/game.board.rs"));

impl From<crate::game::game::Board> for Board {
    fn from(board: crate::game::game::Board) -> Self {
        let mut rows = Vec::new();
        for row in board {
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
        const INIT: Option<crate::game::tile::Tile> = None;
        const T: [Option<crate::game::tile::Tile>; 26] = [INIT; 26];
        let mut board = [T; 26];
        let player = Arc::new(Player::new(0, String::from("test")));
        board[0][0] = Some(crate::game::tile::Tile::new(
            String::from("a"),
            player.clone(),
        ));
        board[0][25] = Some(crate::game::tile::Tile::new(
            String::from("b"),
            player.clone(),
        ));
        board[25][0] = Some(crate::game::tile::Tile::new(
            String::from("c"),
            player.clone(),
        ));
        board[25][25] = Some(crate::game::tile::Tile::new(
            String::from("d"),
            player.clone(),
        ));
        let board = Board::from(board);
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
