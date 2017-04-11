use std::fmt;

const SIZE: usize = 8;

#[derive(Debug,Clone,Copy)]
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

struct Board(pub [[Option<Face>; SIZE]; SIZE]);

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

fn main() {
    println!("{}", Board::initial());
}