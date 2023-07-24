use std::vec::Vec;


const BOARD_INDENT: &str = "    ";
const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;
const TOKEN_EMPTY: &str = "  ";  // Player tokens are 2 chars wide.
const TOKEN_P1: &str = "🟡";
const TOKEN_P2: &str = "🔴";


pub fn print_board(board: &Vec<Vec<i32>>, player: i32) {
    clear_board();
    for row in (0..BOARD_HEIGHT).rev() {
        print_row(board, row);
        println!("");
    }
    print_bottom_row();
    print_command_row();
    println!("");
    print_instructions(player);
}


pub fn print_end(board: &Vec<Vec<i32>>) {
    clear_board();
    for row in (0..BOARD_HEIGHT).rev() {
        print_row(board, row);
        println!("");
    }
    print_bottom_row();
}


fn token(player: i32) -> String {
    return match player {
        1 => String::from(TOKEN_P1),
        2 => String::from(TOKEN_P2),
        _ => String::from(TOKEN_EMPTY),
    }
}


fn print_bottom_row() {
    print!("{}", BOARD_INDENT);
    println!(" ==================================== ");
}


fn print_command_row() {
    print!("{}", BOARD_INDENT);
    println!("   1    2    3    4    5    6    7  ");
}


fn print_instructions(player: i32) {
    println!("Choose a column - [1] through [{}] - and press [Enter] to play that column. ",
             BOARD_WIDTH);
    println!("Enter [q] to quit.");
    println!("It's {}'s turn.", token(player));
}


fn print_row(board: &Vec<Vec<i32>>, row: usize) {
    print!("{}", BOARD_INDENT);
    print!(" | ");
    for col in 0..BOARD_WIDTH {
        if board[col].len() <= row {
            print!("{}", String::from(TOKEN_EMPTY));
        } else {
            print!("{}", token(board[col][row]));
        }
        print!(" | ");
    }
}


fn clear_board() {
    println!("{esc}c", esc = 27 as char);
}

