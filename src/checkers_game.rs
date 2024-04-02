use std::rc::Rc;
use std::cell::RefCell;
use crate::checkers_board::*;
use crate::movements::*;
use crate::player_trait::PlayerColor;
use crate::game_actions::ActionMove;
use crate::checkers_rules::*;


// Define the Subject trait
pub trait Subject {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn GameBoardObserver>>);
    fn remove_observer(&mut self, bo: Rc<RefCell<dyn GameBoardObserver>>);
    fn notify_observers(&self);
}


impl Subject for CheckersGame {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn GameBoardObserver>>) {
        self.observers.push(bo);
    }

    fn remove_observer(&mut self, bo: Rc<RefCell<dyn GameBoardObserver>>) {
        let index = self.observers.iter().position(|o| Rc::ptr_eq(o, &bo));

        if let Some(index) = index {
            self.observers.remove(index);
        }
    }

    fn notify_observers(&self) {
        for observer in self.observers.iter() {
            // Call the update method of the observer
            observer.borrow_mut().update(&self.game_board);
        }
    }
}

pub struct CheckersGame {
    observers: Vec<Rc<RefCell<dyn GameBoardObserver>>>,
    game_board: CheckersBoard
}

impl CheckersGame {
    pub fn new() -> Self {
        CheckersGame {
        observers: Vec::new(),
        game_board: CheckersBoard::new()
        }
    }

    pub fn move_piece(&mut self, action: &ActionMove) -> Result<(), String> {

        let res = self.is_move_valid(action)?;
        if res.as_any().downcast_ref::<Shift>().is_some() {
            if let Some(shift) = res.as_any().downcast_ref::<Shift>() {
                self.move_src_to_dst(shift.from(), shift.to);
            }
        }
        if res.as_any().downcast_ref::<Jump>().is_some() {
            if let Some(jump) = res.as_any().downcast_ref::<Jump>() {
                let tile_eaten = CheckersRules::get_eaten_tile_index(jump.from(), jump.to[0]);
                self.game_board.tiles[tile_eaten] = TileState::Empty;
                self.move_src_to_dst(jump.from(), *jump.to.last().unwrap());
                for i in 0..jump.to.len() - 1 {
                    let tile_eaten = CheckersRules::get_eaten_tile_index(jump.to[i], jump.to[i+1]);
                    self.game_board.tiles[tile_eaten] = TileState::Empty;
                    self.game_board.tiles[jump.to[i]] = TileState::Empty;
                }
            }
        }

        self.notify_observers();
        return Ok(());
    }

    fn move_src_to_dst(&mut self, src: usize, dst: usize) {
        self.game_board.tiles[dst] = self.game_board.tiles[src];
        if dst > 27 && self.game_board.tiles[dst] == TileState::BlackMan {
            self.game_board.tiles[dst] = TileState::BlackKnight;
        }
        if dst < 4 && self.game_board.tiles[dst] == TileState::RedMan {
            self.game_board.tiles[dst] = TileState::RedKnight;
        }
        self.game_board.tiles[src] = TileState::Empty;
    }

    fn is_move_valid(&mut self, action: &ActionMove) -> Result<Box<dyn Movement>, String> {
        // Verify if the action has enough tiles
        if (*action).tiles.len() < 2 {
            return Err("The action does not have at least two tiles (soruce and destination)".into());
        }

        // Verify if tile indexes are in the correct range
        for t in &(*action).tiles {
            if *t > 31 {
                return Err("Tile index is out of range.".into());
            }
        }

        // Verify if the source tile of the action is the right color
        let src = (*action).tiles[0];
        if action.player_color == PlayerColor::Black {
            if (self.game_board.tiles[src] != TileState::BlackMan) && (self.game_board.tiles[src] != TileState::BlackKnight) {
                return Err("Player is not moving a black piece.".into());
            }
        }
        else {
            if (self.game_board.tiles[src] != TileState::RedMan) && (self.game_board.tiles[src] != TileState::RedKnight) {
                return Err("Player is not moving a red piece.".into());
            }
        }
        
        // Verify if all destination tiles are empty
        for (_index, &element) in (*action).tiles.iter().enumerate().skip(1) {
            if self.game_board.tiles[element] != TileState::Empty {
                return Err("Destination tile is not empty.".into());
            }
        }

        // Verify if this is a shift, and if it is a valid shift
        let src = (*action).tiles[0];
        let dst = (*action).tiles[1];
        if ((*action).tiles.len() == 2) &&
            (((dst > src) && (dst - src) < 6) ||
             ((dst < src) && (src - dst) < 6)) {
            let cur_shift = Shift::new(src, dst);
            //let possible_shifts = self.get_possible_shifts(src);
            let possible_shifts = CheckersRules::get_possible_shifts(&self.game_board, src);
            if !possible_shifts.contains(&cur_shift) {
                return Err("Invalid shift.".into());
            }
            return Ok(Box::new(cur_shift));
        }

        // This is a jump. Verify if it is a valid jump
        let the_jump = Jump::new(src, &((*action).tiles[1..]).to_vec());
        if CheckersRules::is_jump_valid(&self.game_board, &the_jump) {
            return Ok(Box::new(the_jump));            
        }
        return Err("Invalid action.".into());
    }


