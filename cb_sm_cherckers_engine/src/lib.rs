//
// https://www.fierz.ch/cbdeveloper.php
//
// int WINAPI getmove(int board[8][8], 
//                    int color,
//                    double maxtime,
//                    char str[1024], 
//                    int *playnow, 
//                    int info, 
//                    int moreinfo, 
//                    struct CBmove *move);
//
// Where:
// struct CBmove
// {
//    int jumps; // number of jumps in this move
//    int newpiece; // moving piece after jump
//    int oldpiece; // moving piece before jump
//    struct coor from,to; // from,to squares of moving piece
//    struct coor path[12]; // intermediate squares to jump to
//    struct coor del[12]; // squares where men are removed
//    int delpiece[12]; // piece type which is removed
//  }
// Getmove should return a value between 0 and 3, defined as follows:
// #define DRAW 0
// #define WIN 1
// #define LOSS 2
// #define UNKNOWN 3
//
//
// int WINAPI enginecommand(char command[256], char reply[1024]);
// int WINAPI enginecommand(char command[256], char reply[512]);  ===> MOST LIKELY


use std::fs::OpenOptions;
use std::io::Write;

use std::os::raw::{c_char, c_int, c_double};
use std::ffi::{CString, CStr};

use crate::cb_helpers::*;
mod cb_helpers;

// Define the CBmove struct
#[repr(C)]
pub struct CBmove {
    jumps: c_int,
    newpiece: c_int,
    oldpiece: c_int,
    from: coor,
    to: coor,
    path: [coor; 12],
    del: [coor; 12],
    delpiece: [c_int; 12]
}

// Define the coor struct
#[repr(C)]
pub struct coor {
    // Define the fields of the coor struct here
    x: c_int,
    y: c_int,
}

#[no_mangle]
pub extern "stdcall" fn getmove(
    board: *mut [c_int; BOARD_SIZE*BOARD_SIZE],    // int board[8][8], 
    color:c_int,                        // int color, 
    maxtime: c_double,                  // double maxtime, 
    short_reply: *mut c_char,                   // char str[1024], 
    playnow: *mut c_int,                // int *playnow,
    info: c_int,                        // int info, 
    moreinfo: c_int,                    // int moreinfo,
    cb_move: *mut CBmove) -> c_int {       // struct CBmove *move

        
    let path = "c:\\tmp\\sm_checkers_engine_cmd_log.txt";

    {
        // Open the file with options to create and append
        let mut trace_file = OpenOptions::new()
            .create(true)  // Create the file if it does not exist
            .append(true)  // Append to the file if it exists
            .open(path)
            .expect("Failed to open file");
        
        // Write some data to the file
        writeln!(trace_file, "GETMOVE: ").unwrap();


        writeln!(trace_file, "Info: {}", info).unwrap();
        writeln!(trace_file, "Moreinfo: {}", moreinfo).unwrap();

        writeln!(trace_file, "COLOR: {}", color).unwrap();
        writeln!(trace_file, "MAXTIME: {}", maxtime).unwrap();

    }

    // Transform board to chckersBoard
    let mut my_board = cb_board_2_checkers_board(board);
    // Notify board observers

    unsafe {

        (*board)[2] = 0x0;
        (*board)[11] = 0x6;
        
    }

    let short_message = "Je pense...\n";
    let short_message_cstring = CString::new(short_message).expect("Failed to create reply CString");
    unsafe {
        std::ptr::write_bytes(short_reply, 0, 1024);
        std::ptr::copy_nonoverlapping(short_message_cstring.as_ptr(), short_reply, short_message_cstring.as_bytes().len());
    }







    return 3;
}


////////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub extern "stdcall" fn enginecommand(command: *mut c_char, reply: *mut c_char) -> c_int {
    let mut command_str = unsafe { CStr::from_ptr(command).to_str().unwrap() }; // Convert C string to Rust string
    let mut response_str = "?"; // Your response message

    let path = "c:\\tmp\\sm_checkers_engine_cmd_log.txt";
    
    // Open the file with options to create and append
    let mut file = OpenOptions::new()
        .create(true)  // Create the file if it does not exist
        .append(true)  // Append to the file if it exists
        .open(path)
        .expect("Failed to open file");
    
    // Write some data to the file
    writeln!(file, "CDM: <{}>", command_str).unwrap();


    command_str = command_str.trim();
    let cmd = command_str.to_lowercase();

    if cmd == "about" {
        //response_str = "Serge Malo's Rust Checkers Engine\r\nhttps://github.com/smalo/sm_checkers_engine\r\n";
        response_str = "Serge 1.0";
    }
    else  if cmd == "get protocolversion" {
        response_str = "2";
    }
    else  if cmd == "get gametype" {
        response_str = "21"; // American/English: 21, Italian: 22, Spanish: 24, Russian: 25, Brazilian: 26.
    }
    else  if cmd == "get book" {
        response_str = "0"; // print the book strength in the reply. Currently, CheckerBoard supports values 0...3, meaning no book, all kinds of moves, good moves, best moves, respectively. How you want to interpret the book strength is your decision.
    }
    else  if cmd == "get hashsize" {
        response_str = "0";
    }
    else  if cmd == "get dbmbytes" {
        response_str = "0";
    }
    else  if cmd == "get allscores" {
        response_str = "0"; // print 1 if you are in all scores mode (the engine displays a list of all moves with their scores instead of the normal search info), 0 if you are in normal mode. The all scores mode is a good tool for a human to help in analysis
    }
    else  if cmd == "help" {
        response_str = "https://www.fierz.ch/cbdeveloper.php";
    }
    else  if cmd == "name" {
        response_str = "cb_sm_checkers_engine\n";
    }

    let reply_cstring = CString::new(response_str).expect("Failed to create reply CString");
    writeln!(file, "ANSWER: <{}> {}", response_str, response_str.len()).unwrap();
    writeln!(file, "ANSWER LEN:  {}", reply_cstring.as_bytes().len()).unwrap();
    unsafe {
        std::ptr::write_bytes(reply, 0, 512);
        std::ptr::copy_nonoverlapping(reply_cstring.as_ptr(), reply, reply_cstring.as_bytes().len());
    }

 
    0 // Return value example, modify as needed
}





