use std:: result::Result;
use std::error::Error;

use regex::Regex;
struct Position {
    x: usize,
    y: usize
}

pub trait Piece {
    fn sign(&self) -> char;
    fn valid_move(&self, game: &mut Game, x1: usize, y1: usize, x2: usize, y2: usize) -> Result<(), Box<dyn Error>>;
    fn give_color(&self) -> bool;
}

pub struct Spot {
    piece: Option<Box<dyn Piece>>,
    position: Position
}

impl Spot {
    pub fn new(piece: Option<Box<dyn Piece>>, x: usize, y: usize) -> Spot{
        Spot {
            piece,
            position: Position{
                x,
                y
            }
        }
    }
}

struct Rook {
    sign: char,
    white: bool
}

impl Rook {
    pub fn new(white: bool) -> Rook {
        if white {
            Rook {
                sign: 'R',
                white
            }
        } else {
            Rook {
                sign: 'r',
                white
            }
        }
    }
}

impl Piece for Rook {
    fn sign(&self) -> char {
        self.sign
    }
    fn valid_move(&self,game: &mut Game, x1: usize, y1: usize, x2: usize, y2: usize) -> Result<(), Box<dyn Error>> {
        let (a, b, c, d);
        
        if x1 > x2 {
            (a, b) = (x1 as i32, x2 as i32);
        
        } else {
            (b , a) = (x1 as i32, x2 as i32);
        }

        if y1 > y2 {
            (c, d) = (y1 as i32, y2 as i32);
        } else {
            (d, c) = (y1 as i32, y2 as i32);
        }

        if a - b != 0 && c - d == 0 {
            for x  in b+1..a {
                match &game.board.boxes[x as usize][y1].piece {
                    Some(_) => return Err("Illegal Move!".into()),
                    None => continue,  
                }
            }
            match &game.board.boxes[x2][y2].piece {
                Some(piece) => {
                    if piece.give_color() != self.white {
                        Ok(())
                    } else {
                        Err("Illegal Move!".into())
                    }
                }
                None => Ok(())
            }
        } else if a - b == 0 && c - d !=0 {
            for y  in d+1..c {
                match &game.board.boxes[x1][y as usize].piece {
                    Some(_) => return Err("Illegal Move!".into()),
                    None => continue,  
                }
            }
            match &game.board.boxes[x2][y2].piece {
                Some(piece) => {
                    if piece.give_color() != self.white {
                        Ok(())
                    } else {
                        Err("Illegal Move!".into())
                    }
                }
                None => Ok(())
            }
        } else {
            Err("Illegal Move!".into())
        }

    }

    fn give_color(&self) -> bool {
        self.white
    }

}


struct Knight {
    sign: char,
    white: bool
}

impl Knight {
    pub fn new(white: bool) -> Knight {
        if white {
            Knight {
                sign: 'N',
                white
            }
        } else {
            Knight {
                sign: 'n',
                white
            }
        }
    }
}

impl Piece for Knight {
    fn sign(&self) -> char {
        self.sign
    }
    fn valid_move(&self, game: &mut Game, x1: usize, y1: usize, x2: usize, y2: usize) -> Result<(), Box<dyn Error>> {
        
        if ( x1.abs_diff(x2) == 2 && y1.abs_diff(y2) == 1) || ( x1.abs_diff(x2) == 1 && y1.abs_diff(y2) == 2) {
            match &game.board.boxes[x2][y2].piece {
                Some(piece) => {
                    if piece.give_color() != self.white {
                        Ok(())
                    } else {
                        Err("Illegal Move!".into())
                    }
                },
                None => Ok(())
            }
        } else {
            Err("Illegal Move!".into())
        }

    }

    fn give_color(&self) -> bool {
        self.white
    }
}

struct Bishop {
    sign: char,
    white: bool
}

impl Bishop {
    pub fn new(white: bool) -> Bishop {
        if white {
            Bishop {
                sign: 'B',
                white
            }
        } else {
            Bishop {
                sign: 'b',
                white
            }
        }
    }
}

impl Piece for Bishop {
    fn sign(&self) -> char {
        self.sign
    }
    fn valid_move(&self, game: &mut Game, x1: usize, y1: usize, x2: usize, y2: usize) -> Result<(), Box<dyn Error>> {
        let (mut a, mut b, c, d) = (x1, y1 , x2 , y2);
        
        if (a as i32 - c as i32).abs() == (b as i32 - d as i32).abs()  {
            for _  in 1..(a as i32 - c as i32).abs() {
               if a > c {
                a = a - 1;
               } else {
                a = a + 1;
               }

               if b > d {
                b = b - 1;
               } else {
                b = b + 1;
               }    
               
                match &game.board.boxes[a][b].piece {
                    Some(_) => {
                        return Err("Illegal Move!".into())},
                    None => continue,  
                }
            }
            match &game.board.boxes[x2][y2].piece {
                Some(piece) => {
                    if piece.give_color() != self.white {
                        Ok(())
                    } else {
                        Err("Illegal Move!".into())
                    }
                }
                None => Ok(())
            }
        }  else {
            Err("Illegal Move!".into())
        }
    }
    fn give_color(&self) -> bool {
        self.white
    }
}

