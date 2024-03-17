use std::collections::LinkedList;

pub enum ActionType {
    Quit,
    Move
}
pub trait GameAction {
    fn get_type(&self) -> ActionType;
}

pub struct ActionQuit {}

impl GameAction for ActionQuit {
    fn get_type(&self) -> ActionType {
        ActionType::Quit
    }
}

impl ActionQuit {
    pub fn new() -> ActionQuit {
        ActionQuit {}
    }
}


pub struct ActionMove {
    pub list: LinkedList<u32>
}

impl GameAction for ActionMove {
    fn get_type(&self) -> ActionType {
        ActionType::Move
    }
}

impl ActionMove {
    pub fn new() -> ActionMove {
        ActionMove {
            list: LinkedList::new()
        }
    }
}

