
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
                dst_tile_index = 28 + (src_tile_index / 16) - ((src_tile_index % 8) * 4);
                if  ( (src_tile_index / 0x8 & 0x1) == 0 && (src_tile_index & 0x1) == 0) ||
                    ( (src_tile_index / 0x8 & 0x1) == 1 && (src_tile_index & 0x1) == 1) {
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
    let mut dst_tile_index;
    for i in 0..32 {
        if (i % 8) < 4 {
            dst_tile_index = 8*(1 + 2*(i%4)) + 7-(i/4);
        }
        else {
            dst_tile_index = 16*(i%4) + 7-(i/4);
        }
        unsafe {
            match my_board.tiles[i] {
                TileState::Empty => (*board)[dst_tile_index] = CbTileState::Empty as c_int,
                TileState::BlackMan => (*board)[dst_tile_index] = CbTileState::BlackMan as c_int,
                TileState::BlackKnight => (*board)[dst_tile_index] = CbTileState::BlackKnight as c_int,
                TileState::RedMan => (*board)[dst_tile_index] = CbTileState::WhiteMan as c_int,
                TileState::RedKnight => (*board)[dst_tile_index] = CbTileState::WhiteKnight as c_int
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
    fn test_cb_2_checkers_board_default() {
        let mut input_cb: [c_int; BOARD_SIZE*BOARD_SIZE] = [
            CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int,
            CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int,
            CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int,
            CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int,
            CbTileState::Empty as c_int, CbTileState::WhiteMan as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int, CbTileState::Empty as c_int, CbTileState::BlackMan as c_int
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

        for i in 0..1 {
            assert!(my_board.tiles[i*8+0] == TileState::BlackKnight);
            assert!(my_board.tiles[i*8+1] == TileState::RedKnight);
            assert!(my_board.tiles[i*8+2] == TileState::Empty);
            assert!(my_board.tiles[i*8+3] == TileState::BlackKnight);
    
            assert!(my_board.tiles[i*8+4] == TileState::RedKnight);
            assert!(my_board.tiles[i*8+5] == TileState::Empty);
            assert!(my_board.tiles[i*8+6] == TileState::BlackKnight);
            assert!(my_board.tiles[i*8+7] == TileState::RedKnight);
                
        }

    }

    #[test]
    fn checkers_board_2_cb_board_fefault() {
        let my_board = CheckersBoard::new();
        let mut output_cb: [c_int; BOARD_SIZE*BOARD_SIZE] = [0; BOARD_SIZE*BOARD_SIZE];
        checkers_board_2_cb_board(&my_board, &mut output_cb);

        let my_board2 = cb_board_2_checkers_board(&mut output_cb);

        assert!(my_board.tiles == my_board2.tiles);

    }

}
