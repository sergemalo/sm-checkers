//mod player_trait;

use std::io;
use std::io::Write;

use crate::player_trait::{Player, PlayerAction};


pub struct PlayerHumanConsole<'a> {
    name: &'a str,
}

impl<'a> PlayerHumanConsole<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> Player for PlayerHumanConsole<'a> {
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