mod game;
mod printer;


fn main() {
    println!("Hello, world!");
    let game = game::ConnectFourGame::new();
    printer::print_board(&game.board);
}