    pub fn is_game_over(&self, next_player_color: PlayerColor) -> bool {
        let pieces = CheckersRules::get_player_pieces_indexes(&self.game_board, next_player_color);
        for p in pieces.iter() {
            if CheckersRules::get_possible_shifts(&self.game_board, *p).len() > 0 {
                return false
            }
        }
        for p in pieces.iter() {
            if CheckersRules::get_possible_jumps(&self.game_board, *p).len() > 0 {
                return false
            }
        }
        return true
    }

}


////////////////////////////////////////////////////////////////////////////////
/// Unit tests
/// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_move_invalid() {
        let mut game = CheckersGame::new();

        // Invalid array
        let action = ActionMove::new(PlayerColor::Black, &vec![]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![0]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![28, 32]);
        assert!(game.is_move_valid(&action).is_err());

        // Invalid piece type for player
        let action = ActionMove::new(PlayerColor::Black, &vec![12, 16]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![20, 16]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Red, &vec![8, 12]);
        assert!(game.is_move_valid(&action).is_err());

        // Invalid move
        game.game_board.tiles[5] = TileState::Empty;
        game.game_board.tiles[6] = TileState::Empty;
        game.game_board.tiles[8] = TileState::Empty;
        game.game_board.tiles[10] = TileState::Empty;
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 5]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 6]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 8]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 10]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[5] = TileState::BlackMan;
        game.game_board.tiles[6] = TileState::BlackMan;
        game.game_board.tiles[8] = TileState::BlackMan;
        game.game_board.tiles[10] = TileState::BlackMan;

        // Blocked move
        let action = ActionMove::new(PlayerColor::Black, &vec![5, 8]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![5, 9]);
        assert!(game.is_move_valid(&action).is_err());

        // Invalid jump
        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[9] = TileState::BlackMan;
        game.game_board.tiles[5] = TileState::RedMan;
        game.game_board.tiles[6] = TileState::RedMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 0]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 2]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[9] = TileState::RedMan;
        game.game_board.tiles[13] = TileState::RedMan;
        game.game_board.tiles[14] = TileState::BlackKnight;
        let action = ActionMove::new(PlayerColor::Red, &vec![9, 16]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Red, &vec![9, 18]);
        assert!(game.is_move_valid(&action).is_err());


        // Blocked jump
        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[1] = TileState::BlackMan;
        game.game_board.tiles[5] = TileState::RedMan;
        game.game_board.tiles[6] = TileState::RedMan;
        game.game_board.tiles[8] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![1, 8]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[8] = TileState::Empty;
        game.game_board.tiles[10] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![1, 10]);
        assert!(game.is_move_valid(&action).is_err());

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[17] = TileState::RedKnight;
        game.game_board.tiles[13] = TileState::BlackKnight;
        game.game_board.tiles[14] = TileState::BlackKnight;
        game.game_board.tiles[21] = TileState::BlackKnight;
        game.game_board.tiles[22] = TileState::BlackKnight;
        game.game_board.tiles[8] = TileState::RedMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![17, 8]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[8] = TileState::Empty;
        game.game_board.tiles[10] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![17, 10]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[10] = TileState::Empty;
        game.game_board.tiles[24] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![17, 24]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[24] = TileState::Empty;
        game.game_board.tiles[26] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![17, 26]);
        assert!(game.is_move_valid(&action).is_err());

    }

    #[test]
    fn test_is_move_valid() {
        let mut game = CheckersGame::new();

        let action = ActionMove::new(PlayerColor::Black, &vec![8, 12]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Red, &vec![21, 17]);
        assert!(game.is_move_valid(&action).is_ok());

        game.game_board.tiles[13] = TileState::RedMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![8, 17]);
        assert!(game.is_move_valid(&action).is_ok());
        game.game_board.tiles[13] = TileState::Empty;

        game.game_board.tiles[16] = TileState::BlackKnight;
        let action = ActionMove::new(PlayerColor::Red, &vec![20, 13]);
        assert!(game.is_move_valid(&action).is_ok());

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[22] = TileState::RedKnight;
        game.game_board.tiles[17] = TileState::BlackMan;
        game.game_board.tiles[18] = TileState::BlackMan;
        game.game_board.tiles[25] = TileState::BlackMan;
        game.game_board.tiles[26] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![22, 13]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Red, &vec![22, 15]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Red, &vec![22, 29]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Red, &vec![22, 31]);
        assert!(game.is_move_valid(&action).is_ok());

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[8] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![8, 13]);
        assert!(game.is_move_valid(&action).is_ok());


    }

    #[test]
    fn test_move_action() {
        let mut game = CheckersGame::new();

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[24] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![24, 28]);
        assert!(game.move_piece(&action).is_ok());
        assert!(game.game_board.tiles[28] == TileState::BlackKnight);

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[5] = TileState::RedMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![5, 1]);
        assert!(game.move_piece(&action).is_ok());
        assert!(game.game_board.tiles[1] == TileState::RedKnight);
}


    #[test]
    fn test_is_multi_jump_valid() {
        let mut game = CheckersGame::new();

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[0] = TileState::BlackMan;
        game.game_board.tiles[5] = TileState::RedMan;
        game.game_board.tiles[14] = TileState::RedKnight;
        game.game_board.tiles[13] = TileState::RedKnight;
        let action = ActionMove::new(PlayerColor::Black, &vec![0, 9, 18]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Black, &vec![0, 9, 16]);
        assert!(game.is_move_valid(&action).is_ok());

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[20] = TileState::RedKnight;
        game.game_board.tiles[16] = TileState::BlackMan;
        game.game_board.tiles[17] = TileState::BlackKnight;
        game.game_board.tiles[18] = TileState::BlackMan;
        game.game_board.tiles[10] = TileState::BlackKnight;
        let action = ActionMove::new(PlayerColor::Red, &vec![20, 13, 22, 15, 6]);
        assert!(game.is_move_valid(&action).is_ok());
    }


    #[test]
    fn test_is_game_over() {
        let mut game = CheckersGame::new();

        // Test #1: default game
        assert_eq!(game.is_game_over(PlayerColor::Black), false);
        assert_eq!(game.is_game_over(PlayerColor::Red), false);

        // Test #2: empty game: game is over
        game.game_board.tiles.fill(TileState::Empty);
        assert_eq!(game.is_game_over(PlayerColor::Black), true);
        assert_eq!(game.is_game_over(PlayerColor::Red), true);

        // Test #3: Only one man blocked
        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[0] = TileState::BlackMan;
        game.game_board.tiles[4] = TileState::RedMan;
        game.game_board.tiles[5] = TileState::RedMan;
        game.game_board.tiles[9] = TileState::RedMan;
        assert_eq!(game.is_game_over(PlayerColor::Black), true);

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[1] = TileState::BlackMan;
        game.game_board.tiles[5] = TileState::RedMan;
        game.game_board.tiles[6] = TileState::RedMan;
        game.game_board.tiles[8] = TileState::RedMan;
        game.game_board.tiles[10] = TileState::RedMan;
        assert_eq!(game.is_game_over(PlayerColor::Black), true);

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[3] = TileState::BlackMan;
        game.game_board.tiles[7] = TileState::RedMan;
        game.game_board.tiles[10] = TileState::RedMan;
        assert_eq!(game.is_game_over(PlayerColor::Black), true);

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[4] = TileState::BlackMan;
        game.game_board.tiles[6] = TileState::BlackMan;
        game.game_board.tiles[8] = TileState::RedMan;
        assert_eq!(game.is_game_over(PlayerColor::Black), false);

    }

}