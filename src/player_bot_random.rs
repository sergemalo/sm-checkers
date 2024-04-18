use rand::Rng;
use sm_checkers_base::checkers_board::*;

use crate::player_trait::*;
use crate::game_actions::*;
use crate::checkers_rules::*;
use crate::movements::*;

pub struct PlayerBotRandom {
    name: String,
    color: PlayerColor,
    board: CheckersBoard
}

impl PlayerBotRandom {
    pub fn new(name_in: & str, color_in: PlayerColor) -> Self {
        PlayerBotRandom {
            name: name_in.to_owned(),
            color: color_in,
            board: CheckersBoard::new()
        }
    }
}

impl Player for PlayerBotRandom {
    fn get_color(&self) -> PlayerColor {
        self.color.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn play_turn(&self) -> Box<dyn GameAction> {
        // Find all of my pieces
        let mut pieces = CheckersRules::get_player_pieces_indexes(&self.board, self.color);
        //println!("{} - pieces: {:?}", self.name, pieces);
        // Check if a jump is possible, one piece at a time, randomly
        let mut pieces_for_jump = pieces.clone();
        while !pieces_for_jump.is_empty() {
            let rand_piece_idx = rand::thread_rng().gen_range(0..pieces_for_jump.len());
            let jumps = CheckersRules::get_possible_jumps(&self.board, pieces_for_jump[rand_piece_idx]);
            if !jumps.is_empty() {
                let jump_choice = rand::thread_rng().gen_range(0..jumps.len());
                let mut jump_vec = vec![jumps[jump_choice].from()];
                jump_vec.extend(jumps[jump_choice].to.clone());
                let action = ActionMove::new(self.color, &jump_vec);
                println!("{} - jumping: {:?}", self.name, action);
                return Box::new(action);                
            }
            else {
                pieces_for_jump.remove(rand_piece_idx);
            }
        }

        // Check if a shift is possible, one piece at a time, randomly
        while !pieces.is_empty() {
            let rand_piece_idx = rand::thread_rng().gen_range(0..pieces.len());
            let shifts = CheckersRules::get_possible_shifts(&self.board, pieces[rand_piece_idx]);
            if !shifts.is_empty() {
                let shift_choice = rand::thread_rng().gen_range(0..shifts.len());
                let mut shift_vec = vec![shifts[shift_choice].from()];
                shift_vec.push(shifts[shift_choice].to);
                let action = ActionMove::new(self.color, &shift_vec);
                println!("{} - piece idx: {}, choice: {}, shifting: {:?}", self.name, rand_piece_idx, shift_choice, action);
                return Box::new(action);                
            }
            else {
                pieces.remove(rand_piece_idx);
            }
        }

        // Can't jump or can't shift !!
        // Seems like the game should be over...
        // Panicking for now
        panic!("{} - Game should be over, I can't find a move to do.", self.name);
    }
}

impl GameBoardObserver for PlayerBotRandom {
    fn update(&mut self, board: &CheckersBoard) {
        self.board = (*board).clone();
    }

}