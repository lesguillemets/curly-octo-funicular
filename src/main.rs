use std::fmt;
use std::io;

fn main() {
    let board: Board = Board::initial();
    println!("{}", board);
    main_loop(&board, Face::Black);
}

fn read_hand() -> (i8,i8){
    loop {
        if let Some(hand) = read_loc() {
            println!("{:?}",hand);
            return hand;
        } else {
            println!("invalid input");
        }
    }
}

fn main_loop(board:&Board, player:Face) -> (){
    let hand = read_hand();
    match board.place(player, hand) {
        Ok(b) => {
            println!("{}", b);
            main_loop(&b, player.not())
        }
        Err(e) => {
            println!("{}", e);
            main_loop(board,player)
        }
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

#[derive(Debug, Clone)]
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

impl Board {
    fn assign(&mut self, player: Face, loc: (i8, i8)) -> Result<(), String> {
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
            _ => (),
        }
        let mut b = self.clone();
        let mut turn_any = false;
        for &dir in DIRS {
            let mut cur = loc.madd(dir);
            let mut flips: Vec<(i8, i8)> = Vec::new();
            while let Some(Some(face)) = self.at(cur) {
                if face == player {
                    if !(flips.is_empty()) {
                        turn_any = true;
                        for flip in flips.iter() {
                            // FIXME JUST NOW
                            // println!("{:?}", flip);
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
            b.assign(player, loc).expect("ierror");
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
    fn mempty() -> Self;
    fn madd(&self, Self) -> Self;
}

impl Monoid for (i8, i8) {
    fn mempty() -> (i8, i8) {
        (0, 0)
    }
    fn madd(&self, v1: (i8, i8)) -> (i8, i8) {
        addv(*self, v1)
    }
}

fn addv(v0: (i8, i8), v1: (i8, i8)) -> (i8, i8) {
    (v0.0 + v1.0, v0.1 + v1.1)
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO is this ok?
        let mut buf = String::new();
        buf.push_str(" ");
        for i in 'a' as u8..'a' as u8 + SIZE as u8 {
            buf.push_str(&format!("|{}", i as char));
        }
        buf.push_str("|\n");
        for (i, xs) in self.0.iter().enumerate() {
            buf.push_str(&format!("{}|", i));
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

fn read_loc() -> Option<(i8,i8)>{
    // TODO: only valid for SIZE < 10
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("read error");
    let mut ch = buf.chars();
    match (ch.next(), ch.next()){
        (Some(r), Some(c)) => from_char(r,c),
        _ => None
    }
}

fn from_char(r:char, c:char) -> Option<(i8,i8)> {
    let row = r as usize;
    let col = c as usize;
    if row >= 97 && row < 97+SIZE && col >= 48 && col < 48 + SIZE {
        Some((col as i8 - 48 , row as i8 -97))
    } else {
        None
    }
}
