//mod player_trait;

use std::io;
use std::io::Write;

use crate::player_trait::{Player, PlayerAction};


pub struct PlayerHumanConsole {
    name: String,
}

impl PlayerHumanConsole {
    pub fn new(name_in: & str) -> Self {
        PlayerHumanConsole {
            name: name_in.to_owned()
        }
    }
}

impl Player for PlayerHumanConsole {
    fn play_turn(&self) -> &PlayerAction {
        println!("{}'s turn: quit (q) or move:", self.name);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read Action from Player");
        
        match input.trim().to_lowercase().as_str() {
            "q" => &PlayerAction::Quit,
            _ => &PlayerAction::Move,
        }
    }
}