use std::rc::Rc;
use std::cell::RefCell;
use crate::board_content::*;
use crate::move_piece::*;
use crate::player_trait::PlayerColor;
use crate::game_actions::ActionMove;


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

    pub fn move_piece(&mut self, action: &ActionMove) -> Result<(), String> {
        // Verify if the action has enough tiles
        if (*action).tiles.len() < 2 {
            return Err("The action does not have at least two tiles (soruce and destination)".into());
        }

        // Verify if the source tile of the action is the right color
        let src = (*action).tiles[0];
        if action.player_color == PlayerColor::Black {
            if (self.bc.tiles[src] != TileState::BlackMan) && (self.bc.tiles[src] != TileState::BlackKnight) {
                return Err("Player is not moving a black piece.".into());
            }
        }
        else {
            if (self.bc.tiles[src] != TileState::RedMan) && (self.bc.tiles[src] != TileState::RedKnight) {
                return Err("Player is not moving a red piece.".into());
            }
        }
        return Ok(());
        /*
        // Transform User Action to either a Move or a Jump
        let dst = (*action).tiles[1];
        let jump = if (*action).tiles.len() > 2 { Some((*action).tiles[2]) } else { None };
        let move_piece = MovePiece::new(src, dst, jump);
        */
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
        let mut tiles_to_check: Vec<TileState> = vec![TileState::RedMan, TileState::RedKnight];
        let mut jumps = vec![];
        match self.bc.tiles[index] {
            TileState::BlackMan => {
                self.get_possible_jump_bl( index, &tiles_to_check, &mut jumps);
                self.get_possible_jump_br( index, &tiles_to_check, &mut jumps);
            }
            TileState::BlackKnight => {
                self.get_possible_jump_bl( index, &tiles_to_check, &mut jumps);
                self.get_possible_jump_br( index, &tiles_to_check, &mut jumps);
                self.get_possible_jump_tl( index, &tiles_to_check, &mut jumps);
                self.get_possible_jump_tr( index, &tiles_to_check, &mut jumps);
            }
            TileState::RedMan => {
                tiles_to_check[0] = TileState::BlackMan;
                tiles_to_check[1] = TileState::BlackKnight;
                self.get_possible_jump_tl( index, &tiles_to_check, &mut jumps);
                self.get_possible_jump_tr( index, &tiles_to_check, &mut jumps);
            }
            TileState::RedKnight => {
                tiles_to_check[0] = TileState::BlackMan;
                tiles_to_check[1] = TileState::BlackKnight;
                self.get_possible_jump_bl( index, &tiles_to_check, &mut jumps);
                self.get_possible_jump_br( index, &tiles_to_check, &mut jumps);
                self.get_possible_jump_tl( index, &tiles_to_check, &mut jumps);
                self.get_possible_jump_tr( index, &tiles_to_check, &mut jumps);
            }
            _ => {}
        }
        return jumps;
        
    }

    fn get_possible_jump_bl(&self, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index > 23 {
            return
        }
        if (index % 4) > 0 {
            if self.bc.tiles[index +7] == TileState::Empty {
                if (index %8) < 4 {
                    if (self.bc.tiles[index+4] == tiles_to_check[0]) ||
                        (self.bc.tiles[index+4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+7]));
                    }                            
                }
                else {
                    if (self.bc.tiles[index+3] == tiles_to_check[0]) ||
                        (self.bc.tiles[index+3] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+7]));
                    }                            
                }
            }
        }        
    }

    fn get_possible_jump_br(&self, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index > 23 {
            return
        }
        if (index % 4) < 3 {
            if self.bc.tiles[index +9] == TileState::Empty {
                if (index %8) < 4 {
                    if (self.bc.tiles[index+5] == tiles_to_check[0]) ||
                        (self.bc.tiles[index+5] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+9]));
                    }                            
                }
                else {
                    if (self.bc.tiles[index+4] == tiles_to_check[0]) ||
                        (self.bc.tiles[index+4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+9]));
                    }                            
                }
            }
        }        
    }

    fn get_possible_jump_tl(&self, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index < 8 {
            return;
        }
        if (index % 4) > 0 {
            if self.bc.tiles[index -9] == TileState::Empty {
                if (index %8) < 4 {
                    if (self.bc.tiles[index-4] == tiles_to_check[0]) ||
                        (self.bc.tiles[index-4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -9]));
                    }                            
                }
                else {
                    if (self.bc.tiles[index-5] == tiles_to_check[0]) ||
                        (self.bc.tiles[index-5] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -9]));
                    }                            
                }
            }
        }    }    

    fn get_possible_jump_tr(&self, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index < 8 {
            return;
        }
        if (index % 4) < 3 {
            if self.bc.tiles[index -7] == TileState::Empty {
                if (index %8) < 4 {
                    if (self.bc.tiles[index-3] == tiles_to_check[0]) ||
                        (self.bc.tiles[index-3] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -7]));
                    }                            
                }
                else {
                    if (self.bc.tiles[index-4] == tiles_to_check[0]) ||
                        (self.bc.tiles[index-4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -7]));
                    }                            
                }
            }
        } 
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


        let index = 1;
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

    #[test]
    fn test_get_possible_jumps_rm() {
        let mut board = Board::new();

        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::RedMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![22])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackKnight;
        assert_jumps(&board, index, &[(index, vec![22])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::RedMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);


        let index = 30;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::RedMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![23])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackKnight;
        assert_jumps(&board, index, &[(index, vec![23])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[25] = TileState::BlackKnight;
        assert_jumps(&board, index, &[(index, vec![23]), (index, vec![21])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[25] = TileState::BlackKnight;
        board.bc.tiles[23] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![21])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[25] = TileState::BlackKnight;
        board.bc.tiles[21] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![23])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[25] = TileState::BlackKnight;
        board.bc.tiles[23] = TileState::RedMan;
        board.bc.tiles[21] = TileState::RedMan;
        assert_jumps(&board, index, &[]);        


        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[24] = TileState::RedMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[24] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![21])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[24] = TileState::BlackKnight;
        assert_jumps(&board, index, &[(index, vec![21])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[24] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        


        let index = 1;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[6] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[5] = TileState::BlackMan;
        board.bc.tiles[6] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        

    }    
     

    fn run_jump_knight_test(src_state: TileState) {
        let mut tile1 = TileState::BlackMan;
        let mut tile2 = TileState::RedMan;
        let mut tile3 = TileState::RedKnight;
        if src_state == TileState::RedKnight {
            tile1 = TileState::RedMan;
            tile2 = TileState::BlackMan;
            tile3 = TileState::BlackKnight;
        }
        let mut board = Board::new();

        let index = 0;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        assert_jumps(&board, index, &[(index, vec![9])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile3;
        assert_jumps(&board, index, &[(index, vec![9])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[9] = tile1;
        assert_jumps(&board, index, &[]);


        let index = 1;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile3;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[6] = tile3;
        assert_jumps(&board, index, &[(index, vec![8]), (index, vec![10])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[6] = tile3;
        board.bc.tiles[8] = tile1;
        assert_jumps(&board, index, &[(index, vec![10])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[6] = tile3;
        board.bc.tiles[10] = tile1;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[6] = tile3;
        board.bc.tiles[8] = tile1;
        board.bc.tiles[10] = tile1;
        assert_jumps(&board, index, &[]);        


        let index = 4;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile2;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile3;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile2;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile3;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile2;
        board.bc.tiles[13] = tile1;
        assert_jumps(&board, index, &[]);        


        let index = 5;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile2;
        assert_jumps(&board, index, &[(index, vec![12])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile3;
        assert_jumps(&board, index, &[(index, vec![12])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile2;
        board.bc.tiles[9] = tile3;
        assert_jumps(&board, index, &[(index, vec![12]), (index, vec![14])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile2;
        board.bc.tiles[9] = tile3;
        board.bc.tiles[12] = tile1;
        assert_jumps(&board, index, &[(index, vec![14])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile2;
        board.bc.tiles[9] = tile3;
        board.bc.tiles[14] = tile1;
        assert_jumps(&board, index, &[(index, vec![12])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[8] = tile2;
        board.bc.tiles[9] = tile3;
        board.bc.tiles[12] = tile1;
        board.bc.tiles[14] = tile1;
        assert_jumps(&board, index, &[]); 


        let index = 8;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        assert_jumps(&board, index, &[(index, vec![1])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile3;
        assert_jumps(&board, index, &[(index, vec![1])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[13] = tile3;
        assert_jumps(&board, index, &[(index, vec![1]), (index, vec![17])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[13] = tile3;
        board.bc.tiles[1] = tile1;
        assert_jumps(&board, index, &[(index, vec![17])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[13] = tile3;
        board.bc.tiles[17] = tile1;
        assert_jumps(&board, index, &[(index, vec![1])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[5] = tile2;
        board.bc.tiles[13] = tile3;
        board.bc.tiles[1] = tile1;
        board.bc.tiles[17] = tile1;
        assert_jumps(&board, index, &[]);

        let index = 25;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[21] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[21] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[21] = tile2;
        assert_jumps(&board, index, &[(index, vec![16])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[21] = tile3;
        assert_jumps(&board, index, &[(index, vec![16])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[21] = tile2;
        board.bc.tiles[22] = tile3;
        assert_jumps(&board, index, &[(index, vec![16]), (index, vec![18])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[21] = tile2;
        board.bc.tiles[22] = tile3;
        board.bc.tiles[16] = tile1;
        assert_jumps(&board, index, &[(index, vec![18])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[21] = tile2;
        board.bc.tiles[22] = tile3;
        board.bc.tiles[18] = tile1;
        assert_jumps(&board, index, &[(index, vec![16])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[21] = tile2;
        board.bc.tiles[22] = tile3;
        board.bc.tiles[16] = tile1;
        board.bc.tiles[18] = tile1;
        assert_jumps(&board, index, &[]);             


        let index = 25;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[29] = tile2;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[30] = tile3;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, src_state);
        board.bc.tiles[29] = tile2;
        board.bc.tiles[30] = tile3;
        assert_jumps(&board, index, &[]);              
    }

    #[test]
    fn test_get_possible_jumps_bk() {
        run_jump_knight_test(TileState::BlackKnight);
        run_jump_knight_test(TileState::RedKnight);
    }     

    // more tests
}