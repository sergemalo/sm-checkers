use crate::checkers_ui::CheckersUi;
use crate::board::Board;

pub mod board;
pub mod checkers_ui;
pub mod checkers_ui_text;


fn main() {



    let gui = checkers_ui_text::CheckersUiText {};
    gui.splash_screen();

    // Create game
    // Create Board
    let mut board = Board::new();
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