use chess::Game;

fn main() {
    let mut game = Game::new();

    game.make_move("2a->4a");
    game.make_move("7a->5a");
    game.make_move("7b->5b");
    game.make_move("4a->3a");
    // game.make_move("6a->4a");

}
