use crate::game_actions::GameAction;

pub trait Player {
    // Returns a GameAction: 
    // The implementation of the play_turn should create a GameAction and move the ownership of the GameAction to the caller
    // In Rust, this is automatic when return a local variable
    fn play_turn(&self) -> Box<dyn GameAction>;
}

