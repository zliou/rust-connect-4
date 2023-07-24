mod game;
mod printer;


fn main() {
    let mut game = game::ConnectFourGame::new();
    game.play();
}

