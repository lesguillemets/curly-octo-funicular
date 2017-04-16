use std::fmt;
use std::io;

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
pub enum Face {
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
    pub fn not(&self) -> Self {
        match *self {
            Face::White => Face::Black,
            Face::Black => Face::White,
        }
    }

    pub fn flip(&mut self) -> () {
        // TODO
        *self = self.not();
    }
}
