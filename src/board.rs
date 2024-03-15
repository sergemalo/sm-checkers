use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Copy, Clone)]
pub enum TileState {
    Empty,
    RedMan,
    RedKnight,
    BlackMan,
    BlackKnight
}

#[derive(Debug)]
pub struct BoardContent {
    pub tiles: [TileState; 32]
}


impl BoardContent {
    fn new() -> Self {
        let mut temp_tiles: [TileState; 32] = [TileState::Empty; 32];
        for i in 0..12 {
            temp_tiles[i] = TileState::BlackMan;
            temp_tiles[i+20] = TileState::RedMan;
        }
        BoardContent {
            tiles: temp_tiles
        }
    }
}

// Define the Observer trait
pub trait BoardObserver {
    fn update(&self, bc: &BoardContent);
}

// Define the Subject trait
pub trait Subject {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>);
    fn remove_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>);
    fn notify_observers(&self);
}

pub struct Board {
    observers: Vec<Rc<RefCell<dyn BoardObserver>>>,
    board_name: String,
    board_tiles: BoardContent
}

impl Board {
    pub fn new() -> Self {
        Board {
        observers: Vec::new(),
        board_name: String::from("ZE Board"),
        board_tiles: BoardContent::new()
        }
    }

    pub fn doit(&self) {
        self.notify_observers();
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