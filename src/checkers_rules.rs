use crate::checkers_board::*;
use crate::movements::*;
use crate::player_trait::PlayerColor;

pub struct CheckersRules {
}

impl CheckersRules {


    pub fn is_jump_valid(board: &CheckersBoard, jump: &Jump) -> bool {
        let cur_jump = Jump::new((*jump).from(), &vec![(*jump).to[0]]);
        let possible_jumps = CheckersRules::get_possible_jumps(board, (*jump).from());
        if possible_jumps.contains(&cur_jump) {
            if (*jump).to.len() == 1 {
                return true
            }
            let next_jump = Jump::new((*jump).to[0], &(*jump).to[1..].to_vec());
            let mut next_bc = (*board).clone();
            // Do the jump in the temp board
            next_bc.tiles[next_jump.from()] = next_bc.tiles[(*jump).from()];
            next_bc.tiles[(*jump).from()] = TileState::Empty;
            next_bc.tiles[CheckersRules::get_eaten_tile_index((*jump).from(), (*jump).to[0])] = TileState::Empty;
            return CheckersRules::is_jump_valid(&next_bc, &next_jump)
        }
        else {
            return false
        }
    }

    // This methos assumes that the jump is valid
    // Otherwise it will panic
    pub fn get_eaten_tile_index(src: usize, dst: usize) -> usize {
        let delta = ( dst as i32 ) -  ( src as i32 );
        match delta {
            7 => {
                if src % 8 < 4 {
                    return dst - 3
                }
                else {
                    return dst - 4
                }
            }
            9 => {
                if src % 8 < 4 {
                    return dst - 4
                }
                else {
                    return dst - 5
                }
            }
            -7 => {
                if src % 8 < 4 {
                    return src - 3
                }
                else {
                    return src - 4
                }
            }
            -9 => {
                if src % 8 < 4 {
                    return src - 4
                }
                else {
                    return src - 5
                }
            }
            _ => {
                panic!("Invalid jump");
            }

        }
    }


    pub fn get_possible_shifts(board: &CheckersBoard, index: usize) -> Vec<Shift> {
        if index > 31 {
            panic!("CheckersRules::get_possible_shifts: Index out of bounds");
        }
        match board.tiles[index] {
            TileState::BlackMan => {
                return CheckersRules::get_possible_shifts_for_piece(board, index, TileState::BlackMan);
            }
            TileState::BlackKnight | TileState::RedKnight => {
                let mut shifts: Vec<Shift>  = vec![];
                let mut shifts2 = CheckersRules::get_possible_shifts_for_piece(board, index, TileState::BlackMan);
                shifts.append(&mut shifts2);
                let mut shifts3 = CheckersRules::get_possible_shifts_for_piece(board, index, TileState::RedMan);
                shifts.append(&mut shifts3);
                return shifts;
            }
            TileState::RedMan => {
                return CheckersRules::get_possible_shifts_for_piece(board, index, TileState::RedMan);
            }
            _ => {
                return vec![];
            }
        }
    }

    fn get_possible_shifts_for_piece(board: &CheckersBoard, index: usize, state: TileState) -> Vec<Shift> {
        match state {
            TileState::BlackMan => {
                if index > 27 {
                    return vec![];
                }
                if ((index % 8) == 3) || ((index % 8) == 4) {
                    if board.tiles[index+4] == TileState::Empty {
                        return vec![Shift::new(index, index+4)];
                    }
                    return vec![];
                }
                else {
                    let mut shifts = vec![];
                    if (index % 8) > 3 {
                        if board.tiles[index+3] == TileState::Empty {
                            shifts.push(Shift::new(index, index+3));
                        }
                    }
                    if board.tiles[index+4] == TileState::Empty {
                        shifts.push(Shift::new(index, index+4));
                    }
                    if (index % 8) < 4 {
                        if board.tiles[index+5] == TileState::Empty {
                            shifts.push(Shift::new(index, index+5));
                        }
                    }
                    return shifts;
                }                        
            }
            TileState::RedMan => {
                if index < 4 {
                    return vec![];
                }
                if ((index % 8) == 3) || ((index % 8) == 4) {
                    if board.tiles[index-4] == TileState::Empty {
                        return vec![Shift::new(index, index-4)];
                    }
                    return vec![];
                }
                else {
                    let mut shifts = vec![];

                    if (index % 8) > 3 {
                        if board.tiles[index-5] == TileState::Empty {
                            shifts.push(Shift::new(index, index-5));
                        }
                    }
                    if board.tiles[index-4] == TileState::Empty {
                        shifts.push(Shift::new(index, index-4));
                    }
                    if (index % 8) < 4 {
                        if board.tiles[index-3] == TileState::Empty {
                            shifts.push(Shift::new(index, index-3));
                        }
                    }
                    return shifts;
                }                        
            }
            _ => {
                panic!("CheckersBoard::get_possible_shifts_for_piece: Invalid piece state!");
            }
        }
    }

