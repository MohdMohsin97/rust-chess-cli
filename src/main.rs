use chess::Game;

fn main() {
    let mut game = Game::new();

    game.make_move("2a->3a");
    game.make_move("3a->4a");
}
