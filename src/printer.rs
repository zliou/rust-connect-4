use std::vec::Vec;


const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;



pub fn print_row(board: &Vec<Vec<i32>>, row: usize) {
    print!(" | ");
    for col in 0..BOARD_WIDTH {
        print!("{}", board[col][row]);
        print!(" | ");
    }
}

pub fn print_board(board: &Vec<Vec<i32>>) {
    for row in 0..BOARD_HEIGHT {
        print_row(board, row);
        println!("");
    }
}

