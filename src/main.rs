use chess::Game;

fn main() {
    let mut game = Game::new();

    game.make_move("2a->4a");
    game.make_move("2b->4b");
    game.make_move("7b->5b");
    game.make_move("4a->5b");
    game.make_move("1a->6a");
    game.make_move("6a->6h");
    game.make_move("6h->7h");
    game.make_move("7h->2h");
    
    // game.make_move("8a->8h");
    // game.make_move("2c->4c");
    // game.make_move("4b->5b");
    // game.make_move("7a->5a");
    // game.make_move("7b->5b");
    // game.make_move("6a->4a");

}
