use std::rc::Rc;
use std::cell::RefCell;

use sm_checkers_base::Color;

use sm_checkers_players::*;
use crate::cyclic_iterator::CyclicIterator;

use crate::checkers_ui::CheckersUi;
use crate::checkers_ui_text::CheckersUiText;

use crate::checkers_game::CheckersGame;
use crate::checkers_game::Subject;


mod checkers_game;

mod checkers_ui;
mod checkers_ui_text;

mod cyclic_iterator;


fn main() {


    // Create UI
    let gui = Rc::new(RefCell::new(CheckersUiText::new()));
    gui.borrow_mut().splash_screen();

    // Create game
    let mut game = CheckersGame::new();

    // Create Players
    //let human = Rc::new(RefCell::new(PlayerHumanConsole::new("Player 1", Color::Black)));
    //let bot1 = Rc::new(RefCell::new(PlayerBotRandom::new("ZE BOT A", Color::Black)));
    //let bot2 = Rc::new(RefCell::new(PlayerBotRandom::new("ZE BOT II", Color::Red)));
    let bot1 = Rc::new(RefCell::new(PlayerBotAI::new("AI BOT 1", Color::Black)));
    let bot2 = Rc::new(RefCell::new(PlayerBotAI::new("AI BOT 2", Color::Red)));

    game.register_observer(gui.clone());
    //game.register_observer(human.clone()); 
    game.register_observer(bot1.clone()); 
    game.register_observer(bot2.clone()); 


    //let players: Vec<Rc<RefCell<dyn Player>>> = vec![human.clone(), bot.clone()];
    let players: Vec<Rc<RefCell<dyn Player>>> = vec![bot1.clone(), bot2.clone()];

    let mut players_cyclic_iter = CyclicIterator::new(&players);
    let mut nb_turns = 0;
    for player in players_cyclic_iter.by_ref() {
        if game.is_game_over((*player).borrow().get_color()) {
            println!("{} has lost!", (*player).borrow().get_name());
            println!("{} has won!", (players_cyclic_iter.next()).unwrap().borrow().get_name());
            println!("Number of turns: {}", nb_turns);
            break;
        }

        let mut action_valid = false;
        while !action_valid {
            println!("{}'s turn - You have the {:?} pieces", (*player).borrow().get_name(), (*player).borrow().get_color());

            let ac = player.borrow().play_turn();
            if ac.as_any().downcast_ref::<player_actions::ActionMove>().is_some() {
                if let Some(ac_move) = ac.as_any().downcast_ref::<player_actions::ActionMove>() {
                    println!("Move: {:?}", ac_move);
                    match game.move_piece(ac_move) {
                        Ok(_) => {
                            action_valid = true;
                            nb_turns += 1;
                        }
                        Err(e) => {
                            println!("Your move was invalid: {}", e);
                        }
                    }
                }
            }
            else if ac.as_any().downcast_ref::<player_actions::ActionQuit>().is_some() {
                if let Some(ac_quit) = ac.as_any().downcast_ref::<player_actions::ActionQuit>() {
                    println!("Quit: {:?}", ac_quit);
                    println!("Bye!");
                    std::process::exit(0);
                }
            }              
        }
    }
    println!("GAME OVER");
}

