
use std::vec::Vec;


const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;


#[derive(PartialEq)]
#[derive(Debug)]
enum TurnResult {
    Valid,
    Invalid,
}


pub struct ConnectFourGame {
    // The board is a vector of columns, with column 0 on the left.
    // The rows within each column are from bottom-to-top (0 on the bottom).
    pub board: Vec<Vec<i32>>,
}


impl ConnectFourGame {
    pub fn new() -> Self {
        return ConnectFourGame {
            board: vec![Vec::new(); BOARD_WIDTH],
        }
    }


    fn is_board_full(&mut self) -> bool {
        for col in &self.board {
            if col.len() != BOARD_HEIGHT {
                return false;
            }
        }
        return true;
    }


    // Place the given player's token in the given column.
    // Return whether the move is valid.
    fn place(&mut self, player: i32, col: usize) -> TurnResult {
        if self.board[col].len() >= BOARD_HEIGHT {
            return TurnResult::Invalid;
        }
        self.board[col].push(player);
        return TurnResult::Valid;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let game = ConnectFourGame::new();
        for column in game.board {
            assert_eq!(column.len(), 0);
        }
    }

    #[test]
    fn test_place_success() {
        let mut game = ConnectFourGame::new();
        assert_eq!(game.place(1, 0), TurnResult::Valid);
        assert_eq!(game.board[0].len(), 1);
        assert_eq!(game.board[0][0], 1);
    }

    #[test]
    fn test_invalid_place() {
        let mut game = ConnectFourGame::new();
        for _i in 0..BOARD_HEIGHT {
            assert_eq!(game.place(1, 0), TurnResult::Valid);
        }
        assert_eq!(game.place(1, 0), TurnResult::Invalid);
    }

    #[test]
    fn test_board_not_full() {
        let mut game = ConnectFourGame::new();
        assert!(!game.is_board_full());
    }

    #[test]
    fn test_board_full() {
        let mut game = ConnectFourGame::new();
        game.board = vec![vec![1; BOARD_HEIGHT]; BOARD_WIDTH];
        assert!(game.is_board_full());
    }

/*
    #[test]
    fn test_() {
        let mut game = ConnectFourGame::new();
    }
*/

}
