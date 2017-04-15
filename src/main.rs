use std::fmt;

fn main() {
    let board: Board = Board::initial();
    println!("{}", board);
    board.print_count();
    println!("{:?}", board.place(Face::Black, (2,3)));
    if let Ok(b) = board.place(Face::White, (2,3)) {
        println!("{}", b);
    }
}

const SIZE: usize = 8;
// -- TODO

const DIRS: &[(i8, i8)] = &[(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];


#[derive(Debug,Clone,Copy, PartialEq, Eq)]
enum Face {
    Black,
    White,
}
impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match *self {
            Face::White => "●",
            Face::Black => "○",
        };
        write!(f, "{}", ch)
    }
}

impl Face {
    fn not(&self) -> Self {
        match *self {
            Face::White => Face::Black,
            Face::Black => Face::White,
        }
    }

    fn flip(&mut self) -> () {
        // TODO
        *self = self.not();
    }
}

#[derive(Debug, Clone, Copy)]
struct Board(pub [[Option<Face>; SIZE]; SIZE]);
/// For simplicity, we'll denote to locations as (x,y)
/// and access to the cell by board[x][y].

impl Board {
    fn empty() -> Board {
        Board([[None; SIZE]; SIZE])
    }
    fn initial() -> Board {
        let mut board = Board::empty();
        let mid = SIZE / 2 - 1;
        board.0[mid][mid] = Some(Face::Black);
        board.0[mid + 1][mid + 1] = Some(Face::Black);
        board.0[mid + 1][mid] = Some(Face::White);
        board.0[mid][mid + 1] = Some(Face::White);
        board
    }

    // TODO : concider type
    fn at(&self, loc: (i8, i8)) -> Option<Option<Face>> {
        self.0
            .get(loc.0 as usize)
            .and_then(|r| r.get(loc.1 as usize).map(|c| c.to_owned()))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO is this ok?
        let mut buf = String::new();
        for xs in self.0.iter() {
            buf.push_str("|");
            for p in xs {
                // lifetime issue
                // buf.push_str(match *p {
                //     None => " |",
                //     Some(ref x) => &format!("{}|", x)
                // });
                buf.push_str(&match *p {
                    None => " |".to_string(),
                    Some(x) => format!("{}|", x),
                });
            }
            buf.push_str("\n");
        }
        write!(f, "{}", buf)
    }
}

impl Board {
    fn assign(&mut self, player:Face, loc:(i8,i8)) -> Result<(),String> {
        if let Some(_) = self.at(loc) {
            self.0[loc.0 as usize][loc.1 as usize] = Some(player);
            Ok(())
        } else {
            Err("out of bounds?".to_owned())
        }
    }
    fn place(&self, player: Face, loc: (i8, i8)) -> Result<Board, String> {
        match self.at(loc) {
            None => return Err("Out of bounds".to_owned()),
            Some(Some(_)) => return Err("Location Occupied".to_owned()),
            _ => ()
        }
        let mut b = self.clone();
        let mut turn_any = false;
        for dir in DIRS {
            let mut cur = loc.madd(dir);
            let mut flips : Vec<(i8,i8)> = Vec::new();
            while let Some(Some(face)) = self.at(cur) {
                if face == player {
                    if !(flips.is_empty()) {
                        turn_any = true;
                        for flip in flips.iter() {
                            // FIXME JUST NOW
                            println!("{:?}", flip);
                            b.assign(player, *flip).expect("ierror");
                        }
                    }
                    break;
                } else {
                    flips.push(cur);
                }
                cur = cur.madd(dir);
            }
        }
        if turn_any {
            b.assign(player,loc).expect("ierror");
            Ok(b)
        } else {
            Err("Invalid Move".to_owned())
        }
    }
}

impl Board {
    fn count_bw(&self) -> (usize, usize) {
        let mut black: usize = 0;
        let mut white: usize = 0;
        for row in self.0.iter() {
            for cell in row {
                match *cell {
                    Some(Face::Black) => black += 1,
                    Some(Face::White) => white += 1,
                    _ => (),
                }
            }
        }
        (black, white)
    }

    fn print_count(&self) -> () {
        let counts = self.count_bw();
        println!("black : {}  white : {}", counts.0, counts.1);
    }
}


trait Monoid {
    fn mempty () -> Self;
    fn madd (&self, &Self) -> Self;
}

impl Monoid for (i8,i8) {
    fn mempty () -> (i8,i8) { (0,0) }
    fn madd(&self,v1:&(i8,i8)) -> (i8,i8) { addv(self,v1) }
}

fn addv(v0:&(i8,i8), v1:&(i8,i8)) -> (i8,i8) {
    (v0.0+v1.0, v0.1+v1.1)
}