    pub fn get_possible_jumps(board: &CheckersBoard, index: usize) -> Vec<Jump> {
        if index > 31 {
            panic!("Board::get_possible_jumps: Index out of bounds");
        }
        let mut tiles_to_check: Vec<TileState> = vec![TileState::RedMan, TileState::RedKnight];
        let mut jumps = vec![];
        match board.tiles[index] {
            TileState::BlackMan => {
                CheckersRules::get_possible_jump_bl(board, index, &tiles_to_check, &mut jumps);
                CheckersRules::get_possible_jump_br(board, index, &tiles_to_check, &mut jumps);
            }
            TileState::BlackKnight => {
                CheckersRules::get_possible_jump_bl(board, index, &tiles_to_check, &mut jumps);
                CheckersRules::get_possible_jump_br(board, index, &tiles_to_check, &mut jumps);
                CheckersRules::get_possible_jump_tl(board, index, &tiles_to_check, &mut jumps);
                CheckersRules::get_possible_jump_tr(board, index, &tiles_to_check, &mut jumps);
            }
            TileState::RedMan => {
                tiles_to_check[0] = TileState::BlackMan;
                tiles_to_check[1] = TileState::BlackKnight;
                CheckersRules::get_possible_jump_tl(board, index, &tiles_to_check, &mut jumps);
                CheckersRules::get_possible_jump_tr(board, index, &tiles_to_check, &mut jumps);
            }
            TileState::RedKnight => {
                tiles_to_check[0] = TileState::BlackMan;
                tiles_to_check[1] = TileState::BlackKnight;
                CheckersRules::get_possible_jump_bl(board, index, &tiles_to_check, &mut jumps);
                CheckersRules::get_possible_jump_br(board, index, &tiles_to_check, &mut jumps);
                CheckersRules::get_possible_jump_tl(board, index, &tiles_to_check, &mut jumps);
                CheckersRules::get_possible_jump_tr(board, index, &tiles_to_check, &mut jumps);
            }
            _ => {}
        }
        return jumps;
        
    }

    fn get_possible_jump_bl(board: &CheckersBoard, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index > 23 {
            return
        }
        if (index % 4) > 0 {
            if board.tiles[index +7] == TileState::Empty {
                if (index %8) < 4 {
                    if (board.tiles[index+4] == tiles_to_check[0]) ||
                        (board.tiles[index+4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+7]));
                    }                            
                }
                else {
                    if (board.tiles[index+3] == tiles_to_check[0]) ||
                        (board.tiles[index+3] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+7]));
                    }                            
                }
            }
        }        
    }

    fn get_possible_jump_br(board: &CheckersBoard, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index > 23 {
            return
        }
        if (index % 4) < 3 {
            if board.tiles[index +9] == TileState::Empty {
                if (index %8) < 4 {
                    if (board.tiles[index+5] == tiles_to_check[0]) ||
                        (board.tiles[index+5] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+9]));
                    }                            
                }
                else {
                    if (board.tiles[index+4] == tiles_to_check[0]) ||
                        (board.tiles[index+4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+9]));
                    }                            
                }
            }
        }        
    }

    fn get_possible_jump_tl(board: &CheckersBoard, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index < 8 {
            return;
        }
        if (index % 4) > 0 {
            if board.tiles[index -9] == TileState::Empty {
                if (index %8) < 4 {
                    if (board.tiles[index-4] == tiles_to_check[0]) ||
                        (board.tiles[index-4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -9]));
                    }                            
                }
                else {
                    if (board.tiles[index-5] == tiles_to_check[0]) ||
                        (board.tiles[index-5] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -9]));
                    }                            
                }
            }
        }    }    

    fn get_possible_jump_tr(board: &CheckersBoard, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index < 8 {
            return;
        }
        if (index % 4) < 3 {
            if board.tiles[index -7] == TileState::Empty {
                if (index %8) < 4 {
                    if (board.tiles[index-3] == tiles_to_check[0]) ||
                        (board.tiles[index-3] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -7]));
                    }                            
                }
                else {
                    if (board.tiles[index-4] == tiles_to_check[0]) ||
                        (board.tiles[index-4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -7]));
                    }                            
                }
            }
        } 
    }       

    pub fn get_player_pieces_indexes(board: &CheckersBoard, player_color: PlayerColor) -> Vec<usize> {
        let mut player_pieces_indexes = Vec::new();
        for (i, tile) in board.tiles.iter().enumerate() {
            if (player_color == PlayerColor::Black && (*tile == TileState::BlackMan || *tile == TileState::BlackKnight)) || 
               (player_color == PlayerColor::Red && (*tile == TileState::RedMan || *tile == TileState::RedKnight)) {
                player_pieces_indexes.push(i);
            }
        }
        return player_pieces_indexes
    }    
}


