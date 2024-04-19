use std::any::Any;
use sm_checkers_base::player_colors::Color;


pub enum ActionType {
    Quit,
    Move
}
pub trait Action {
    fn get_type(&self) -> ActionType;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct ActionQuit {}

impl Action for ActionQuit {
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


#[derive(Debug)]
pub struct ActionMove {
    pub player_color: Color,
    pub tiles: Vec<usize>
}

impl Action for ActionMove {
    fn get_type(&self) -> ActionType {
        ActionType::Move
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ActionMove {
    pub fn new(player_color: Color, tiles: &Vec<usize>) -> ActionMove {
        ActionMove {
            player_color,
            tiles: (*tiles).clone()
        }
    }
}

