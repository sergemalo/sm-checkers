use crate::game_actions::GameAction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerColor {
    Red,
    Black
}

pub fn opposite_color(color_in: PlayerColor) -> PlayerColor {
    match color_in {
        PlayerColor::Red => PlayerColor::Black,
        PlayerColor::Black => PlayerColor::Red
    }
}

pub trait Player {
    fn get_name(&self) -> String;
    fn get_color(&self) -> PlayerColor;

    // Returns a GameAction: 
    // The implementation of the play_turn should create a GameAction and move the ownership of the GameAction to the caller
    // In Rust, this is automatic when return a local variable
    fn play_turn(&self) -> Box<dyn GameAction>;
}

