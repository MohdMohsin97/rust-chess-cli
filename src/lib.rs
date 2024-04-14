pub struct Spot {
    piece: Box<dyn Piece>,
    x: usize,
    y: usize
}

impl Spot {
    pub fn new(piece: Box<dyn Piece>, x: usize, y: usize) -> Spot{
        Spot {
            piece,
            x,
            y
        }
    }
}

struct Rook {
}

pub trait Piece {
    
}

pub struct Board {
    boxes: Vec<Vec<Option<Spot>>>,
}

impl Board {
    pub fn new() -> Board {
        const NONE: Option<Spot> = None;
        let mut boxes: Vec<Vec<Option<Spot>>> = Vec::with_capacity(8);
        for _ in 0..8 {
            let row: Vec<Option<Spot>> = Vec::from([NONE; 8]);
            boxes.push(row);
        }

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
                    Some(_) => print!("a"),
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