use crate::checkers_rules::*;
use crate::movements::*;
use crate::player_colors::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileState {
    Empty,
    RedMan,
    RedKnight,
    BlackMan,
    BlackKnight
}

#[derive(Debug, Clone)]
pub struct CheckersBoard {
    pub tiles: [TileState; 32]
}

impl CheckersBoard {
    pub fn new() -> Self {
        let mut default_tiles: [TileState; 32] = [TileState::Empty; 32];
        for i in 0..12 {
            default_tiles[i] = TileState::BlackMan;
            default_tiles[i+20] = TileState::RedMan;
        }
        CheckersBoard {
            tiles: default_tiles
        }
    }

    pub fn move_piece(&mut self, movement: &Box<dyn Movement>) -> Result<(), String> {

        //CheckersRules::is_movement_valid(self, movement)?;
        if let Some(sh) = movement.as_any().downcast_ref::<Shift>() {
            self.move_src_to_dst(sh.from(), sh.to);
            return Ok(());
        }
        if let Some(ju) = movement.as_any().downcast_ref::<Jump>() {
            let tile_eaten = CheckersRules::get_eaten_tile_index(ju.from(), ju.to[0]);
            self.tiles[tile_eaten] = TileState::Empty;
            self.move_src_to_dst(ju.from(), *ju.to.last().unwrap());
            for i in 0..ju.to.len() - 1 {
                let tile_eaten = CheckersRules::get_eaten_tile_index(ju.to[i], ju.to[i+1]);
                self.tiles[tile_eaten] = TileState::Empty;
                self.tiles[ju.to[i]] = TileState::Empty;
            }
            return Ok(());
        }
        return Err("Unable to move piece".to_string());

        //self.notify_observers();

    }

    fn move_src_to_dst(&mut self, src: usize, dst: usize) {
        self.tiles[dst] = self.tiles[src];
        if dst > 27 && self.tiles[dst] == TileState::BlackMan {
            self.tiles[dst] = TileState::BlackKnight;
        }
        if dst < 4 && self.tiles[dst] == TileState::RedMan {
            self.tiles[dst] = TileState::RedKnight;
        }
        self.tiles[src] = TileState::Empty;
    }

    pub fn is_game_over(&self, next_player_color: Color) -> bool {
        let pieces = CheckersRules::get_player_pieces_indexes(&self, next_player_color);
        for p in pieces.iter() {
            if CheckersRules::get_possible_shifts(&self, *p).len() > 0 {
                return false
            }
        }
        for p in pieces.iter() {
            if CheckersRules::get_possible_jumps(&self, *p).len() > 0 {
                return false
            }
        }
        return true
    }

}

pub trait GameBoardObserver {
    fn update(&mut self, bc: &CheckersBoard);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_game_over() {
        let mut board = CheckersBoard::new();

        // Test #1: default game
        assert_eq!(board.is_game_over(Color::Black), false);
        assert_eq!(board.is_game_over(Color::Red), false);

        // Test #2: empty game: game is over
        board.tiles.fill(TileState::Empty);
        assert_eq!(board.is_game_over(Color::Black), true);
        assert_eq!(board.is_game_over(Color::Red), true);

        // Test #3: Only one man blocked
        board.tiles.fill(TileState::Empty);
        board.tiles[0] = TileState::BlackMan;
        board.tiles[4] = TileState::RedMan;
        board.tiles[5] = TileState::RedMan;
        board.tiles[9] = TileState::RedMan;
        assert_eq!(board.is_game_over(Color::Black), true);

        board.tiles.fill(TileState::Empty);
        board.tiles[1] = TileState::BlackMan;
        board.tiles[5] = TileState::RedMan;
        board.tiles[6] = TileState::RedMan;
        board.tiles[8] = TileState::RedMan;
        board.tiles[10] = TileState::RedMan;
        assert_eq!(board.is_game_over(Color::Black), true);

        board.tiles.fill(TileState::Empty);
        board.tiles[3] = TileState::BlackMan;
        board.tiles[7] = TileState::RedMan;
        board.tiles[10] = TileState::RedMan;
        assert_eq!(board.is_game_over(Color::Black), true);

        board.tiles.fill(TileState::Empty);
        board.tiles[4] = TileState::BlackMan;
        board.tiles[6] = TileState::BlackMan;
        board.tiles[8] = TileState::RedMan;
        assert_eq!(board.is_game_over(Color::Black), false);

    }
}