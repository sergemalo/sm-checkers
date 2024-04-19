use sm_checkers_base::player_colors::Color;
use crate::player_actions::Action;

pub trait Player {
    fn get_name(&self) -> String;
    fn get_color(&self) -> Color;

    // Returns a Action: 
    // The implementation of the play_turn should create a Action and move the ownership of the Action to the caller
    // In Rust, this is automatic when return a local variable
    fn play_turn(&self) -> Box<dyn Action>;
}

