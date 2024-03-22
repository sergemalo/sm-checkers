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
            if self.get_possible_moves(i).len() > 0 {
                return false
            }
        }
        for (i, _p) in pieces.iter().enumerate(){
            if self.get_possible_jumps(i).len() > 0 {
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

    pub fn get_possible_moves(&self, index: usize) -> Vec<Move> {
        if index > 31 {
            panic!("Board::get_possible_moves: Index out of bounds");
        }
        match self.bc.tiles[index] {
            TileState::BlackMan => {
                return self.get_possible_moves_for_piece(index, TileState::BlackMan);
            }
            TileState::BlackKnight | TileState::RedKnight => {
                let mut moves: Vec<Move>  = vec![];
                let mut moves2 = self.get_possible_moves_for_piece(index, TileState::BlackMan);
                moves.append(&mut moves2);
                let mut moves3 = self.get_possible_moves_for_piece(index, TileState::RedMan);
                moves.append(&mut moves3);
                return moves;
            }
            TileState::RedMan => {
                return self.get_possible_moves_for_piece(index, TileState::RedMan);
            }
            _ => {
                return vec![];
            }
        }
    }

    fn get_possible_moves_for_piece(&self, index: usize, state: TileState) -> Vec<Move> {
        match state {
            TileState::BlackMan => {
                if index > 27 {
                    return vec![];
                }
                if ((index % 8) == 3) || ((index % 8) == 4) {
                    if self.bc.tiles[index+4] == TileState::Empty {
                        return vec![Move::new(index, index+4)];
                    }
                    return vec![];
                }
                else {
                    let mut moves = vec![];
                    if (index % 8) > 3 {
                        if self.bc.tiles[index+3] == TileState::Empty {
                            moves.push(Move::new(index, index+3));
                        }
                    }
                    if self.bc.tiles[index+4] == TileState::Empty {
                        moves.push(Move::new(index, index+4));
                    }
                    if (index % 8) < 4 {
                        if self.bc.tiles[index+5] == TileState::Empty {
                            moves.push(Move::new(index, index+5));
                        }
                    }
                    return moves;
                }                        
            }
            TileState::RedMan => {
                if index < 4 {
                    return vec![];
                }
                if ((index % 8) == 3) || ((index % 8) == 4) {
                    if self.bc.tiles[index-4] == TileState::Empty {
                        return vec![Move::new(index, index-4)];
                    }
                    return vec![];
                }
                else {
                    let mut moves = vec![];

                    if (index % 8) > 3 {
                        if self.bc.tiles[index-5] == TileState::Empty {
                            moves.push(Move::new(index, index-5));
                        }
                    }
                    if self.bc.tiles[index-4] == TileState::Empty {
                        moves.push(Move::new(index, index-4));
                    }
                    if (index % 8) < 4 {
                        if self.bc.tiles[index-3] == TileState::Empty {
                            moves.push(Move::new(index, index-3));
                        }
                    }
                    return moves;
                }                        
            }
            _ => {
                panic!("Board::get_possible_moves_for_piece: Invalid piece state!");
            }
        }
    }

    pub fn get_possible_jumps(&self, index: usize) -> Vec<Jump> {
        if index > 31 {
            panic!("Board::get_possible_jumps: Index out of bounds");
        }
        return vec![];
    }

}


////////////////////////////////////////////////////////////////////////////////
/// Unit tests
/// 
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
        let board = Board::new();

        // Test #1: default board
        for i in 0..8 {
            assert_eq!(board.get_possible_moves(i), &[]);
        }
        for i in 12..20 {
            assert_eq!(board.get_possible_moves(i), &[]);
        }
        for i in 24..32 {
            assert_eq!(board.get_possible_moves(i), &[]);
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
            let moves = board.get_possible_moves(start);
            assert_eq!(moves.len(), expected_moves.len(), "Mismatch in number of moves for position {}", start);
        
            for &to in expected_moves {
                assert!(moves.contains(&Move::new(start, to)), "Move from {} to {} not found", start, to);
            }
        }
    }

    fn setup_board_with_one_piece(board: &mut Board, index: usize, state: TileState) {
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[index] = state;
    }
    
    fn assert_moves(board: &Board, index: usize, expected_moves: &[(usize, usize)]) {
        let moves = board.get_possible_moves(index);
        assert_eq!(moves.len(), expected_moves.len(), "Mismatch in number of moves for {:?} at index {}", board.bc.tiles[index], index);
    
        for &(from, to) in expected_moves {
            assert!(moves.contains(&Move::new(from, to)), "Move from {} to {} not found", from, to);
        }
    }

    #[test]
    fn test_get_possible_moves_bm() {
        let mut board = Board::new();
 
        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_moves(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[4] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 4)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[4] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_moves(&board, index, &[(index, 7)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[7] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 20;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_moves(&board, index, &[(index, 24)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 23;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_moves(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[27] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_moves(&board, index, &[]);

    }

    #[test]
    fn test_get_possible_moves_bk() {
        let mut board = Board::new();

        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_moves(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[4] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 4)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[4] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_moves(&board, index, &[(index, 7)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[7] = TileState::BlackMan;
        assert_moves(&board, index, &[]);        


        let index = 4;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_moves(&board, index, &[(index, 0), (index, 8)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[0] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 8)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[8] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 0)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[0] = TileState::BlackMan;
        board.bc.tiles[8] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 7;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_moves(&board, index, &[(index, 2), (index, 3), (index, 10), (index, 11)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 3), (index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[3] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 2), (index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[3] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 3), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 3), (index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 2), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 2), (index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[10] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 2), (index, 3)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 3)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 2)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_moves(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_moves(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[27] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_moves(&board, index, &[]);

    }
 
    #[test]
    fn test_get_possible_moves_rm() {
        let mut board = Board::new();
 
        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_moves(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_moves(&board, index, &[(index, 26), (index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[27] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 24;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_moves(&board, index, &[(index, 20), (index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[20] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[21] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 20)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 27;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_moves(&board, index, &[(index, 23)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[23] = TileState::BlackMan;
        assert_moves(&board, index, &[]);
        
    }


    #[test]
    fn test_get_possible_moves_rk() {
        let mut board = Board::new();

        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_moves(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[27] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_moves(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 27;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_moves(&board, index, &[(index, 23), (index, 31)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[23] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 31)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[31] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 23)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[23] = TileState::BlackMan;
        board.bc.tiles[31] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 24;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_moves(&board, index, &[(index, 20), (index, 21), (index, 28), (index, 29)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 21), (index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[21] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 20), (index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 21), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 21), (index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 20), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 20), (index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[28] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 20), (index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 20)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_moves(&board, index, &[(index, 7)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[7] = TileState::BlackMan;
        assert_moves(&board, index, &[]);


        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_moves(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[4] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_moves(&board, index, &[(index, 4)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[4] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::BlackMan;
        assert_moves(&board, index, &[]);

    }

    fn assert_jumps(board: &Board, index: usize, expected_jumps: &[(usize, Vec<usize>)]) {
        let jumps = board.get_possible_jumps(index);
        assert_eq!(jumps.len(), expected_jumps.len(), "Mismatch in number of jumps for {:?} at index {}", board.bc.tiles[index], index);
    
        for (from, to) in expected_jumps {
            assert!(jumps.contains(&Jump::new(*from, to)), "Move from {} to {:?} not found", from, to);
        }
    }

    #[test]
    fn test_get_possible_jumps_bm() {
        let mut board = Board::new();

        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::BlackKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![9])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedKnight;
        assert_jumps(&board, index, &[(index, vec![9])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[9] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::BlackKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedKnight;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[6] = TileState::RedKnight;
        assert_jumps(&board, index, &[(index, vec![8]), (index, vec![10])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[6] = TileState::RedKnight;
        board.bc.tiles[8] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![10])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[6] = TileState::RedKnight;
        board.bc.tiles[10] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[6] = TileState::RedKnight;
        board.bc.tiles[8] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        


        let index = 4;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[8] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[8] = TileState::BlackKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[8] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[8] = TileState::RedKnight;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[8] = TileState::RedMan;
        board.bc.tiles[13] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        


        let index = 25;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[29] = TileState::RedMan;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[30] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[29] = TileState::RedMan;
        board.bc.tiles[30] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);        

    }
     

    // more tests
}