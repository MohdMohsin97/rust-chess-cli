use std::io;

use chess::Game;

fn main() {
    let mut game = Game::new();
    let mut white = true;
    loop {
        if white {
            println!("White's trun: (example: 2e->4e)");
        } else {
            println!("Black's trun: (example: 2e->4e)");
        }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match game.make_move(white, &input.trim()) {
                Ok(_) => white = !white,
                Err(e) => println!("{e}")
            }
        
    }

}
