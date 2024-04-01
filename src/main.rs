use std::rc::Rc;
use std::cell::RefCell;

use crate::checkers_ui::CheckersUi;
use crate::checkers_ui_text::CheckersUiText;

use crate::checkers_game::CheckersGame;
use crate::checkers_game::Subject;

use crate::player_trait::*;
use crate::player_human_console::PlayerHumanConsole;
use crate::player_bot_random::PlayerBotRandom;
use crate::cyclic_iterator::CyclicIterator;


mod checkers_rules;
mod checkers_game;
mod checkers_board;

mod checkers_ui;
mod checkers_ui_text;

mod cyclic_iterator;
mod player_trait;
mod player_human_console;
mod player_bot_random;

mod game_actions;
mod movements;



fn main() {


    // Create UI
    let gui = Rc::new(RefCell::new(CheckersUiText::new()));
    gui.borrow_mut().splash_screen();

    // Create game
    let mut game = CheckersGame::new();

    // Create Players
    let mut players = vec![];
    players.push(PlayerHumanConsole::new("Player 1", PlayerColor::Black));
    players.push(PlayerHumanConsole::new("Player 2", PlayerColor::Red));
    //players.push(PlayerBotRandom::new("ZE BOT", PlayerColor::Red));


    // Add UI and Players to the Board's Observers
    // TODO: NOT SURE ABOUT THIS - CLONE ?
    game.register_observer(gui.clone());
    game.register_observer(Rc::new(RefCell::new(players[0].clone()))); 
    game.register_observer(Rc::new(RefCell::new(players[1].clone())));

    let mut players_cyclic_iter = CyclicIterator::new(&players);
    for player in players_cyclic_iter.by_ref() {
        if game.is_game_over((*player).get_color()) {
            println!("{} has lost!", (*player).get_name());
            println!("{} has won!", (players_cyclic_iter.next()).unwrap().get_name());
            break;
        }

        let mut action_valid = false;
        while !action_valid {
            println!("{}'s turn - You have the {:?} pieces", (*player).get_name(), (*player).get_color());

            let ac = player.play_turn();
            if ac.as_any().downcast_ref::<game_actions::ActionMove>().is_some() {
                if let Some(ac_move) = ac.as_any().downcast_ref::<game_actions::ActionMove>() {
                    println!("Move: {:?}", ac_move);
                    match game.move_piece(ac_move) {
                        Ok(_) => {
                            action_valid = true;
                        }
                        Err(e) => {
                            println!("Your move was invalid: {}", e);
                        }
                    }
                }
            }
            else if ac.as_any().downcast_ref::<game_actions::ActionQuit>().is_some() {
                if let Some(ac_quit) = ac.as_any().downcast_ref::<game_actions::ActionQuit>() {
                    println!("Quit: {:?}", ac_quit);
                    println!("Bye!");
                    std::process::exit(0);
                }
            }              
        }
    }
    println!("GAME OVER");
}

