use std::cmp::max;
use std::cmp::min;
use std::vec::Vec;


const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;
const COMMAND_QUIT: &str = "q";
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


    // Loop until a valid player input is received.
    fn get_player_input(&self) -> String {
        loop {
            let mut input = String::new();
            let _b = std::io::stdin().read_line(&mut input).unwrap();
            if self.is_valid_input(&input) {
                return input;
            }
            println!("Invalid move. Please try again.");
        }
    }


    // Return whether the given input is a valid game control.
    fn is_valid_input(&self, input: &String) -> bool {
        return input.len() == 1 && (input == COMMAND_QUIT || (
                '1' <= input.chars().nth(0).unwrap() && 
                input.chars().nth(0).unwrap() <= '7'));
    }


    // Convert the given column input (1-indexed, string) into a column index (0-indexed, usize).
    fn convert_input_to_column(&self, input: String) -> usize {
        let col: usize = input.parse::<usize>().unwrap() - 1;
        return col;
    }


    // Run one turn of the game. Return the resulting game state.
    fn turn(&mut self, active_player: i32) -> GameState {
        loop {
            let input: String = self.get_player_input();
            if input == COMMAND_QUIT {
                return GameState::Tie;
            }
            let col: usize = self.convert_input_to_column(input);
            if self.place(active_player, col) == TurnResult::Valid {
                return self.check_win(active_player, col);
            }
        }
        
    }


    // Return whether a given column is full.
    fn is_column_full(&self, col: usize) -> bool {
        return self.board[col].len() >= BOARD_HEIGHT;
    }


    // Return whether the board is full.
    fn is_board_full(&self) -> bool {
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
        if self.is_column_full(col) {
            println!("Column {} is full. Try another column.", col + 1);
            return TurnResult::Invalid;
        }
        self.board[col].push(player);
        return TurnResult::Valid;
    }


    // Check for a win given the last column played by the given player.
    // Return a tie if the board is full and there is no winner.
    // Since moves don't affect the arrangement of already-placed tokens, we only need to
    // check if the newest token results in a win.
    fn check_win(&self, player_hint: i32, col_hint: usize) -> GameState {
        let states: Vec<GameState> = vec![
                self.check_row_win(player_hint, col_hint),
                self.check_column_win(player_hint, col_hint),
                self.check_forward_diagonal_win(player_hint, col_hint),
                self.check_back_diagonal_win(player_hint, col_hint)];
        for state in states {
            if state != GameState::InProgress {
                return state;
            }
        }
        if self.is_board_full() {
            return GameState::Tie;
        }
        return GameState::InProgress;
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
        let mut row: usize = max(sum as i32 - (BOARD_WIDTH - 1) as i32, 0) as usize;
        let mut col: i32 = min(sum, BOARD_WIDTH - 1) as i32;
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
    fn test_column_not_full() {
        let game = ConnectFourGame::new();
        assert!(!game.is_column_full(0));
    }

    #[test]
    fn test_column_full() {
        let mut game = ConnectFourGame::new();
        game.board = vec![vec![1; BOARD_HEIGHT]; BOARD_WIDTH];
        assert!(game.is_column_full(0));
    }

    #[test]
    fn test_board_not_full() {
        let game = ConnectFourGame::new();
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
    fn test_forward_diagonal_win_high() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1,2], 
            vec![2,2,2], 
            vec![2,1,2,2], 
            vec![2,1,1,1,2], 
            vec![1,2,1,2], 
            vec![1,1,], 
            vec![2,1,2,1,1], 
        ];
        assert_eq!(game.check_forward_diagonal_win(/*player_hint=*/2, /*col_hint=*/3),
                   GameState::WinP2);
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
    fn test_back_diagonal_win_high() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![2,2], 
            vec![2], 
            vec![1,1,1,2,1], 
            vec![2,1,1,1,2], 
            vec![1,2,1,2,1], 
            vec![2,1,2,2,], 
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

    #[test]
    fn test_check_win_row() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![], 
            vec![2,1], 
            vec![1,1,1,2], 
            vec![1,1,2,1], 
            vec![2,1], 
        ];
        assert_eq!(game.check_win(/*player_hint=*/1, /*col_hint=*/3), GameState::WinP1);
    }

    #[test]
    fn test_check_win_column() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![], 
            vec![2,1], 
            vec![2,2,2,2], 
            vec![1,1,2,1], 
            vec![2], 
        ];
        assert_eq!(game.check_win(/*player_hint=*/2, /*col_hint=*/4), GameState::WinP2);
    }

    #[test]
    fn test_check_win_forward_diagonal() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![1], 
            vec![2,1], 
            vec![1,2,1,2], 
            vec![1,1,2,1], 
            vec![2], 
        ];
        assert_eq!(game.check_win(/*player_hint=*/1, /*col_hint=*/3), GameState::WinP1);
    }

    #[test]
    fn test_check_win_back_diagonal() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![], 
            vec![2,1,2,1], 
            vec![1,2,1,2], 
            vec![1,1,2,1], 
            vec![1], 
        ];
        assert_eq!(game.check_win(/*player_hint=*/1, /*col_hint=*/3), GameState::WinP1);
    }

    #[test]
    fn test_check_win_no_win() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1], 
            vec![1,2], 
            vec![], 
            vec![2,1], 
            vec![1,2], 
            vec![1,1,2,1], 
            vec![2], 
        ];
        assert_eq!(game.check_win(/*player_hint=*/2, /*col_hint=*/4), GameState::InProgress);
    }

    #[test]
    fn test_check_win_tie() {
        let mut game = ConnectFourGame::new();
        game.board = vec![
            vec![1,2,1,2,1,2], 
            vec![2,1,2,1,2,1], 
            vec![2,1,2,1,2,1], 
            vec![1,2,1,2,1,2], 
            vec![2,1,2,1,2,1], 
            vec![2,1,2,1,2,1], 
            vec![1,2,1,2,1,2], 
        ];
        assert_eq!(game.check_win(/*player_hint=*/2, /*col_hint=*/0), GameState::Tie);
    }

    #[test]
    fn test_accept_valid_input() {
        let mut game = ConnectFourGame::new();
        assert!(game.is_valid_input(&String::from("2")));
    }

    #[test]
    fn test_accept_valid_input_quit() {
        let mut game = ConnectFourGame::new();
        assert!(game.is_valid_input(&String::from("q")));
    }

    #[test]
    fn test_reject_invalid_input() {
        let mut game = ConnectFourGame::new();
        assert!(!game.is_valid_input(&String::from("x")));
    }

    #[test]
    fn test_convert_input_to_column() {
        let mut game = ConnectFourGame::new();
        for x in 1..7 {
            assert_eq!(game.convert_input_to_column(x.to_string()), x - 1);
        }
    }

/*
    #[test]
    fn test_() {
        let mut game = ConnectFourGame::new();
    }
*/

}