struct Queen {
    sign: char,
    white: bool
}

impl Queen {
    pub fn new(white: bool) -> Queen {
        if white {
            Queen {
                sign: 'Q',
                white
            }
        } else {
            Queen {
                sign: 'q',
                white
            }
        }
    }
}

impl Piece for Queen {
    fn sign(&self) -> char {
        self.sign
    }
    fn valid_move(&self, game: &mut Game, x1: usize, y1: usize, x2: usize, y2: usize) -> Result<(), Box<dyn Error>> {
        let (mut a, mut b) = (x1, y1);
        if ( x1.abs_diff(x2) == 0 ) || ( y1.abs_diff(y2) == 0 ) || ( x1.abs_diff(x2) == y1.abs_diff(y2) )  {
            while a != x2 && b != y2 {
                if a > x2 {
                    a = a - 1;
                } else if a < x2 {
                    a = a + 1;
                } 
                
                if b > y2 {
                    b = b - 1;
                } else if b < y2 {
                    b = b + 1;
                }
                if a == x2 && b == y2 {
                    break;
                }

                println!("{a} {b}");
                 match &game.board.boxes[a][b].piece {
                     Some(_) => return Err("Illegal Move!".into()),
                     None => continue
                 }
            }
            match &game.board.boxes[x2][y2].piece {
                Some(piece) =>  {
                    if piece.give_color() != self.white {
                        Ok(())
                    } else {
                      Err("Illegal Move!".into())

                    }
                }
                None => Ok(())
            }
        } else {
            Err("Illegal Move!".into())
        }
    }
    fn give_color(&self) -> bool {
        self.white
    }
}


struct King {
    sign: char,
    white: bool
}

impl King {
    pub fn new(white: bool) -> King {
        if white {
            King {
                sign: 'K',
                white
            }
        } else {
            King {
                sign: 'k',
                white
            }
        }
    }
}

impl Piece for King {
    fn sign(&self) -> char {
        self.sign
    }
    fn valid_move(&self, game: &mut Game, x1: usize, y1: usize, x2: usize, y2: usize) -> Result<(), Box<dyn Error>> {

        if(x1.abs_diff(x2) == 1 && y1.abs_diff(y2) == 0 ) || (x1.abs_diff(x2) == 0 && y1.abs_diff(y2) == 1 ) || (x1.abs_diff(x2) == 1 && y1.abs_diff(y2) == 1 ) {
            match &game.board.boxes[x2][y2].piece {
                Some(piece) => {
                    if piece.give_color() != self.white {
                        Ok(())
                    } else {
                        Err("Illegal Move!".into())
                    }
                },
                None => Ok(())
            }
        } else {
            Err("Illegal Move!".into())
        }
    }

    fn give_color(&self) -> bool {
        self.white
    }
}


struct Pawn {
    sign: char,
    white: bool
}

impl Pawn {
    pub fn new(white: bool) -> Pawn {
        if white {
            Pawn {
                sign: 'P',
                white
            }
        } else {
            Pawn {
                sign: 'p',
                white
            }
        }
    }
}

impl Piece for Pawn {
    fn sign(&self) -> char {
        self.sign
    }
    fn valid_move(&self, game: &mut Game, x1: usize, y1: usize, x2: usize, y2: usize) -> Result<(), Box<dyn Error>> {
        let a;
        let b;
        let (c, d) = (y1 as i32, y2 as i32);
        
        if self.white {
            (a, b) = (x1 as i32, x2 as i32);
        } else {
            (b, a) = (x1 as i32, x2 as i32);
        }

        if a == 6 && b == 4 && d - c == 0 {
            match &game.board.boxes[x2][y2].piece {
                Some(_) => Err("Illegal Move!".into()),
                None => Ok(())
            }
        } else if a == 3 && b == 1 && d - c == 0 {
            match &game.board.boxes[x2][y2].piece {
                Some(_) => Err("Illegal Move!".into()),
                None => Ok(())
            }
        }
         else if a - b == 1 && d - c == 0 {
            match &game.board.boxes[x2][y2].piece {
                Some(_) => Err("Illegal Move!".into()),
                None => Ok(())
            }
        } else if a - b == 1 && (c - d == 1 || d - c == 1) {
            match &game.board.boxes[x2][y2].piece {
                Some(piece) => {
                    if piece.give_color() == self.white {
                        Err("Illegal Move".into())
                    } else {
                        Ok(())
                    }
                },
                None => Err("Illegal Move!".into())
            }
        } else {
            Err("Illegal Move!".into())
        }
    }
    fn give_color(&self) -> bool {
        self.white
    }
}

