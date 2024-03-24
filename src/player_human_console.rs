use std::io;
use std::io::Write;

use crate::player_trait::*;
use crate::board_content::*;
use crate::game_actions::*;

#[derive(Clone)]
pub struct PlayerHumanConsole {
    name: String,
    color: PlayerColor
}

impl PlayerHumanConsole {
    pub fn new(name_in: & str, color_in: PlayerColor) -> Self {
        PlayerHumanConsole {
            name: name_in.to_owned(),
            color: color_in
        }
    }
}

impl Player for PlayerHumanConsole {
    fn get_color(&self) -> PlayerColor {
        self.color.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn play_turn(&self) -> Box<dyn GameAction> {
        println!("{} - Please write move or quit with letter (q):", self.name);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read Action from Player");
        
        match input.trim().to_lowercase().as_str() {
            "q" => {
                let action = ActionQuit::new();
                return Box::new(action);
            }
            _ => {
                let action = ActionMove::new(self.color, &vec![]);
                return Box::new(action);
            }
        }
    }
}

impl BoardObserver for PlayerHumanConsole {
    fn update(&self, _bc: &BoardContent) {
        println!("{} - Received new board", self.name);
    }

}