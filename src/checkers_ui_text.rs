extern crate ansi_term;

use ansi_term::Colour::Red;

use crate::checkers_ui::CheckersUi;
use crate::checkers_board::*;


const WHITE_MAN: char = '\u{26C0}';
const WHITE_KNIGHT: char = '\u{26C1}';
const BLACK_MAN: char = '\u{26C2}';
const BLACK_KNIGHT: char = '\u{26C3}';
//const BLACK_SQUARE: char = '\u{2BC0}';
const WHITE_SQUARE: char = '\u{2610}';
const EMPTY_TILE: char = ' ';

fn print_empty_board() {
    let l0 = String::from(format!("| {} | {} ", EMPTY_TILE, WHITE_SQUARE));
    let l1 = String::from(format!("| {} | {} ", WHITE_SQUARE, EMPTY_TILE));
    let mut line0 = String::new();
    let mut line1 = String::new();
    for _ in 0..4 {
        line0.push_str(l0.as_str());
        line1.push_str(l1.as_str());
    }
    line0.push('|');
    line1.push('|');
    for _ in 0..4 {
        println!("{}", line0);
        println!("{}", line1);
    }
}


pub struct CheckersUiText {
    // fields go here
}

impl CheckersUiText {
    pub fn new() -> CheckersUiText {
        CheckersUiText {
            // fields go here
        }
    }

    fn print_tile(&self, ts: &TileState) {
        match ts {
            TileState::Empty => {
                print!("{} ", WHITE_SQUARE);
            }
            TileState::RedMan => {
                print!("{} ", Red.paint(WHITE_MAN.to_string()));
            }
            TileState::RedKnight => {
                print!("{} ", Red.paint(WHITE_KNIGHT.to_string()));
            }
            TileState::BlackMan => {
                print!("{} ", BLACK_MAN);
            }
            TileState::BlackKnight => {
                print!("{} ", BLACK_KNIGHT);
            }
        }
    }
}


impl GameBoardObserver for CheckersUiText {
    fn update(&mut self, bc: &CheckersBoard) {
        self.draw_board(bc);
    }
}


impl CheckersUi for CheckersUiText {
    fn splash_screen(&self) {
        println!("SM-Checkers v{} - {} {} {} {}",  env!("CARGO_PKG_VERSION"), WHITE_MAN, BLACK_MAN, WHITE_KNIGHT, BLACK_KNIGHT);
        print_empty_board();
   }

    fn draw_board(&self, bc: &CheckersBoard) {
        println!("");
        for i in 0..64 {
            print!("| ");
            if (i / 8) % 2 == 0 {
                if i % 2 == 0 {
                    print!("{} ", EMPTY_TILE);
                }
                else {
                    self.print_tile(&bc.tiles[i/2]);
                }
            }
            else {
                if i % 2 == 1 {
                    print!("{} ", EMPTY_TILE);
                }
                else {
                    self.print_tile(&bc.tiles[i/2]);
                }
                
            }
            if i % 8 == 7 {
                println!("|");
            }
        }
    }
}