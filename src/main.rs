use std::rc::Rc;
use std::cell::RefCell;

use crate::checkers_ui::CheckersUi;
use crate::checkers_ui_text::CheckersUiText;

use crate::board::Board;
use crate::board::Subject;

use crate::player_human_console::PlayerHumanConsole;

mod board;
mod checkers_ui;
mod checkers_ui_text;
mod player_human_console;
mod player_trait;
mod board_content;

fn main() {


    // Create UI
    let gui = Rc::new(RefCell::new(CheckersUiText::new()));
    gui.borrow_mut().splash_screen();

    // Create game
    // Create Board
    let mut board = Board::new();

    // Create Players
    let player1 = Rc::new(RefCell::new(PlayerHumanConsole::new("Player 1")));
    let player2 = Rc::new(RefCell::new(PlayerHumanConsole::new("Player 2")));

    // Add UI and Players to the Board's Observers
    board.register_observer(gui.clone());
    board.register_observer(player1.clone());
    board.register_observer(player2.clone());
    board.doit();
}

