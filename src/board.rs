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
    board_tiles: BoardContent
}

impl Board {
    pub fn new() -> Self {
        Board {
        observers: Vec::new(),
        board_tiles: BoardContent::new()
        }
    }

    pub fn doit(&self) {
        self.notify_observers();
    }

    pub fn is_game_over(&self) -> bool {
        false
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
            observer.borrow().update(&self.board_tiles);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_game_over() {
        let board = Board::new();
        // perform some operations on the board
        assert_eq!(board.is_game_over(), false);
    }

    // more tests
}