////////////////////////////////////////////////////////////////////////////////
/// Unit tests
/// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_possible_shifts() {
        let board = CheckersBoard::new();

        // Test #1: default board
        for i in 0..8 {
            assert_eq!(CheckersRules::get_possible_shifts(&board, i), &[]);
        }
        for i in 12..20 {
            assert_eq!(CheckersRules::get_possible_shifts(&board, i), &[]);
        }
        for i in 24..32 {
            assert_eq!(CheckersRules::get_possible_shifts(&board, i), &[]);
        }
        let test_cases = [
            (8, vec![12, 13]),
            (9, vec![13, 14]),
            (10, vec![14, 15]),
            (11, vec![15]), // Note that (11, vec![15]) has only one shift.
            (20, vec![16]), // Note that (20, vec![16]) has only one shift.
            (21, vec![16, 17]),
            (22, vec![17, 18]),
            (23, vec![18, 19]),
        ];        
        for &(start, ref expected_shifts) in &test_cases {
            let shifts = CheckersRules::get_possible_shifts(&board, start);
            assert_eq!(shifts.len(), expected_shifts.len(), "Mismatch in number of shifts for position {}", start);
        
            for &to in expected_shifts {
                assert!(shifts.contains(&Shift::new(start, to)), "Shift from {} to {} not found", start, to);
            }
        }
    }

    fn setup_board_with_one_piece(board: &mut CheckersBoard, index: usize, state: TileState) {
        board.tiles.fill(TileState::Empty);
        board.tiles[index] = state;
    }
    
    fn assert_shifts(board: &CheckersBoard, index: usize, expected_shifts: &[(usize, usize)]) {
        let shifts = CheckersRules::get_possible_shifts(board, index);
        assert_eq!(shifts.len(), expected_shifts.len(), "Mismatch in number of shifts for {:?} at index {}", board.tiles[index], index);
    
        for &(from, to) in expected_shifts {
            assert!(shifts.contains(&Shift::new(from, to)), "Shift from {} to {} not found", from, to);
        }
    }

    #[test]
    fn test_get_possible_shifts_bm() {
        let mut board = CheckersBoard::new();
 
        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_shifts(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[4] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 4)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[4] = TileState::BlackMan;
        board.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_shifts(&board, index, &[(index, 7)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[7] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 20;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_shifts(&board, index, &[(index, 24)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[24] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 23;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_shifts(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[26] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[26] = TileState::BlackMan;
        board.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);

    }

    #[test]
    fn test_get_possible_shifts_bk() {
        let mut board = CheckersBoard::new();

        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[4] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 4)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[4] = TileState::BlackMan;
        board.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 7)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[7] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);        


        let index = 4;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 0), (index, 8)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[0] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 8)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[8] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 0)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[0] = TileState::BlackMan;
        board.tiles[8] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 7;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 2), (index, 3), (index, 10), (index, 11)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[2] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 3), (index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[3] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2), (index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[2] = TileState::BlackMan;
        board.tiles[3] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[2] = TileState::BlackMan;
        board.tiles[10] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 3), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[2] = TileState::BlackMan;
        board.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 3), (index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[3] = TileState::BlackMan;
        board.tiles[10] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[3] = TileState::BlackMan;
        board.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2), (index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[10] = TileState::BlackMan;
        board.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2), (index, 3)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[2] = TileState::BlackMan;
        board.tiles[3] = TileState::BlackMan;
        board.tiles[10] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[2] = TileState::BlackMan;
        board.tiles[3] = TileState::BlackMan;
        board.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[2] = TileState::BlackMan;
        board.tiles[10] = TileState::BlackMan;
        board.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 3)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[3] = TileState::BlackMan;
        board.tiles[10] = TileState::BlackMan;
        board.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[2] = TileState::BlackMan;
        board.tiles[3] = TileState::BlackMan;
        board.tiles[10] = TileState::BlackMan;
        board.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[24] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[26] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.tiles[26] = TileState::BlackMan;
        board.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);

    }
 
    #[test]
    fn test_get_possible_shifts_rm() {
        let mut board = CheckersBoard::new();
 
        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_shifts(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[24] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_shifts(&board, index, &[(index, 26), (index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackMan;
        board.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 24;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_shifts(&board, index, &[(index, 20), (index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[20] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[21] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[20] = TileState::BlackMan;
        board.tiles[21] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 27;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_shifts(&board, index, &[(index, 23)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[23] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);
        
    }


    #[test]
    fn test_get_possible_shifts_rk() {
        let mut board = CheckersBoard::new();

        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[26] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[26] = TileState::BlackMan;
        board.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[24] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 27;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 23), (index, 31)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[23] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 31)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[31] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 23)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[23] = TileState::BlackMan;
        board.tiles[31] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 24;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 20), (index, 21), (index, 28), (index, 29)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[20] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21), (index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[21] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20), (index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[20] = TileState::BlackMan;
        board.tiles[21] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[20] = TileState::BlackMan;
        board.tiles[28] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[20] = TileState::BlackMan;
        board.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21), (index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[21] = TileState::BlackMan;
        board.tiles[28] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[21] = TileState::BlackMan;
        board.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20), (index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[28] = TileState::BlackMan;
        board.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20), (index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[20] = TileState::BlackMan;
        board.tiles[21] = TileState::BlackMan;
        board.tiles[28] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[20] = TileState::BlackMan;
        board.tiles[21] = TileState::BlackMan;
        board.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[20] = TileState::BlackMan;
        board.tiles[28] = TileState::BlackMan;
        board.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[21] = TileState::BlackMan;
        board.tiles[28] = TileState::BlackMan;
        board.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[20] = TileState::BlackMan;
        board.tiles[21] = TileState::BlackMan;
        board.tiles[28] = TileState::BlackMan;
        board.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 7)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[7] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[4] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 4)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.tiles[4] = TileState::BlackMan;
        board.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);

    }

    fn assert_jumps(board: &CheckersBoard, index: usize, expected_jumps: &[(usize, Vec<usize>)]) {
        let jumps = CheckersRules::get_possible_jumps(&board, index);
        assert_eq!(jumps.len(), expected_jumps.len(), "Mismatch in number of jumps for {:?} at index {}", board.tiles[index], index);
    
        for (from, to) in expected_jumps {
            assert!(jumps.contains(&Jump::new(*from, to)), "Jump from {} to {:?} not found", from, to);
        }
    }

    #[test]
    fn test_get_possible_jumps_bm() {
        let mut board = CheckersBoard::new();

        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::BlackKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![9])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedKnight;
        assert_jumps(&board, index, &[(index, vec![9])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedMan;
        board.tiles[9] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);


        let index = 1;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::BlackKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedKnight;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedMan;
        board.tiles[6] = TileState::RedKnight;
        assert_jumps(&board, index, &[(index, vec![8]), (index, vec![10])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedMan;
        board.tiles[6] = TileState::RedKnight;
        board.tiles[8] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![10])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedMan;
        board.tiles[6] = TileState::RedKnight;
        board.tiles[10] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[5] = TileState::RedMan;
        board.tiles[6] = TileState::RedKnight;
        board.tiles[8] = TileState::BlackMan;
        board.tiles[10] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        


        let index = 4;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[8] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[8] = TileState::BlackKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[8] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[8] = TileState::RedKnight;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[8] = TileState::RedMan;
        board.tiles[13] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        


        let index = 25;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[29] = TileState::RedMan;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[30] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.tiles[29] = TileState::RedMan;
        board.tiles[30] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);        

    }

    #[test]
    fn test_get_possible_jumps_rm() {
        let mut board = CheckersBoard::new();

        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::RedMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![22])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackKnight;
        assert_jumps(&board, index, &[(index, vec![22])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::RedMan;
        board.tiles[27] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);


        let index = 30;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::RedMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![23])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackKnight;
        assert_jumps(&board, index, &[(index, vec![23])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackMan;
        board.tiles[25] = TileState::BlackKnight;
        assert_jumps(&board, index, &[(index, vec![23]), (index, vec![21])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackMan;
        board.tiles[25] = TileState::BlackKnight;
        board.tiles[23] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![21])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackMan;
        board.tiles[25] = TileState::BlackKnight;
        board.tiles[21] = TileState::RedMan;
        assert_jumps(&board, index, &[(index, vec![23])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[26] = TileState::BlackMan;
        board.tiles[25] = TileState::BlackKnight;
        board.tiles[23] = TileState::RedMan;
        board.tiles[21] = TileState::RedMan;
        assert_jumps(&board, index, &[]);        


        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[24] = TileState::RedMan;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[24] = TileState::RedKnight;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[24] = TileState::BlackMan;
        assert_jumps(&board, index, &[(index, vec![21])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[24] = TileState::BlackKnight;
        assert_jumps(&board, index, &[(index, vec![21])]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[24] = TileState::BlackMan;
        board.tiles[28] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        


        let index = 1;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[5] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[6] = TileState::BlackMan;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.tiles[5] = TileState::BlackMan;
        board.tiles[6] = TileState::BlackMan;
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
        let mut board = CheckersBoard::new();

        let index = 0;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        assert_jumps(&board, index, &[(index, vec![9])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile3;
        assert_jumps(&board, index, &[(index, vec![9])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[9] = tile1;
        assert_jumps(&board, index, &[]);


        let index = 1;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile3;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[6] = tile3;
        assert_jumps(&board, index, &[(index, vec![8]), (index, vec![10])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[6] = tile3;
        board.tiles[8] = tile1;
        assert_jumps(&board, index, &[(index, vec![10])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[6] = tile3;
        board.tiles[10] = tile1;
        assert_jumps(&board, index, &[(index, vec![8])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[6] = tile3;
        board.tiles[8] = tile1;
        board.tiles[10] = tile1;
        assert_jumps(&board, index, &[]);        


        let index = 4;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile2;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile3;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile2;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile3;
        assert_jumps(&board, index, &[(index, vec![13])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile2;
        board.tiles[13] = tile1;
        assert_jumps(&board, index, &[]);        


        let index = 5;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile2;
        assert_jumps(&board, index, &[(index, vec![12])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile3;
        assert_jumps(&board, index, &[(index, vec![12])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile2;
        board.tiles[9] = tile3;
        assert_jumps(&board, index, &[(index, vec![12]), (index, vec![14])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile2;
        board.tiles[9] = tile3;
        board.tiles[12] = tile1;
        assert_jumps(&board, index, &[(index, vec![14])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile2;
        board.tiles[9] = tile3;
        board.tiles[14] = tile1;
        assert_jumps(&board, index, &[(index, vec![12])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[8] = tile2;
        board.tiles[9] = tile3;
        board.tiles[12] = tile1;
        board.tiles[14] = tile1;
        assert_jumps(&board, index, &[]); 


        let index = 8;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        assert_jumps(&board, index, &[(index, vec![1])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile3;
        assert_jumps(&board, index, &[(index, vec![1])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[13] = tile3;
        assert_jumps(&board, index, &[(index, vec![1]), (index, vec![17])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[13] = tile3;
        board.tiles[1] = tile1;
        assert_jumps(&board, index, &[(index, vec![17])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[13] = tile3;
        board.tiles[17] = tile1;
        assert_jumps(&board, index, &[(index, vec![1])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[5] = tile2;
        board.tiles[13] = tile3;
        board.tiles[1] = tile1;
        board.tiles[17] = tile1;
        assert_jumps(&board, index, &[]);

        let index = 25;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[21] = tile1;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[21] = src_state;
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[21] = tile2;
        assert_jumps(&board, index, &[(index, vec![16])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[21] = tile3;
        assert_jumps(&board, index, &[(index, vec![16])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[21] = tile2;
        board.tiles[22] = tile3;
        assert_jumps(&board, index, &[(index, vec![16]), (index, vec![18])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[21] = tile2;
        board.tiles[22] = tile3;
        board.tiles[16] = tile1;
        assert_jumps(&board, index, &[(index, vec![18])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[21] = tile2;
        board.tiles[22] = tile3;
        board.tiles[18] = tile1;
        assert_jumps(&board, index, &[(index, vec![16])]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[21] = tile2;
        board.tiles[22] = tile3;
        board.tiles[16] = tile1;
        board.tiles[18] = tile1;
        assert_jumps(&board, index, &[]);             


        let index = 25;
        setup_board_with_one_piece(&mut board, index, src_state);
        assert_jumps(&board, index, &[]);

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[29] = tile2;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[30] = tile3;
        assert_jumps(&board, index, &[]);        

        setup_board_with_one_piece(&mut board, index, src_state);
        board.tiles[29] = tile2;
        board.tiles[30] = tile3;
        assert_jumps(&board, index, &[]);              
    }

    #[test]
    fn test_get_possible_jumps_bk() {
        run_jump_knight_test(TileState::BlackKnight);
        run_jump_knight_test(TileState::RedKnight);
    }


    // more tests
}