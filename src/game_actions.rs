use crate::player_trait::PlayerColor;


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
    pub player_color: PlayerColor,
    pub tiles: Vec<usize>
}

impl GameAction for ActionMove {
    fn get_type(&self) -> ActionType {
        ActionType::Move
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

