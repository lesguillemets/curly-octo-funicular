mod base;
mod board;
mod face;

use std::fmt;
use std::io;
use board::Board;
use face::Face;
use base::*;

fn main() {
    let board: Board = Board::initial();
    println!("{}", board);
    main_loop(&board, Face::Black);
}

fn main_loop(board: &Board, player: Face) -> () {
    let hand = read_hand();
    match board.place(player, hand) {
        Ok(b) => {
            println!("{}", b);
            main_loop(&b, player.not())
        }
        Err(e) => {
            println!("{}", e);
            main_loop(board, player)
        }
    }
}

fn read_loc() -> Option<(i8, i8)> {
    // TODO: only valid for SIZE < 10
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("read error");
    let mut ch = buf.chars();
    match (ch.next(), ch.next()) {
        (Some(r), Some(c)) => from_char(r, c),
        _ => None,
    }
}

fn from_char(r: char, c: char) -> Option<(i8, i8)> {
    let row = r as usize;
    let col = c as usize;
    if row >= 97 && row < 97 + SIZE && col >= 48 && col < 48 + SIZE {
        Some((col as i8 - 48, row as i8 - 97))
    } else {
        None
    }
}

fn read_hand() -> (i8, i8) {
    loop {
        if let Some(hand) = read_loc() {
            println!("{:?}", hand);
            return hand;
        } else {
            println!("invalid input");
        }
    }
}
