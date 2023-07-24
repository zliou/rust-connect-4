use std::cmp::max;
use std::cmp::min;
use std::vec::Vec;


const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;
const WIN_LENGTH: usize = 4;


#[derive(Debug)]
#[derive(PartialEq)]
enum GameState {
    InProgress,
    WinP1,
    WinP2,
    Tie,
}


#[derive(Debug)]
#[derive(PartialEq)]
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


    // Return whether the board is full.
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


    //
    fn check_win(&self, player_hint: i32, col_hint: usize) -> GameState {
        return GameState::InProgress;  // TODO
    }


    // Check if a given column contains a win for the given player.
    fn check_column_win(&self, player_hint: i32, col_hint: usize) -> GameState {
        let mut consecutive: usize = 0;
        for i in (0..self.board[col_hint].len()).rev() {
            if self.board[col_hint][i] == player_hint {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive >= WIN_LENGTH {
                return match player_hint {
                    1 => GameState::WinP1,
                    2 => GameState::WinP2,
                    _ => GameState::Tie,
                };
            }
        }
        return GameState::InProgress;
    }

    
    // Check if a win has occurred in the row of the topmost piece in the given column.
    fn check_row_win(&self, player_hint: i32, col_hint: usize) -> GameState {
        let row: usize = self.board[col_hint].len() - 1;
        let mut consecutive: usize = 0;
        for col in 0..BOARD_WIDTH {
            if self.board[col].len() <= row {
                consecutive = 0;
                continue;
            }
            if self.board[col][row] == player_hint {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive >= WIN_LENGTH {
                return match player_hint {
                    1 => GameState::WinP1,
                    2 => GameState::WinP2,
                    _ => GameState::Tie,
                };
            }
        }
        return GameState::InProgress;
    }

    
    // Check if a win has occurred on the back-diagonal containing the topmost piece in the
    // given column. A back-diagonal is shaped like '\'.
    fn check_back_diagonal_win(&self, player_hint: i32, col_hint: usize) -> GameState {
        let placed_row: usize = self.board[col_hint].len() - 1;
        let sum = placed_row + col_hint;
        let mut row: usize = max((sum as i32 - BOARD_WIDTH as i32), 0) as usize;
        let mut col: i32 = min(sum, BOARD_WIDTH) as i32;
        let mut consecutive: usize = 0;
        while col >= 0 && row < BOARD_WIDTH {
            if self.board[col as usize].len() <= row {
                consecutive = 0;
                row += 1;
                col -= 1;
                continue;
            }
            if self.board[col as usize][row] == player_hint {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive >= WIN_LENGTH {
                return match player_hint {
                    1 => GameState::WinP1,
                    2 => GameState::WinP2,
                    _ => GameState::Tie,
                };
            }
            row += 1;
            col -= 1;
        }
        return GameState::InProgress;
    }


    // Check if a win has occurred on the forward-diagonal containing the topmost piece in the
    // given column. A forward-diagonal is shaped like '/'.
    fn check_forward_diagonal_win(&self, player_hint: i32, col_hint: usize) -> GameState {
        let placed_row: usize = self.board[col_hint].len() - 1;
        let diff: usize = col_hint.abs_diff(placed_row);
        let mut row: usize = if placed_row > col_hint { diff } else { 0 };
        let mut col: usize = if placed_row < col_hint { diff } else { 0 };
        let mut consecutive: usize = 0;
        while row < BOARD_HEIGHT && col < BOARD_WIDTH {
            if self.board[col].len() <= row {
                consecutive = 0;
                row += 1;
                col += 1;
                continue;
            }
            if self.board[col][row] == player_hint {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            if consecutive >= WIN_LENGTH {
                return match player_hint {
                    1 => GameState::WinP1,
                    2 => GameState::WinP2,
                    _ => GameState::Tie,
                };
            }
            row += 1;
            col += 1;
        }
        return GameState::InProgress;
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

    #[test]
    fn test_column_win_success() {
        let mut game = ConnectFourGame::new();
        game.board[0] = vec![2,1,1,1,1];
        assert_eq!(game.check_column_win(/*player_hint=*/1, /*col_hint=*/0), GameState::WinP1);
    }

    #[test]
    fn test_no_column_win() {
        let mut game = ConnectFourGame::new();
        game.board[0] = vec![2,1,2,1];
        assert_eq!(game.check_column_win(/*player_hint=*/1, /*col_hint=*/0), GameState::InProgress);
    }

    #[test]
    fn test_row_win_with_blanks() {
        let mut game = ConnectFourGame::new();
        game.board = vec![vec![1]; BOARD_WIDTH];
        game.board[0] = vec![];
        game.board[2] = vec![];
        assert_eq!(game.check_row_win(/*player_hint=*/1, /*col_hint=*/4), GameState::WinP1);
    }

    #[test]
    fn test_row_win() {
        let mut game = ConnectFourGame::new();
        game.board = vec![vec![1]; BOARD_WIDTH];
        assert_eq!(game.check_row_win(/*player_hint=*/1, /*col_hint=*/0), GameState::WinP1);
    }

    #[test]
    fn test_no_row_win_with_blanks() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![], 
            vec![2,1], 
            vec![1,2,1,2], 
            vec![1,1,2,1], 
            vec![2], 
        ];
        assert_eq!(game.check_row_win(/*player_hint=*/1, /*col_hint=*/3), GameState::InProgress);
    }

    #[test]
    fn test_no_row_win() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![], 
            vec![2,1], 
            vec![1,2,1,2], 
            vec![1,1,2,1], 
            vec![2], 
        ];
        assert_eq!(game.check_row_win(/*player_hint=*/2, /*col_hint=*/6), GameState::InProgress);
    }

    #[test]
    fn test_forward_diagonal_win() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![2,2], 
            vec![2], 
            vec![2,1], 
            vec![1,2,1,2], 
            vec![1,1,2,1], 
            vec![2,1,2,1,1], 
        ];
        assert_eq!(game.check_forward_diagonal_win(/*player_hint=*/1, /*col_hint=*/3),
                   GameState::WinP1);
    }

    #[test]
    fn test_forward_diagonal_win_long() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![2,2], 
            vec![2,1,2], 
            vec![2,1,1,2], 
            vec![1,2,2,1,2], 
            vec![1,1,2,1,2,2], 
            vec![2,1,2,1,1], 
        ];
        assert_eq!(game.check_forward_diagonal_win(/*player_hint=*/2, /*col_hint=*/3),
                   GameState::WinP2);
    }

    #[test]
    fn test_no_forward_diagonal_win() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![], 
            vec![2,1], 
            vec![1,2,1,2], 
            vec![1,1,2,1], 
            vec![2], 
        ];
        assert_eq!(game.check_forward_diagonal_win(/*player_hint=*/1, /*col_hint=*/3),
                   GameState::InProgress);
    }

    #[test]
    fn test_back_diagonal_win() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![2,2], 
            vec![2], 
            vec![2,1,2,1], 
            vec![1,2,1,2], 
            vec![1,1,2,1], 
            vec![1,1,2,], 
        ];
        assert_eq!(game.check_back_diagonal_win(/*player_hint=*/1, /*col_hint=*/3),
                   GameState::WinP1);
    }

    #[test]
    fn test_back_diagonal_win_long() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![2,2], 
            vec![2,1,2,1,2], 
            vec![2,1,1,2], 
            vec![1,2,2,1,1], 
            vec![1,2,2,1,2,2], 
            vec![2,1,2,1,1], 
        ];
        assert_eq!(game.check_back_diagonal_win(/*player_hint=*/2, /*col_hint=*/2),
                   GameState::WinP2);
    }

    #[test]
    fn test_no_back_diagonal_win() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![], 
            vec![2,1], 
            vec![1,2,1,2], 
            vec![1,1,2,1], 
            vec![2], 
        ];
        assert_eq!(game.check_back_diagonal_win(/*player_hint=*/2, /*col_hint=*/1),
                   GameState::InProgress);
    }

/*
    #[test]
    fn test_() {
        let mut game = ConnectFourGame::new();
    }
*/

}
