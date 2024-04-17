use rand::Rng;
use crate::player_trait::*;
use crate::game_actions::*;
use crate::checkers_rules::*;
use crate::checkers_board::*;
use crate::movements::*;

pub struct PlayerBotAI {
    name: String,
    color: PlayerColor,
    board: CheckersBoard,
    weights: Vec<f64>,
    eta: f64
}

impl PlayerBotAI {
    pub fn new(name_in: & str, color_in: PlayerColor) -> Self {
        PlayerBotAI {
            name: name_in.to_owned(),
            color: color_in,
            board: CheckersBoard::new(),
            weights: vec![0.0; NB_WEIGHTS],
            eta: 0.1
        }
    }
}

const NB_WEIGHTS: usize = 7;
enum WeightType {
    Bias,
    PlayerPieces,
    OpponentPieces,
    PlayerKnights,
    OpponentKnights,
    OpponentJumps,  // Our number of pieces threatened by the opponnent
    PlayerJumps    // Number of pieces we are theatening
}


impl PlayerBotAI {
    // This is the V'hat function
    fn get_board_value_approx(&self) -> f64 {
        // W0
        let mut result: f64 = self.weights[WeightType::Bias as usize];

        let my_pieces = CheckersRules::get_player_pieces_indexes(&self.board, self.color);
        let mut my_knights_tile_state = TileState::BlackKnight;
        if self.color == PlayerColor::Red {
            my_knights_tile_state = TileState::RedKnight;
        }
        let my_knights_count = my_pieces.iter().filter(|&x| self.board.tiles[*x] == my_knights_tile_state).count();

        let opp_pieces = CheckersRules::get_player_pieces_indexes(&self.board, opposite_color(self.color));
        let mut opp_knights_tile_state = TileState::RedKnight;
        if self.color == PlayerColor::Red {
            opp_knights_tile_state = TileState::BlackKnight;
        }
        let opp_knights_count = opp_pieces.iter().filter(|&x| self.board.tiles[*x] == opp_knights_tile_state).count();
        
        // W1
        result += self.weights[WeightType::PlayerPieces as usize] * my_pieces.len() as f64;
        // W2
        result += self.weights[WeightType::OpponentPieces as usize] * opp_pieces.len() as f64;
        // W3
        result += self.weights[WeightType::PlayerKnights as usize] * my_knights_count as f64;
        // W4
        result += self.weights[WeightType::OpponentKnights as usize] * opp_knights_count as f64;
        // W5
        let mut opp_jumps_count = 0;
        for piece in &opp_pieces {
            opp_jumps_count += CheckersRules::get_possible_jumps(&self.board, *piece).len();
        }
        result += self.weights[WeightType::OpponentJumps as usize] * opp_jumps_count as f64;
        // W6
        let mut my_jumps_count = 0;
        for piece in &my_pieces {
            my_jumps_count += CheckersRules::get_possible_jumps(&self.board, *piece).len();
        }
        result += self.weights[WeightType::PlayerJumps as usize] * my_jumps_count as f64;
        result
    }

    /*
    fn update_weights(&mut self, error: f64) {
        for i in 0..NB_WEIGHTS {
            self.weights[i] += self.eta * error * self.get_attribute_value(i as WeightType);
        }
    }

    fn get_attribute_value(&self, attribute: WeightType) -> f64 {
        match attribute {
            WeightType::Bias => 1.0,
            WeightType::PlayerPieces => 1.0,
            WeightType::OpponentPieces => 1.0,
            WeightType::PlayerKnights => 1.0,
            WeightType::OpponentKnights => 1.0,
            WeightType::OpponentJumps => 1.0,
            WeightType::PlayerJumps => 1.0
        }
    }
    fn choose_move(&self) -> Box<dyn GameAction> {
        
    )}

    */


}

impl Player for PlayerBotAI {
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

impl GameBoardObserver for PlayerBotAI {
    fn update(&mut self, board: &CheckersBoard) {
        self.board = (*board).clone();
    }

}