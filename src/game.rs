
use std::vec::Vec;


const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;


pub struct ConnectFourGame {
    pub board: Vec<Vec<i32>>,
}

impl ConnectFourGame {
    pub fn new() -> Self {
        return ConnectFourGame {
            board: vec![vec![0; BOARD_HEIGHT]; BOARD_WIDTH],
        }
    }
}


