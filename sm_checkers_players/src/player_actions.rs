use std::any::Any;
use sm_checkers_base::player_colors::Color;
use sm_checkers_base::movements::*;


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

    pub fn to_movement(&self) -> Box<dyn Movement> {
        if self.tiles.len() < 2 {
            panic!("Cannot convert ActionMove to a Movement - it must have at least 2 tiles");
        }
        let src = self.tiles[0];
        let dst = self.tiles[1];
        if (self.tiles.len() == 2) &&
            (((dst > src) && (dst - src) < 6) ||
             ((dst < src) && (src - dst) < 6)) {
            return Box::new(Shift::new(src, dst));
        }
        return Box::new(Jump::new(src, &(self.tiles[1..]).to_vec()));
    }
}

