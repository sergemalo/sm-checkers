use std::rc::Rc;
use std::cell::RefCell;
use crate::board_content::*;
use crate::movements::*;
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

        let res = self.is_move_valid(action)?;
        if res.as_any().downcast_ref::<Shift>().is_some() {
            if let Some(shift) = res.as_any().downcast_ref::<Shift>() {
                self.bc.tiles[shift.to] = self.bc.tiles[shift.from()];
                self.bc.tiles[shift.from()] = TileState::Empty;
            }
        }
        if res.as_any().downcast_ref::<Jump>().is_some() {
            if let Some(jump) = res.as_any().downcast_ref::<Jump>() {
                let tile_eaten = Board::get_eaten_tile_index(jump.from(), jump.to[0]);
                self.bc.tiles[tile_eaten] = TileState::Empty;
                self.bc.tiles[*jump.to.last().unwrap()] = self.bc.tiles[jump.from()];
                self.bc.tiles[jump.from()] = TileState::Empty;
                // TODO: For loop
            }
        }

        self.notify_observers();
        return Ok(());
    }

    fn is_move_valid(&mut self, action: &ActionMove) -> Result<Box<dyn Movement>, String> {
        // Verify if the action has enough tiles
        if (*action).tiles.len() < 2 {
            return Err("The action does not have at least two tiles (soruce and destination)".into());
        }

        // Verify if tile indexes are in the correct range
        for t in &(*action).tiles {
            if *t > 31 {
                return Err("Tile index is out of range.".into());
            }
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
        
        // Verify if all destination tiles are empty
        for (_index, &element) in (*action).tiles.iter().enumerate().skip(1) {
            if self.bc.tiles[element] != TileState::Empty {
                return Err("Destination tile is not empty.".into());
            }
        }

        // Verify if this is a shift, and if it is a valid shift
        let src = (*action).tiles[0];
        let dst = (*action).tiles[1];
        //let mut is_shift = false;
        if ((*action).tiles.len() == 2) &&
            (((dst > src) && (dst - src) <5) ||
             ((dst < src) && (src - dst) < 6)) {
            let cur_shift = Shift::new(src, dst);
            let possible_shifts = self.get_possible_shifts(src);
            if !possible_shifts.contains(&cur_shift) {
                return Err("Invalid shift.".into());
            }
            return Ok(Box::new(cur_shift));
        }

        // This is a jump. Verify if it is a valid jump
        let the_jump = Jump::new(src, &((*action).tiles[1..]).to_vec());
        if Board::is_jump_valid(&self.bc, &the_jump) {
            return Ok(Box::new(the_jump));            
        }
        return Err("Invalid action.".into());
    }

    fn is_jump_valid(bc: &BoardContent, jump: &Jump) -> bool {
        let cur_jump = Jump::new((*jump).from(), &vec![(*jump).to[0]]);
        let possible_jumps = Board::get_possible_jumps(bc, (*jump).from());
        if possible_jumps.contains(&cur_jump) {
            if (*jump).to.len() == 1 {
                return true
            }
            let next_jump = Jump::new((*jump).to[0], &(*jump).to[1..].to_vec());
            let mut next_bc = (*bc).clone();
            // Do the jump in the temp board
            next_bc.tiles[next_jump.from()] = next_bc.tiles[(*jump).from()];
            next_bc.tiles[(*jump).from()] = TileState::Empty;
            next_bc.tiles[Board::get_eaten_tile_index((*jump).from(), (*jump).to[0])] = TileState::Empty;
            return Board::is_jump_valid(&next_bc, &next_jump)
        }
        else {
            return false
        }
    }

    // This methos assumes that the jump is valid
    // Otherwise it will panic
    fn get_eaten_tile_index(src: usize, dst: usize) -> usize {
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

    pub fn is_game_over(&self, next_player_color: PlayerColor) -> bool {
        let pieces = self.get_player_pieces_indexes(next_player_color);
        for (i, _p) in pieces.iter().enumerate(){
            if self.get_possible_shifts(i).len() > 0 {
                return false
            }
        }
        for (i, _p) in pieces.iter().enumerate(){
            if Board::get_possible_jumps(&self.bc, i).len() > 0 {
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

    pub fn get_possible_shifts(&self, index: usize) -> Vec<Shift> {
        if index > 31 {
            panic!("Board::get_possible_shifts: Index out of bounds");
        }
        match self.bc.tiles[index] {
            TileState::BlackMan => {
                return self.get_possible_shifts_for_piece(index, TileState::BlackMan);
            }
            TileState::BlackKnight | TileState::RedKnight => {
                let mut shifts: Vec<Shift>  = vec![];
                let mut shifts2 = self.get_possible_shifts_for_piece(index, TileState::BlackMan);
                shifts.append(&mut shifts2);
                let mut shifts3 = self.get_possible_shifts_for_piece(index, TileState::RedMan);
                shifts.append(&mut shifts3);
                return shifts;
            }
            TileState::RedMan => {
                return self.get_possible_shifts_for_piece(index, TileState::RedMan);
            }
            _ => {
                return vec![];
            }
        }
    }

    fn get_possible_shifts_for_piece(&self, index: usize, state: TileState) -> Vec<Shift> {
        match state {
            TileState::BlackMan => {
                if index > 27 {
                    return vec![];
                }
                if ((index % 8) == 3) || ((index % 8) == 4) {
                    if self.bc.tiles[index+4] == TileState::Empty {
                        return vec![Shift::new(index, index+4)];
                    }
                    return vec![];
                }
                else {
                    let mut shifts = vec![];
                    if (index % 8) > 3 {
                        if self.bc.tiles[index+3] == TileState::Empty {
                            shifts.push(Shift::new(index, index+3));
                        }
                    }
                    if self.bc.tiles[index+4] == TileState::Empty {
                        shifts.push(Shift::new(index, index+4));
                    }
                    if (index % 8) < 4 {
                        if self.bc.tiles[index+5] == TileState::Empty {
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
                    if self.bc.tiles[index-4] == TileState::Empty {
                        return vec![Shift::new(index, index-4)];
                    }
                    return vec![];
                }
                else {
                    let mut shifts = vec![];

                    if (index % 8) > 3 {
                        if self.bc.tiles[index-5] == TileState::Empty {
                            shifts.push(Shift::new(index, index-5));
                        }
                    }
                    if self.bc.tiles[index-4] == TileState::Empty {
                        shifts.push(Shift::new(index, index-4));
                    }
                    if (index % 8) < 4 {
                        if self.bc.tiles[index-3] == TileState::Empty {
                            shifts.push(Shift::new(index, index-3));
                        }
                    }
                    return shifts;
                }                        
            }
            _ => {
                panic!("Board::get_possible_shifts_for_piece: Invalid piece state!");
            }
        }
    }

    pub fn get_possible_jumps(bc: &BoardContent, index: usize) -> Vec<Jump> {
        if index > 31 {
            panic!("Board::get_possible_jumps: Index out of bounds");
        }
        let mut tiles_to_check: Vec<TileState> = vec![TileState::RedMan, TileState::RedKnight];
        let mut jumps = vec![];
        match bc.tiles[index] {
            TileState::BlackMan => {
                Board::get_possible_jump_bl(bc, index, &tiles_to_check, &mut jumps);
                Board::get_possible_jump_br(bc, index, &tiles_to_check, &mut jumps);
            }
            TileState::BlackKnight => {
                Board::get_possible_jump_bl(bc, index, &tiles_to_check, &mut jumps);
                Board::get_possible_jump_br(bc, index, &tiles_to_check, &mut jumps);
                Board::get_possible_jump_tl(bc, index, &tiles_to_check, &mut jumps);
                Board::get_possible_jump_tr(bc, index, &tiles_to_check, &mut jumps);
            }
            TileState::RedMan => {
                tiles_to_check[0] = TileState::BlackMan;
                tiles_to_check[1] = TileState::BlackKnight;
                Board::get_possible_jump_tl(bc, index, &tiles_to_check, &mut jumps);
                Board::get_possible_jump_tr(bc, index, &tiles_to_check, &mut jumps);
            }
            TileState::RedKnight => {
                tiles_to_check[0] = TileState::BlackMan;
                tiles_to_check[1] = TileState::BlackKnight;
                Board::get_possible_jump_bl(bc, index, &tiles_to_check, &mut jumps);
                Board::get_possible_jump_br(bc, index, &tiles_to_check, &mut jumps);
                Board::get_possible_jump_tl(bc, index, &tiles_to_check, &mut jumps);
                Board::get_possible_jump_tr(bc, index, &tiles_to_check, &mut jumps);
            }
            _ => {}
        }
        return jumps;
        
    }

    fn get_possible_jump_bl(bc: &BoardContent, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index > 23 {
            return
        }
        if (index % 4) > 0 {
            if bc.tiles[index +7] == TileState::Empty {
                if (index %8) < 4 {
                    if (bc.tiles[index+4] == tiles_to_check[0]) ||
                        (bc.tiles[index+4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+7]));
                    }                            
                }
                else {
                    if (bc.tiles[index+3] == tiles_to_check[0]) ||
                        (bc.tiles[index+3] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+7]));
                    }                            
                }
            }
        }        
    }

    fn get_possible_jump_br(bc: &BoardContent, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index > 23 {
            return
        }
        if (index % 4) < 3 {
            if bc.tiles[index +9] == TileState::Empty {
                if (index %8) < 4 {
                    if (bc.tiles[index+5] == tiles_to_check[0]) ||
                        (bc.tiles[index+5] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+9]));
                    }                            
                }
                else {
                    if (bc.tiles[index+4] == tiles_to_check[0]) ||
                        (bc.tiles[index+4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index+9]));
                    }                            
                }
            }
        }        
    }

    fn get_possible_jump_tl(bc: &BoardContent, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index < 8 {
            return;
        }
        if (index % 4) > 0 {
            if bc.tiles[index -9] == TileState::Empty {
                if (index %8) < 4 {
                    if (bc.tiles[index-4] == tiles_to_check[0]) ||
                        (bc.tiles[index-4] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -9]));
                    }                            
                }
                else {
                    if (bc.tiles[index-5] == tiles_to_check[0]) ||
                        (bc.tiles[index-5] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -9]));
                    }                            
                }
            }
        }    }    

    fn get_possible_jump_tr(bc: &BoardContent, index: usize, tiles_to_check: &Vec<TileState>, jumps: &mut Vec<Jump>)
    {
        if index < 8 {
            return;
        }
        if (index % 4) < 3 {
            if bc.tiles[index -7] == TileState::Empty {
                if (index %8) < 4 {
                    if (bc.tiles[index-3] == tiles_to_check[0]) ||
                        (bc.tiles[index-3] == tiles_to_check[1]) {
                        jumps.push(Jump::new(index, &vec![index -7]));
                    }                            
                }
                else {
                    if (bc.tiles[index-4] == tiles_to_check[0]) ||
                        (bc.tiles[index-4] == tiles_to_check[1]) {
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
    fn test_is_move_invalid() {
        let mut board = Board::new();

        // Invalid array
        let action = ActionMove::new(PlayerColor::Black, &vec![]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![0]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![28, 32]);
        assert!(board.is_move_valid(&action).is_err());

        // Invalid piece type for player
        let action = ActionMove::new(PlayerColor::Black, &vec![12, 16]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![20, 16]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Red, &vec![8, 12]);
        assert!(board.is_move_valid(&action).is_err());

        // Invalid move
        board.bc.tiles[5] = TileState::Empty;
        board.bc.tiles[6] = TileState::Empty;
        board.bc.tiles[8] = TileState::Empty;
        board.bc.tiles[10] = TileState::Empty;
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 5]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 6]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 8]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 10]);
        assert!(board.is_move_valid(&action).is_err());
        board.bc.tiles[5] = TileState::BlackMan;
        board.bc.tiles[6] = TileState::BlackMan;
        board.bc.tiles[8] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;

        // Blocked move
        let action = ActionMove::new(PlayerColor::Black, &vec![5, 8]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![5, 9]);
        assert!(board.is_move_valid(&action).is_err());

        // Invalid jump
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[9] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[6] = TileState::RedMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 0]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Black, &vec![9, 2]);
        assert!(board.is_move_valid(&action).is_err());
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[9] = TileState::RedMan;
        board.bc.tiles[13] = TileState::RedMan;
        board.bc.tiles[14] = TileState::BlackKnight;
        let action = ActionMove::new(PlayerColor::Red, &vec![9, 16]);
        assert!(board.is_move_valid(&action).is_err());
        let action = ActionMove::new(PlayerColor::Red, &vec![9, 18]);
        assert!(board.is_move_valid(&action).is_err());


        // Blocked jump
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[1] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[6] = TileState::RedMan;
        board.bc.tiles[8] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![1, 8]);
        assert!(board.is_move_valid(&action).is_err());
        board.bc.tiles[8] = TileState::Empty;
        board.bc.tiles[10] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![1, 10]);
        assert!(board.is_move_valid(&action).is_err());

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[17] = TileState::RedKnight;
        board.bc.tiles[13] = TileState::BlackKnight;
        board.bc.tiles[14] = TileState::BlackKnight;
        board.bc.tiles[21] = TileState::BlackKnight;
        board.bc.tiles[22] = TileState::BlackKnight;
        board.bc.tiles[8] = TileState::RedMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![17, 8]);
        assert!(board.is_move_valid(&action).is_err());
        board.bc.tiles[8] = TileState::Empty;
        board.bc.tiles[10] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![17, 10]);
        assert!(board.is_move_valid(&action).is_err());
        board.bc.tiles[10] = TileState::Empty;
        board.bc.tiles[24] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![17, 24]);
        assert!(board.is_move_valid(&action).is_err());
        board.bc.tiles[24] = TileState::Empty;
        board.bc.tiles[26] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![17, 26]);
        assert!(board.is_move_valid(&action).is_err());

    }

    #[test]
    fn test_is_move_valid() {
        let mut board = Board::new();

        let action = ActionMove::new(PlayerColor::Black, &vec![8, 12]);
        assert!(board.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Red, &vec![21, 17]);
        assert!(board.is_move_valid(&action).is_ok());

        board.bc.tiles[13] = TileState::RedMan;
        let action = ActionMove::new(PlayerColor::Black, &vec![8, 17]);
        assert!(board.is_move_valid(&action).is_ok());
        board.bc.tiles[13] = TileState::Empty;

        board.bc.tiles[16] = TileState::BlackKnight;
        let action = ActionMove::new(PlayerColor::Red, &vec![20, 13]);
        assert!(board.is_move_valid(&action).is_ok());

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[22] = TileState::RedKnight;
        board.bc.tiles[17] = TileState::BlackMan;
        board.bc.tiles[18] = TileState::BlackMan;
        board.bc.tiles[25] = TileState::BlackMan;
        board.bc.tiles[26] = TileState::BlackMan;
        let action = ActionMove::new(PlayerColor::Red, &vec![22, 13]);
        assert!(board.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Red, &vec![22, 15]);
        assert!(board.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Red, &vec![22, 29]);
        assert!(board.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Red, &vec![22, 31]);
        assert!(board.is_move_valid(&action).is_ok());

    }

    #[test]
    fn test_is_multi_jump_valid() {
        let mut board = Board::new();

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[0] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::RedMan;
        board.bc.tiles[14] = TileState::RedKnight;
        board.bc.tiles[13] = TileState::RedKnight;
        let action = ActionMove::new(PlayerColor::Black, &vec![0, 9, 18]);
        assert!(board.is_move_valid(&action).is_ok());
        let action = ActionMove::new(PlayerColor::Black, &vec![0, 9, 16]);
        assert!(board.is_move_valid(&action).is_ok());

        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[20] = TileState::RedKnight;
        board.bc.tiles[16] = TileState::BlackMan;
        board.bc.tiles[17] = TileState::BlackKnight;
        board.bc.tiles[18] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackKnight;
        let action = ActionMove::new(PlayerColor::Red, &vec![20, 13, 22, 15, 6]);
        assert!(board.is_move_valid(&action).is_ok());
    }


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
    fn test_get_possible_shifts() {
        let board = Board::new();

        // Test #1: default board
        for i in 0..8 {
            assert_eq!(board.get_possible_shifts(i), &[]);
        }
        for i in 12..20 {
            assert_eq!(board.get_possible_shifts(i), &[]);
        }
        for i in 24..32 {
            assert_eq!(board.get_possible_shifts(i), &[]);
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
            let shifts = board.get_possible_shifts(start);
            assert_eq!(shifts.len(), expected_shifts.len(), "Mismatch in number of shifts for position {}", start);
        
            for &to in expected_shifts {
                assert!(shifts.contains(&Shift::new(start, to)), "Shift from {} to {} not found", start, to);
            }
        }
    }

    fn setup_board_with_one_piece(board: &mut Board, index: usize, state: TileState) {
        board.bc.tiles.fill(TileState::Empty);
        board.bc.tiles[index] = state;
    }
    
    fn assert_shifts(board: &Board, index: usize, expected_shifts: &[(usize, usize)]) {
        let shifts = board.get_possible_shifts(index);
        assert_eq!(shifts.len(), expected_shifts.len(), "Mismatch in number of shifts for {:?} at index {}", board.bc.tiles[index], index);
    
        for &(from, to) in expected_shifts {
            assert!(shifts.contains(&Shift::new(from, to)), "Shift from {} to {} not found", from, to);
        }
    }

    #[test]
    fn test_get_possible_shifts_bm() {
        let mut board = Board::new();
 
        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_shifts(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[4] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 4)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[4] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_shifts(&board, index, &[(index, 7)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[7] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 20;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_shifts(&board, index, &[(index, 24)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 23;
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        assert_shifts(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackMan);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);

    }

    #[test]
    fn test_get_possible_shifts_bk() {
        let mut board = Board::new();

        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[4] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 4)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[4] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 7)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[7] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);        


        let index = 4;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 0), (index, 8)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[0] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 8)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[8] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 0)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[0] = TileState::BlackMan;
        board.bc.tiles[8] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 7;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 2), (index, 3), (index, 10), (index, 11)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 3), (index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[3] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2), (index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[3] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 10), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 3), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 3), (index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2), (index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2), (index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[10] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2), (index, 3)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 11)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 10)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 3)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 2)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[2] = TileState::BlackMan;
        board.bc.tiles[3] = TileState::BlackMan;
        board.bc.tiles[10] = TileState::BlackMan;
        board.bc.tiles[11] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        assert_shifts(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::BlackKnight);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);

    }
 
    #[test]
    fn test_get_possible_shifts_rm() {
        let mut board = Board::new();
 
        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_shifts(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_shifts(&board, index, &[(index, 26), (index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 24;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_shifts(&board, index, &[(index, 20), (index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[20] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[21] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 27;
        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        assert_shifts(&board, index, &[(index, 23)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedMan);
        board.bc.tiles[23] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);
        
    }


    #[test]
    fn test_get_possible_shifts_rk() {
        let mut board = Board::new();

        let index = 31;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 26), (index, 27)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[26] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 27)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 26)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[26] = TileState::BlackMan;
        board.bc.tiles[27] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 28;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 24)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[24] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 27;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 23), (index, 31)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[23] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 31)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[31] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 23)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[23] = TileState::BlackMan;
        board.bc.tiles[31] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 24;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 20), (index, 21), (index, 28), (index, 29)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21), (index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[21] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20), (index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 28), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21), (index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20), (index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20), (index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[28] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20), (index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 29)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 28)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 21)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 20)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[20] = TileState::BlackMan;
        board.bc.tiles[21] = TileState::BlackMan;
        board.bc.tiles[28] = TileState::BlackMan;
        board.bc.tiles[29] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 3;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 7)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[7] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);


        let index = 0;
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        assert_shifts(&board, index, &[(index, 4), (index, 5)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[4] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 5)]);

        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[(index, 4)]);
        
        setup_board_with_one_piece(&mut board, index, TileState::RedKnight);
        board.bc.tiles[4] = TileState::BlackMan;
        board.bc.tiles[5] = TileState::BlackMan;
        assert_shifts(&board, index, &[]);

    }

    fn assert_jumps(board: &Board, index: usize, expected_jumps: &[(usize, Vec<usize>)]) {
        let jumps = Board::get_possible_jumps(&board.bc, index);
        assert_eq!(jumps.len(), expected_jumps.len(), "Mismatch in number of jumps for {:?} at index {}", board.bc.tiles[index], index);
    
        for (from, to) in expected_jumps {
            assert!(jumps.contains(&Jump::new(*from, to)), "Jump from {} to {:?} not found", from, to);
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