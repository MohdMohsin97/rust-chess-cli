use chess::Board;

fn main() {
    let board = Board::new();

    board.display();

    board.show_spot(1, 0);
}
