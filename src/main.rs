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
    game.make_move("1c->2b");
    game.make_move("2c->4c");
    game.make_move("2b->7g");
    game.make_move("7g->4d");
    game.make_move("8f->6h");
    game.make_move("6h->2d");
    game.make_move("1b->3c");
    game.make_move("3c->5d");
    game.make_move("5d->7e");
    game.make_move("8g->7e");
    game.make_move("1e->2d");
    game.make_move("8e->8f");
    game.make_move("5b->6b");
    game.make_move("1d->4a");
    game.make_move("4a->7d");
    game.make_move("7d->3h");
    game.make_move("8d->4d");
    
    
    // game.make_move("4d->8a");
    // game.make_move("4d->2f");
    // game.make_move("8f->7f");
    // game.make_move("7e->5a");
    // game.make_move("2f->4f");
    // game.make_move("7h->2h");
    // game.make_move("8a->8h");
    // game.make_move("2c->4c");
    // game.make_move("4b->5b");
    // game.make_move("7a->5a");
    // game.make_move("7b->5b");
    // game.make_move("6a->4a");

}
