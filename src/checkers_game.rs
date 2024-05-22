
use std::rc::Rc;
use std::cell::RefCell;

use sm_checkers_base::checkers_board::*;
use sm_checkers_base::checkers_rules::*;
use sm_checkers_base::player_colors::Color;
use sm_checkers_players::player_actions::ActionMove;


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

    pub fn is_game_over(&self, next_player_color: Color) -> bool {
        return self.game_board.is_game_over(next_player_color);
    }

    pub fn move_piece(&mut self, action: &ActionMove) -> Result<(), String> {

        self.is_move_valid(action)?;
        self.game_board.move_piece(&action.to_movement()).unwrap();
        self.notify_observers();
        return Ok(());
    }
    
    pub fn is_move_valid(&self, action: &ActionMove) -> Result<(), String> {
        if (*action).tiles.len() < 2 {
            return Err("The action does not have at least two tiles (soruce and destination)".into());
        }

        for t in &(*action).tiles {
            if *t > 31 {
                return Err("Tile index is out of range.".into());
            }
        }

        let src = (*action).tiles[0];
        if action.player_color == Color::Black {
            if (self.game_board.tiles[src] != TileState::BlackMan) && (self.game_board.tiles[src] != TileState::BlackKnight) {
                return Err("Player is not moving a black piece.".into());
            }
        }
        else {
            if (self.game_board.tiles[src] != TileState::RedMan) && (self.game_board.tiles[src] != TileState::RedKnight) {
                return Err("Player is not moving a red piece.".into());
            }
        }
        
        return CheckersRules::is_movement_valid(&self.game_board, &action.to_movement());
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
        let action = ActionMove::new(Color::Black, &vec![]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Black, &vec![0]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Black, &vec![28, 32]);
        assert!(game.is_move_valid(&action).is_err());

        // Invalid piece type for player
        let action = ActionMove::new(Color::Black, &vec![12, 16]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Black, &vec![20, 16]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Red, &vec![8, 12]);
        assert!(game.is_move_valid(&action).is_err());

        // Invalid move
        game.game_board.tiles[5] = TileState::Empty;
        game.game_board.tiles[6] = TileState::Empty;
        game.game_board.tiles[8] = TileState::Empty;
        game.game_board.tiles[10] = TileState::Empty;
        let action = ActionMove::new(Color::Black, &vec![9, 5]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Black, &vec![9, 6]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Black, &vec![9, 8]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Black, &vec![9, 10]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[5] = TileState::BlackMan;
        game.game_board.tiles[6] = TileState::BlackMan;
        game.game_board.tiles[8] = TileState::BlackMan;
        game.game_board.tiles[10] = TileState::BlackMan;

        // Blocked move
        let action = ActionMove::new(Color::Black, &vec![5, 8]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Black, &vec![5, 9]);
        assert!(game.is_move_valid(&action).is_err());

        // Invalid jump
        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[9] = TileState::BlackMan;
        game.game_board.tiles[5] = TileState::RedMan;
        game.game_board.tiles[6] = TileState::RedMan;
        let action = ActionMove::new(Color::Black, &vec![9, 0]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Black, &vec![9, 2]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[9] = TileState::RedMan;
        game.game_board.tiles[13] = TileState::RedMan;
        game.game_board.tiles[14] = TileState::BlackKnight;
        let action = ActionMove::new(Color::Red, &vec![9, 16]);
        assert!(game.is_move_valid(&action).is_err());
        let action = ActionMove::new(Color::Red, &vec![9, 18]);
        assert!(game.is_move_valid(&action).is_err());


        // Blocked jump
        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[1] = TileState::BlackMan;
        game.game_board.tiles[5] = TileState::RedMan;
        game.game_board.tiles[6] = TileState::RedMan;
        game.game_board.tiles[8] = TileState::BlackMan;
        let action = ActionMove::new(Color::Black, &vec![1, 8]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[8] = TileState::Empty;
        game.game_board.tiles[10] = TileState::BlackMan;
        let action = ActionMove::new(Color::Black, &vec![1, 10]);
        assert!(game.is_move_valid(&action).is_err());

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[17] = TileState::RedKnight;
        game.game_board.tiles[13] = TileState::BlackKnight;
        game.game_board.tiles[14] = TileState::BlackKnight;
        game.game_board.tiles[21] = TileState::BlackKnight;
        game.game_board.tiles[22] = TileState::BlackKnight;
        game.game_board.tiles[8] = TileState::RedMan;
        let action = ActionMove::new(Color::Red, &vec![17, 8]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[8] = TileState::Empty;
        game.game_board.tiles[10] = TileState::BlackMan;
        let action = ActionMove::new(Color::Red, &vec![17, 10]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[10] = TileState::Empty;
        game.game_board.tiles[24] = TileState::BlackMan;
        let action = ActionMove::new(Color::Red, &vec![17, 24]);
        assert!(game.is_move_valid(&action).is_err());
        game.game_board.tiles[24] = TileState::Empty;
        game.game_board.tiles[26] = TileState::BlackMan;
        let action = ActionMove::new(Color::Red, &vec![17, 26]);
        assert!(game.is_move_valid(&action).is_err());

    }

    #[test]
    fn test_is_move_valid() {
        let mut game = CheckersGame::new();

        let action = ActionMove::new(Color::Black, &vec![8, 12]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(Color::Red, &vec![21, 17]);
        assert!(game.is_move_valid(&action).is_ok());

        game.game_board.tiles[13] = TileState::RedMan;
        let action = ActionMove::new(Color::Black, &vec![8, 17]);
        assert!(game.is_move_valid(&action).is_ok());
        game.game_board.tiles[13] = TileState::Empty;

        game.game_board.tiles[16] = TileState::BlackKnight;
        let action = ActionMove::new(Color::Red, &vec![20, 13]);
        assert!(game.is_move_valid(&action).is_ok());

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[22] = TileState::RedKnight;
        game.game_board.tiles[17] = TileState::BlackMan;
        game.game_board.tiles[18] = TileState::BlackMan;
        game.game_board.tiles[25] = TileState::BlackMan;
        game.game_board.tiles[26] = TileState::BlackMan;
        let action = ActionMove::new(Color::Red, &vec![22, 13]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(Color::Red, &vec![22, 15]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(Color::Red, &vec![22, 29]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(Color::Red, &vec![22, 31]);
        assert!(game.is_move_valid(&action).is_ok());

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[8] = TileState::BlackMan;
        let action = ActionMove::new(Color::Black, &vec![8, 13]);
        assert!(game.is_move_valid(&action).is_ok());


    }

    #[test]
    fn test_move_action() {
        let mut game = CheckersGame::new();

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[24] = TileState::BlackMan;
        let action = ActionMove::new(Color::Black, &vec![24, 28]);
        assert!(game.move_piece(&action).is_ok());
        assert!(game.game_board.tiles[28] == TileState::BlackKnight);

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[5] = TileState::RedMan;
        let action = ActionMove::new(Color::Red, &vec![5, 1]);
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
        let action = ActionMove::new(Color::Black, &vec![0, 9, 18]);
        assert!(game.is_move_valid(&action).is_ok());
        let action = ActionMove::new(Color::Black, &vec![0, 9, 16]);
        assert!(game.is_move_valid(&action).is_ok());

        game.game_board.tiles.fill(TileState::Empty);
        game.game_board.tiles[20] = TileState::RedKnight;
        game.game_board.tiles[16] = TileState::BlackMan;
        game.game_board.tiles[17] = TileState::BlackKnight;
        game.game_board.tiles[18] = TileState::BlackMan;
        game.game_board.tiles[10] = TileState::BlackKnight;
        let action = ActionMove::new(Color::Red, &vec![20, 13, 22, 15, 6]);
        assert!(game.is_move_valid(&action).is_ok());
    }

}