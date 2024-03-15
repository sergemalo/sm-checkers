use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;

use crate::checkers_ui::CheckersUi;
use crate::checkers_ui_text::CheckersUiText;
use crate::board::Board;
use crate::board::Subject;

pub mod board;
pub mod checkers_ui;
pub mod checkers_ui_text;


fn main() {


    let gui = Rc::new(RefCell::new(CheckersUiText::new()));

    gui.borrow_mut().splash_screen();

    // Create game
    // Create Board
    let mut board = Board::new();
    board.register_observer(gui.clone());
    board.doit();
    board.doit();
    // Create UI
    // Create Players
    // Add UI and Players to the Board's Observers
}

/*
pub mod traits;
pub mod structures;
pub mod consumer;

fn main() {
    let my_struct = structures::MyStruct {
        // Initialize fields as necessary
    };

    // Pass a reference to MyStruct as a trait object to the consumer function
    consumer::use_trait_object(&my_struct);
}
*/