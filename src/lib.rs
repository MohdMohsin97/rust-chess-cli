struct Position {
    x: usize,
    y: usize
}

pub struct Spot {
    piece: Box<dyn Piece>,
    position: Position
}

impl Spot {
    pub fn new(piece: Box<dyn Piece>, x: usize, y: usize) -> Spot{
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

pub trait Piece {
    fn sign(&self) -> char;
}

pub struct Board {
    boxes: Vec<Vec<Option<Spot>>>,
}

impl Board {
    pub fn new() -> Board {
        const NONE: Option<Spot> = None;
        let mut boxes: Vec<Vec<Option<Spot>>> = Vec::with_capacity(8);

        let mut row1: Vec<Option<Spot>> = Vec::with_capacity(8);
        
        for _ in 0..8 {
            let rook = Rook::new(false);
            row1.push(Some(Spot::new(Box::new(rook), 0, 0)));
        }

        boxes.push(row1);

        let mut row2: Vec<Option<Spot>> = Vec::with_capacity(8);
        
        for _ in 0..8 {
            let pawn = Pawn::new(false);
            row2.push(Some(Spot::new(Box::new(pawn), 0, 0)));
        }

        boxes.push(row2);

        for _ in 0..4 {
            let row: Vec<Option<Spot>> = Vec::from([NONE; 8]);
            boxes.push(row);
        }

        let mut row7: Vec<Option<Spot>> = Vec::with_capacity(8);
        
        for _ in 0..8 {
            let pawn = Pawn::new(true);
            row7.push(Some(Spot::new(Box::new(pawn), 0, 0)));
        }

        boxes.push(row7);

        let mut row8: Vec<Option<Spot>> = Vec::with_capacity(8);
        
        for _ in 0..8 {
            let rook = Rook::new(true);
            row8.push(Some(Spot::new(Box::new(rook), 0, 0)));
        }

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
                match spot_option {
                    Some(spot) => print!("{}", spot.piece.sign() ),
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
}