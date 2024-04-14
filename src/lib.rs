struct Position {
    x: usize,
    y: usize
}

pub trait Piece {
    fn sign(&self) -> char;
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
                // match spot_option {
                //     Some(spot) => print!("{}", spot.piece.sign() ),
                //     None => print!("*")
                // }
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

