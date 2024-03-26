use std::any::Any;
use crate::player_trait::PlayerColor;


pub enum ActionType {
    Quit,
    Move
}
pub trait GameAction {
    fn get_type(&self) -> ActionType;
    fn as_any(&self) -> &dyn Any;
}

pub struct ActionQuit {}

impl GameAction for ActionQuit {
    fn get_type(&self) -> ActionType {
        ActionType::Quit
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ActionQuit {
    pub fn new() -> ActionQuit {
        ActionQuit {}
    }
}


pub struct ActionMove {
    pub player_color: PlayerColor,
    pub tiles: Vec<usize>
}

impl GameAction for ActionMove {
    fn get_type(&self) -> ActionType {
        ActionType::Move
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ActionMove {
    pub fn new(player_color: PlayerColor, tiles: &Vec<usize>) -> ActionMove {
        ActionMove {
            player_color,
            tiles: (*tiles).clone()
        }
    }
}

