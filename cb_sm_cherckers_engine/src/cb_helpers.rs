
use std::fs::OpenOptions;
use std::io::Write;

use std::os::raw::c_int;

use sm_checkers_base::checkers_board::*;

// From cb API (https://www.fierz.ch/cbdeveloper.php)
pub const BOARD_SIZE: usize = 8;

enum CbTileState {
    Empty = 0,
    WhiteMan = 5,
    BlackMan = 6,
    WhiteKnight = 9,
    BlackKnight = 10
}

/*
impl From<CbTileState> for i32 {
    fn from(value: CbTileState) -> i32 {
        match value {
            CbTileState::Empty => 0,
            CbTileState::WhiteMan => 5,
            CbTileState::BlackMan => 6,
            CbTileState::WhiteKnight => 9,
            CbTileState::BlackKnight => 10
        }
    }
}

impl Into<i32> for CbTileState {
    fn into(self) -> i32 {
        match self {
            CbTileState::Empty => 0,
            CbTileState::WhiteMan => 5,
            CbTileState::BlackMan => 6,
            CbTileState::WhiteKnight => 9,
            CbTileState::BlackKnight => 10
        }
    }
}
*/


pub fn cb_board_2_checkers_board(board: *mut [c_int; BOARD_SIZE*BOARD_SIZE]) -> CheckersBoard {
    
    let path = "c:\\tmp\\sm_checkers_engine_cmd_log.txt";

    // Open the file with options to create and append
    let mut file = OpenOptions::new()
        .create(true)  // Create the file if it does not exist
        .append(true)  // Append to the file if it exists
        .open(path)
        .expect("Failed to open file");
    
    // Write some data to the file
    writeln!(file, "BOARD:").unwrap();

    let mut my_board = sm_checkers_base::CheckersBoard::new();

    // Unsafe required because we dereference the pointer received by our API.
    unsafe {
        let mut src_tile_index = 0;
        let mut dst_tile_index;
        if let Some(b) = board.as_mut() {  // Check if the pointer is not null and can be safely converted to a reference
            for &value in b.iter() {
            
                writeln!(file, "{}", value).unwrap();
                if  ( (src_tile_index / 0x8 & 0x1) == 0 && (src_tile_index & 0x1) == 0) ||
                    ( (src_tile_index / 0x8 & 0x1) == 1 && (src_tile_index & 0x1) == 1) {

                    dst_tile_index = cb_idx_2_checkers_board_idx(src_tile_index);
                    writeln!(file, "[{}] = {}", dst_tile_index, value).unwrap();
                    
                    match value & 0xF  {
                        0 => my_board.tiles[dst_tile_index] = TileState::Empty,
                        6 => my_board.tiles[dst_tile_index] = TileState::BlackMan,
                        10 => my_board.tiles[dst_tile_index] = TileState::BlackKnight,
                        5 => my_board.tiles[dst_tile_index] = TileState::RedMan,
                        9 => my_board.tiles[dst_tile_index] = TileState::RedKnight,
                        _ => panic!("Invalid tile value: {}", value)
                    }
                }
                src_tile_index += 1;
            }
        }
    }

    writeln!(file, "BOARD DONE").unwrap();

    return my_board;
}


pub fn checkers_board_2_cb_board(my_board: &CheckersBoard, board: *mut [c_int; BOARD_SIZE*BOARD_SIZE])
{
    let path = "c:\\tmp\\sm_checkers_engine_cmd_log.txt";
    // Open the file with options to create and append
    let mut file = OpenOptions::new()
        .create(true)  // Create the file if it does not exist
        .append(true)  // Append to the file if it exists
        .open(path)
        .expect("Failed to open file");
    
    // Write some data to the file
    writeln!(file, "RETURN BOARD:").unwrap();


    let mut dst_tile_index;
    for i in 0..32 {
        dst_tile_index = checkers_board_idx_2_cb_idx(i);
        unsafe {
            match my_board.tiles[i] {
                TileState::Empty => (*board)[dst_tile_index] = CbTileState::Empty as c_int,
                TileState::BlackMan => (*board)[dst_tile_index] = CbTileState::BlackMan as c_int,
                TileState::BlackKnight => (*board)[dst_tile_index] = CbTileState::BlackKnight as c_int,
                TileState::RedMan => (*board)[dst_tile_index] = CbTileState::WhiteMan as c_int,
                TileState::RedKnight => (*board)[dst_tile_index] = CbTileState::WhiteKnight as c_int

            }
            writeln!(file, "[{}] = {}", dst_tile_index, (*board)[dst_tile_index]).unwrap();
        }
    }
}

