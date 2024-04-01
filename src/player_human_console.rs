use std::io;
use std::io::Write;

use crate::player_trait::*;
use crate::checkers_board::*;
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
        println!("{} - Please write move (ex: \"m 1, 2\") or quit with letter \"q\":", self.name);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read Action from Player");
        
        loop {
            match input.to_lowercase().chars().next() {
                Some('q') => {
                    println!("{} - Received quit", self.name);
                    let action = ActionQuit::new();
                    return Box::new(action);
                }
                // Matching:
                // m 1, 2
                // m 8, 12
                // m 28, 32
                // m 1, 10, 19, 26
                // Convert to move(1, vec[2])...
                Some('m') => {
                    while input.chars().next().unwrap().is_digit(10) == false {
                       input.remove(0) ;
                    }
                    println!("{} - Received move: {:?}", self.name, input);
                    let tiles_str: Vec<&str> = input.split(',').collect();
                    let mut tiles_idx: Vec<usize> = Vec::new();
                    for s in tiles_str {
                        if s.trim().is_empty() {
                            continue;
                        }
                        let val = s.trim().parse::<usize>().unwrap();
                        if (val == 0) || (val > 32) {
                            println!("Invalid command: {}", input.trim());
                        }
                        tiles_idx.push(val-1);
                    }
    
                    println!(" --> Received move: {:?}", tiles_idx);
                    let action = ActionMove::new(self.color, &tiles_idx);
                    return Box::new(action);
                }
                _ => {
                    println!("Invalid command: {}", input.trim());
               }            
            }
        }
    }
}

impl GameBoardObserver for PlayerHumanConsole {
    fn update(&mut self, _bc: &CheckersBoard) {
        //println!("{} - Received new board", self.name);
    }

}