pub struct Board {
    pub boxes: Vec<Vec<Spot>>,
}

impl Board {
    pub fn new() -> Board {
        let mut boxes: Vec<Vec<Spot>> = Vec::with_capacity(8);

        let mut row1: Vec<Spot> = Vec::with_capacity(8);
        
        row1.push(Spot::new(Some(Box::new(Rook::new(false))), 0, 0));
        row1.push(Spot::new(Some(Box::new(Knight::new(false))), 1, 0));
        row1.push(Spot::new(Some(Box::new(Bishop::new(false))), 2, 0));
        row1.push(Spot::new(Some(Box::new(Queen::new(false))), 3, 0));
        row1.push(Spot::new(Some(Box::new(King::new(false))), 4, 0));
        row1.push(Spot::new(Some(Box::new(Bishop::new(false))), 5, 0));
        row1.push(Spot::new(Some(Box::new(Knight::new(false))), 6, 0));
        row1.push(Spot::new(Some(Box::new(Rook::new(false))), 7, 0));

        boxes.push(row1);

        let mut row2: Vec<Spot> = Vec::with_capacity(8);
        
        for i in 0..8 {
            let pawn = Pawn::new(false);
            row2.push(Spot::new(Some(Box::new(pawn)), i, 1));
        }

        boxes.push(row2);

        for y in 2..6 {
            let mut row: Vec<Spot> = Vec::with_capacity(8);
            for x in 0..8 {
                let spot = Spot::new(None, x, y);
                row.push(spot);
            }
            boxes.push(row);
        }

        let mut row7: Vec<Spot> = Vec::with_capacity(8);
        
        for i in 0..8 {
            let pawn = Pawn::new(true);
            row7.push(Spot::new(Some(Box::new(pawn)), i, 6));
        }

        boxes.push(row7);

        let mut row8: Vec<Spot> = Vec::with_capacity(8);
        
        row8.push(Spot::new(Some(Box::new(Rook::new(true))), 0, 7));
        row8.push(Spot::new(Some(Box::new(Knight::new(true))), 1, 7));
        row8.push(Spot::new(Some(Box::new(Bishop::new(true))), 2, 7));
        row8.push(Spot::new(Some(Box::new(Queen::new(true))), 3, 7));
        row8.push(Spot::new(Some(Box::new(King::new(true))), 4, 7));
        row8.push(Spot::new(Some(Box::new(Bishop::new(true))), 5, 7));
        row8.push(Spot::new(Some(Box::new(Knight::new(true))), 6, 7));
        row8.push(Spot::new(Some(Box::new(Rook::new(true))), 7, 7));

        boxes.push(row8);

        Board {
            boxes 
        }
    }

    pub fn display(&self) {
        let mut r = 8;
        println!(" ------------------- ");
        for  row in self.boxes.iter() {
            print!("{r}| ");
            for spot_option in row {
                match &spot_option.piece {
                    Some(piece) => print!("{}", piece.sign()),
                    None => print!("*")
                }
                print!(" ");
            }
            r = r-1;
            print!("|\n");
        }
        println!( " ------------------- ");
        println!("   a b c d e f g h ")
    }

    pub fn show_spot(&self, x: usize, y: usize) {
        let piece = match &self.boxes[x][y].piece.as_ref() {
            Some(p) => p.sign(),
            None => 'X'
        };
        let position = &self.boxes[x][y].position;

        println!("Piece = {} at x: {} & y: {}.", piece, position.x, position.y)
    }
}



pub struct Game {
    board: Board,
    // player1: bool,
    // player2: bool,
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new();
        // let player1 = true;
        // let player2 = false;
        board.display();
        Game {
            board,
            // player1,
            // player2,
        }
    }

    pub fn make_move(&mut self, player:bool, input: &str) -> Result<(), Box<dyn Error>> {
        let pattern = Regex::new(r"^[1-8][a-h]->[1-8][a-h]$").unwrap();

        let parts: Vec<&str>;

        if pattern.is_match(input) {
            parts = input.split("->").collect();
        } else {
           return  Err("Incorrect input".into())
        }


        let (x1, y1) =  extract_coordination(parts[0])?;
        let (x2, y2) = extract_coordination(parts[1])?;

        if let Some(piece) = self.board.boxes[x1][y1].piece.take() {
            if piece.give_color() != player {
                self.board.boxes[x1][y1].piece = Some(piece);
                return Err("Not your piece".into());
            } else {
                piece.valid_move(self, x1, y1, x2, y2)?;
                self.board.boxes[x2][y2].piece = Some(piece);    
                self.board.display();
                Ok(())
            }
        } else {
            Err("There is no piece to move".into())
        }
    }
}


fn extract_coordination(coord_str: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let chars: Vec<char> = coord_str.chars().collect();
    let x = chars[0].to_digit(10).unwrap() as usize;
    let y = (chars[1] as usize) - ('a' as usize);
        Ok((8-x, y))
}

