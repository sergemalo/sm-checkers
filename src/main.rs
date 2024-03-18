use std::rc::Rc;
use std::cell::RefCell;

use crate::checkers_ui::CheckersUi;
use crate::checkers_ui_text::CheckersUiText;

use crate::board::Board;
use crate::board::Subject;

use crate::player_trait::Player;
use crate::player_human_console::PlayerHumanConsole;
use crate::cyclic_iterator::CyclicIterator;

use crate::game_actions::*;

use crate::board_content::PlayerColor;

mod board;
mod checkers_ui;
mod checkers_ui_text;
mod player_human_console;
mod player_trait;
mod board_content;
mod game_actions;
mod cyclic_iterator;


fn main() {


    // Create UI
    let gui = Rc::new(RefCell::new(CheckersUiText::new()));
    gui.borrow_mut().splash_screen();

    // Create game
    // Create Board
    let mut board = Board::new();

    // Create Players
    let mut players = vec![];
    players.push(PlayerHumanConsole::new("Player 1", PlayerColor::Black));
    players.push(PlayerHumanConsole::new("Player 2", PlayerColor::Red));


    // Add UI and Players to the Board's Observers
    board.register_observer(gui.clone());
    board.register_observer(Rc::new(RefCell::new(players[0].clone()))); // TODO: NOT SURE ABOUT THIS - CLONE ?
    board.register_observer(Rc::new(RefCell::new(players[1].clone())));
    board.doit();

    let mut players_cyclic_iter = CyclicIterator::new(&players);
    for player in players_cyclic_iter.by_ref() {
        
        println!("{}'s turn - You have the {:?} pieces", (*player).get_name(), (*player).get_color());

        let ac = player.play_turn();
        match ac.get_type() {
            game_actions::ActionType::Move => {
                //board.move_piece(ac.get_x(), ac.get_y());
                board.doit();
            }
            game_actions::ActionType::Quit => {
                println!("Bye!");
                std::process::exit(0);
            }
        }
        if board.is_game_over() {
            break;
        }
    }

    println!("GAME OVER");
}

