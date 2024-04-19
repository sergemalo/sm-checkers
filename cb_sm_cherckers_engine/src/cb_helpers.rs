
use std::fs::OpenOptions;
use std::io::Write;

use std::os::raw::{c_char, c_int, c_double};

use sm_checkers_base::checkers_board::*;

// From cb API (https://www.fierz.ch/cbdeveloper.php)
pub const BOARD_SIZE: usize = 8;


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
        let mut dst_tile_index = 0;
        if let Some(b) = board.as_mut() {  // Check if the pointer is not null and can be safely converted to a reference
            for &value in b.iter() {
            
                writeln!(file, "{}", value).unwrap();
                dst_tile_index = 28 + (src_tile_index / 16) - ((src_tile_index % 8) * 4);
                if ( ( (src_tile_index / 0x8 & 0x1) == 0 && (src_tile_index & 0x1) == 0) ||
                     ( (src_tile_index / 0x8 & 0x1) == 1 && (src_tile_index & 0x1) == 1)) {
                    writeln!(file, "[{}] = {}", dst_tile_index, value).unwrap();
                    
                    match (value & 0x7) {
                        0x0 => my_board.tiles[dst_tile_index] = TileState::Empty,
                        0x6 => my_board.tiles[dst_tile_index] = TileState::BlackMan,
                        0xA => my_board.tiles[dst_tile_index] = TileState::BlackKnight,
                        0x5 => my_board.tiles[dst_tile_index] = TileState::RedMan,
                        0x9 => my_board.tiles[dst_tile_index] = TileState::RedKnight,
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