
use std::vec::Vec;


const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;


pub struct ConnectFourGame {
    // The board is a vector of columns, with column 0 on the left.
    // The rows within each column are from bottom-to-top (0 on the bottom).
    pub board: Vec<Vec<i32>>,
}

impl ConnectFourGame {
    pub fn new() -> Self {
        return ConnectFourGame {
            board: vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH],
        }
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let game = ConnectFourGame::new();
        for column in game.board {
            for spot in column {
              assert_eq!(spot, 0);
            }
        }
    }
}