fn cb_idx_2_checkers_board_idx(idx: usize) -> usize
{
    match idx {
        0 => 3,
        2 => 11,
        4 => 19,
        6 => 27,
        9 => 7,
        11 => 15,
        13 => 23,
        15 => 31,
        16 => 2,
        18 => 10,
        20 => 18,
        22 => 26,
        25 => 6,
        27 => 14,
        29 => 22,
        31 => 30,
        32 => 1,
        34 => 9,
        36 => 17,
        38 => 25,
        41 => 5,
        43 => 13,
        45 => 21,
        47 => 29,
        48 => 0,
        50 => 8,
        52 => 16,
        54 => 24,
        57 => 4,
        59 => 12,
        61 => 20,
        63 => 28,
        _ => panic!("Invalid index: {}", idx)
    }

}

fn checkers_board_idx_2_cb_idx(idx: usize) -> usize
{
    match idx {
        0 => 48,
        1 => 32,
        2 => 16,
        3 => 0,
        4 => 57,
        5 => 41,
        6 => 25,
        7 => 9,
        8 => 50,
        9 => 34,
        10 => 18,
        11 => 2,
        12 => 59,
        13 => 43,
        14 => 27,
        15 => 11,
        16 => 53,
        17 => 37,
        18 => 21,
        19 => 5,
        20 => 61,
        21 => 45,
        22 => 29,
        23 => 13,
        24 => 54,
        25 => 38,
        26 => 22,
        27 => 6,
        28 => 63,
        29 => 47,
        30 => 31,
        31 => 15,        
        _ => panic!("Invalid index: {}", idx)
    }

}




////////////////////////////////////////////////////////////////////////////////
/// Unit tests
/// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cb_2_checkers_board_default() {
        let mut input_cb: [c_int; BOARD_SIZE*BOARD_SIZE] = [
            CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int,
            CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int,
            CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int,
            CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int
        ];

        let my_board = cb_board_2_checkers_board(&mut input_cb);
        let ref_board = CheckersBoard::new();

        assert!(my_board.tiles == ref_board.tiles);
    }

    #[test]
    fn test_cb_2_checkers_board_knights() {
        let mut input_cb: [c_int; BOARD_SIZE*BOARD_SIZE] = [
            CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int,
            CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int,
            CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int,
            CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int, CbTileState::WhiteKnight as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int, CbTileState::Empty as c_int, CbTileState::BlackKnight as c_int,
        ];

        let my_board = cb_board_2_checkers_board(&mut input_cb);

        assert!(my_board.tiles[0] == TileState::RedKnight);
        assert!(my_board.tiles[1] == TileState::BlackKnight);
        assert!(my_board.tiles[2] == TileState::Empty);
        assert!(my_board.tiles[3] == TileState::RedKnight);

        assert!(my_board.tiles[4] == TileState::BlackKnight);
        assert!(my_board.tiles[5] == TileState::Empty);
        assert!(my_board.tiles[6] == TileState::RedKnight);
        assert!(my_board.tiles[7] == TileState::BlackKnight);
                
    }

    #[test]
    fn checkers_board_2_cb_board_default() {
        let my_board = CheckersBoard::new();
        let mut output_cb: [c_int; BOARD_SIZE*BOARD_SIZE] = [0; BOARD_SIZE*BOARD_SIZE];
        checkers_board_2_cb_board(&my_board, &mut output_cb);

        let my_board2 = cb_board_2_checkers_board(&mut output_cb);
        for i in 0..32 {
            assert!(my_board.tiles[i] == my_board2.tiles[i]);
        }
    }

}
