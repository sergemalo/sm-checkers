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
// If you plan to write an engine which plays English/American checkers, you can immediately forget about the struct CBmove again.
//
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
    from: Coor,
    to: Coor,
    path: [Coor; 12],
    del: [Coor; 12],
    delpiece: [c_int; 12]
}

// Define the coor struct
#[repr(C)]
pub struct Coor {
    // Define the fields of the Coor struct here
    x: c_int,
    y: c_int,
}



use std::sync::{Arc, Mutex, Once};
use std::rc::Rc;
use std::cell::RefCell;
use sm_checkers_base::checkers_board::*;

pub trait Singleton {
    fn get_instance() -> Arc<Mutex<Self>> where Self: Sized + 'static;
}

// Define the Subject trait
pub trait Subject {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn GameBoardObserver>>);
    fn remove_observer(&mut self, bo: Rc<RefCell<dyn GameBoardObserver>>);
    fn notify_observers(&self);
}

impl Subject for BoardReceiver {
    fn register_observer(&mut self, bo: Rc<RefCell<dyn GameBoardObserver>>) {
        self.observers.push(bo);
    }

    fn remove_observer(&mut self, bo: Rc<RefCell<dyn GameBoardObserver>>) {
        let index = self.observers.iter().position(|o| Rc::ptr_eq(o, &bo));

        if let Some(index) = index {
            self.observers.remove(index);
        }
    }

    fn notify_observers(&self) {
        for observer in self.observers.iter() {
            // Call the update method of the observer
            observer.borrow_mut().update(&self.game_board);
        }
    }
}

impl Singleton for BoardReceiver {
    fn get_instance() -> Arc<Mutex<Self>> {
        static ONCE: Once = Once::new();
        static mut SINGLETON: Option<Arc<Mutex<BoardReceiver>>> = None;

        unsafe {
            ONCE.call_once(|| {
                // Initialize with default values or configuration
                let br = BoardReceiver::new();
                SINGLETON = Some(Arc::new(Mutex::new(br)));
            });

            SINGLETON.clone().unwrap()
        }
    }
}

pub struct BoardReceiver {
    observers: Vec<Rc<RefCell<dyn GameBoardObserver>>>,
    game_board: CheckersBoard
}

impl BoardReceiver {
    fn new() -> BoardReceiver {
        BoardReceiver {
            observers: Vec::new(),
            game_board: CheckersBoard::new()
        }
    }
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
    _cb_move: *mut CBmove) -> c_int {   // struct CBmove *move - UNUSED

        
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


        writeln!(trace_file, "COLOR: {}", color).unwrap();
        writeln!(trace_file, "MAXTIME: {}", maxtime).unwrap();

        unsafe {
            writeln!(trace_file, "PLAYNOW: {}", *playnow).unwrap();
        }

        writeln!(trace_file, "Info: {}", info).unwrap();
        writeln!(trace_file, "Moreinfo: {}", moreinfo).unwrap();


    }

    // Receive Board: Transform board to our reprensentation and notify all observers
    let br = BoardReceiver::get_instance();
    let mut br = br.lock().unwrap();
    br.game_board = cb_board_2_checkers_board(board);
    br.notify_observers();


    // Fake move
    br.game_board.tiles[16] = TileState::BlackMan;
    br.game_board.tiles[20] = TileState::Empty;

    checkers_board_2_cb_board(&(br.game_board), board);

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





