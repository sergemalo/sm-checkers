use std::rc::Rc;
use std::cell::RefCell;
use crate::board_content::*;
use crate::move_piece::*;


// Define the Subject trait
pub trait Subject {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>);
    fn remove_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>);
    fn notify_observers(&self);
}

pub struct Board {
    observers: Vec<Rc<RefCell<dyn BoardObserver>>>,
    bc: BoardContent
}

impl Board {
    pub fn new() -> Self {
        Board {
        observers: Vec::new(),
        bc: BoardContent::new()
        }
    }

    pub fn doit(&self) {
        self.notify_observers();
    }

    pub fn is_game_over(&self, next_player_color: PlayerColor) -> bool {
        let pieces = self.get_player_pieces_indexes(next_player_color);
        for (i, _p) in pieces.iter().enumerate(){
            if self.get_possible_moves(i).is_some() {
                return false
            }
        }
        for (i, _p) in pieces.iter().enumerate(){
            if self.get_possible_jumps(i).is_some() {
                return false
            }
        }
        return true
    }

    fn get_player_pieces_indexes(&self, player_color: PlayerColor) -> Vec<usize> {
        let mut player_pieces_indexes = Vec::new();
        for (i, tile) in self.bc.tiles.iter().enumerate() {
            if (player_color == PlayerColor::Black && (*tile == TileState::BlackMan || *tile == TileState::BlackKnight)) || 
               (player_color == PlayerColor::Red && (*tile == TileState::RedMan || *tile == TileState::RedKnight)) {
                player_pieces_indexes.push(i);
            }
        }
        return player_pieces_indexes
    }

    /*
    fn can_move(&self, index: usize) -> bool {
        match self.bc.tiles[index] {
            TileState::BlackMan => self.can_move_black_man(index),
            TileState::BlackKnight => self.can_move_black_knight(index),
            TileState::RedMan => self.can_move_red_man(index),
            TileState::RedKnight => self.can_move_red_knight(index),
            _ => false
        }
    }

    fn can_move_black_man(&self, index: usize) -> bool {
        if index < 9 {
            return false
        }
        if index % 9 == 0 {
            return false
        }
        if index % 9 == 8 {
            return false
        }
        return true
    }
    */
    pub fn get_possible_moves(&self, index: usize) -> Option<Vec<Move>> {
        if index > 31 {
            panic!("Board::get_possible_moves: Index out of bounds");
        }
        return None
    }
    pub fn get_possible_jumps(&self, index: usize) -> Option<Vec<Jump>> {
        if index > 31 {
            panic!("Board::get_possible_jumps: Index out of bounds");
        }
        return None
    }

}


impl Subject for Board {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>) {
        self.observers.push(bo);
    }

    fn remove_observer(&mut self, bo: Rc<RefCell<dyn BoardObserver>>) {
        let index = self.observers.iter().position(|o| Rc::ptr_eq(o, &bo));

        if let Some(index) = index {
            self.observers.remove(index);
        }
    }

    fn notify_observers(&self) {
        for observer in self.observers.iter() {
            observer.borrow().update(&self.bc);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_game_over() {
        let mut board = Board::new();

        // Test #1: default board
        assert_eq!(board.is_game_over(PlayerColor::Black), false);
        assert_eq!(board.is_game_over(PlayerColor::Red), false);

        // Test #2: empty board: game is over
        board.bc.tiles.fill(TileState::Empty);
        assert_eq!(board.is_game_over(PlayerColor::Black), true);
        assert_eq!(board.is_game_over(PlayerColor::Red), true);

        // Test #3: Only one man blocked
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[0] = TileState::BlackMan;
        board.bc.tiles[4] = TileState::RedMan;
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[9] = TileState::RedMan;
        assert_eq!(board.is_game_over(PlayerColor::Black), true);

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[1] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[6] = TileState::RedMan;
        board.bc.tiles[8] = TileState::RedMan;
        board.bc.tiles[10] = TileState::RedMan;
        assert_eq!(board.is_game_over(PlayerColor::Black), true);

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[7] = TileState::RedMan;
        board.bc.tiles[10] = TileState::RedMan;
        assert_eq!(board.is_game_over(PlayerColor::Black), true);


    }

    #[test]
    fn test_get_possible_moves() {
        let mut board = Board::new();

        // Test #1: default board
        for i in 0..8 {
            assert_eq!(board.get_possible_moves(i), None);
        }
        for i in 12..20 {
            assert_eq!(board.get_possible_moves(i), None);
        }
        for i in 16..32 {
            assert_eq!(board.get_possible_moves(i), None);
        }
        let test_cases = [
            (8, vec![12, 13]),
            (9, vec![13, 14]),
            (10, vec![14, 15]),
            (11, vec![15]), // Note that (11, vec![15]) has only one move.
            (20, vec![16]), // Note that (20, vec![16]) has only one move.
            (21, vec![16, 17]),
            (22, vec![17, 18]),
            (23, vec![18, 19]),
        ];        
        for &(start, ref expected_moves) in &test_cases {
            let moves = board.get_possible_moves(start).expect("Expected Some, got None");
            assert_eq!(moves.len(), expected_moves.len(), "Mismatch in number of moves for position {}", start);
        
            for &to in expected_moves {
                assert!(moves.contains(&Move::new(start, to)), "Move from {} to {} not found", start, to);
            }
        }
 
        // Test #2: only one BlackMan at index 0
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[0] = TileState::BlackMan;
        let moves = board.get_possible_moves(0).expect("Expected Some, got None");
        assert_eq!(moves.len(), 2, "Mismatch in number of moves for position 0");
        assert!(moves.contains(&Move::new(0, 4)), "Move from {} to {} not found", 0, 4);
        assert!(moves.contains(&Move::new(0, 5)), "Move from {} to {} not found", 0, 5);

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[0] = TileState::BlackMan;
        board.bc.tiles[4] = TileState::BlackMan;
        let moves = board.get_possible_moves(0).expect("Expected Some, got None");
        assert_eq!(moves.len(), 1, "Mismatch in number of moves for position 0");
        assert!(moves.contains(&Move::new(0, 5)), "Move from {} to {} not found", 0, 5);

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[0] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::BlackMan;
        let moves = board.get_possible_moves(0).expect("Expected Some, got None");
        assert_eq!(moves.len(), 1, "Mismatch in number of moves for position 0");
        assert!(moves.contains(&Move::new(0, 4)), "Move from {} to {} not found", 0, 4);

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[0] = TileState::BlackMan;
        board.bc.tiles[4] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::BlackMan;
        assert_eq!(board.get_possible_moves(0), None);

        // Test #3: only one BlackMan at index 3
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[3] = TileState::BlackMan;
        let moves = board.get_possible_moves(3).expect("Expected Some, got None");
        assert_eq!(moves.len(), 1, "Mismatch in number of moves for position 0");
        assert!(moves.contains(&Move::new(3, 7)), "Move from {} to {} not found", 0, 4);

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[7] = TileState::BlackMan;
        assert_eq!(board.get_possible_moves(0), None);        



    }

    // more tests
}