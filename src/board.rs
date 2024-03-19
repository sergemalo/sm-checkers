use std::rc::Rc;
use std::cell::RefCell;
use crate::board_content::*;


// Define the Subject trait
pub trait Subject {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>);
    fn remove_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>);
    fn notify_observers(&self);
}

pub struct Board {
    observers: Vec<Rc<RefCell<dyn BoardObserver>>>,
    bc: BoardContent
}

impl Board {
    pub fn new() -> Self {
        Board {
        observers: Vec::new(),
        bc: BoardContent::new()
        }
    }

    pub fn doit(&self) {
        self.notify_observers();
    }

    pub fn is_game_over(&self, next_player_color: PlayerColor) -> bool {
        // Check if player has pieces
        if next_player_color == PlayerColor::Black {
            let mut tile_count = 0;
            for tile in self.bc.tiles.iter() {
                if *tile == TileState::BlackMan {
                    tile_count += 1;
                }
                if *tile == TileState::BlackKnight {
                    tile_count += 1;
                }
            }
            if tile_count == 0 {
                return true
            }
        } else if next_player_color == PlayerColor::Red {
            let mut tile_count = 0;
            for tile in self.bc.tiles.iter() {
                
                if *tile == TileState::RedMan {
                    tile_count += 1;
                }
                if *tile == TileState::RedKnight {
                    tile_count += 1;
                }
            }
            if tile_count == 0 {
                return true
            }
        }
        return false
    }

}


impl Subject for Board {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>) {
        self.observers.push(bo);
    }

    fn remove_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>) {
        let index = self.observers.iter().position(|o| Rc::ptr_eq(o, &bo));

        if let Some(index) = index {
            self.observers.remove(index);
        }
    }

    fn notify_observers(&self) {
        for observer in self.observers.iter() {
            observer.borrow().update(&self.bc);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_game_over() {
        let mut board = Board::new();

        // Test #1: default board
        assert_eq!(board.is_game_over(PlayerColor::Black), false);
        assert_eq!(board.is_game_over(PlayerColor::Red), false);

        // Test #2: empty board: game is over
        board.bc.tiles.fill(TileState::Empty);
        assert_eq!(board.is_game_over(PlayerColor::Black), true);
        assert_eq!(board.is_game_over(PlayerColor::Red), true);

        // Test #3: Only one man blocked
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[0] = TileState::BlackMan;
        board.bc.tiles[4] = TileState::RedMan;
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[9] = TileState::RedMan;
        assert_eq!(board.is_game_over(PlayerColor::Black), true);

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[1] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[6] = TileState::RedMan;
        board.bc.tiles[8] = TileState::RedMan;
        board.bc.tiles[10] = TileState::RedMan;
        assert_eq!(board.is_game_over(PlayerColor::Black), true);

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[7] = TileState::RedMan;
        board.bc.tiles[10] = TileState::RedMan;
        assert_eq!(board.is_game_over(PlayerColor::Black), true);


    }

    // more tests
}