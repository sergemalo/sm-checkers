//
//https://www.fierz.ch/cbdeveloper.php
//
// int WINAPI getmove(int board[8][8], int color, double maxtime, char str[1024], int *playnow, int info, int moreinfo, struct CBmove *move);
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
//
//
// int WINAPI enginecommand(char command[256], char reply[1024]);


use std::fs::OpenOptions;
use std::io::Write;

use std::os::raw::{c_char, c_int, c_double};
use std::ffi::{CString, CStr};

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

const BOARD_SIZE: usize = 8;
#[no_mangle]
pub extern "stdcall" fn getmove(
    board: *mut [c_int; BOARD_SIZE],    // int board[8][8], 
    color:c_int,                        // int color, 
    maxtime: c_double,                  // double maxtime, 
    str: *mut c_char,                   // char str[1024], 
    playnow: *mut c_int,                // int *playnow,
    info: c_int,                        // int info, 
    moreinfo: c_int,                    // int moreinfo,
    cb_move: *mut CBmove) -> c_int {       // struct CBmove *move


    // Function body here
    3 // Return value example, modify as needed
}





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
        response_str = "Serge Malo's Rust Checkers Engine";
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
        response_str = "sm_checkers_engine";
    }

    writeln!(file, "ANSER: <{}>", response_str).unwrap();


    let reply_cstring = CString::new(response_str).expect("Failed to create reply CString");
    unsafe {
        std::ptr::copy_nonoverlapping(reply_cstring.as_ptr(), reply, reply_cstring.as_bytes().len());
    }

    0 // Return value example, modify as needed
}





