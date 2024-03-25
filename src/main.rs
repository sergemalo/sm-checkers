use std::rc::Rc;
use std::cell::RefCell;

use crate::checkers_ui::CheckersUi;
use crate::checkers_ui_text::CheckersUiText;

use crate::board::Board;
use crate::board::Subject;

use crate::player_trait::*;
use crate::player_human_console::PlayerHumanConsole;
use crate::cyclic_iterator::CyclicIterator;

use crate::game_actions::*;


mod board;
mod checkers_ui;
mod checkers_ui_text;
mod player_human_console;
mod player_trait;
mod board_content;
mod game_actions;
mod cyclic_iterator;
mod movements;



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
    // TODO: NOT SURE ABOUT THIS - CLONE ?
    board.register_observer(gui.clone());
    board.register_observer(Rc::new(RefCell::new(players[0].clone()))); 
    board.register_observer(Rc::new(RefCell::new(players[1].clone())));
    board.doit();

    let mut players_cyclic_iter = CyclicIterator::new(&players);
    for player in players_cyclic_iter.by_ref() {
        if board.is_game_over((*player).get_color()) {
            println!("{} has lost!", (*player).get_name());
            println!("{} has won!", (players_cyclic_iter.next()).unwrap().get_name());
            break;
        }

        /*
        let mut action_valid = false;
        while !action_valid {
            println!("{}'s turn - You have the {:?} pieces", (*player).get_name(), (*player).get_color());

            let ac = player.play_turn();
            if let Some(ac_move) = ac.downcast_ref::<game_actions::ActionMove>() {
                match board.move_piece(ac_move) {
                    Ok(_) => {
                        action_valid = true;
                    }
                    Err(e) => {
                        println!("Your move was invalid: {}", e);
                    }
                }
            }
            else if let Some(ac_quit) = (&*ac).downcast_ref::<game_actions::ActionQuit>() {
                action_valid = true;
                println!("Bye!");
                std::process::exit(0);
            }
              
        }
        */
        println!("Bye!");
        std::process::exit(0);
}

    println!("GAME OVER");
}

