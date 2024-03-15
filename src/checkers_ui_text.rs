use crate::checkers_ui::CheckersUi;
use crate::board::BoardContent;
use crate::board::BoardObserver;



const WHITE_MAN: char = '\u{26C0}';
const BLACK_MAN: char = '\u{26C1}';
const WHITE_KNIGHT: char = '\u{26C2}';
const BLACK_KNIGHT: char = '\u{26C3}';
const BLACK_SQUARE: char = '\u{2BC0}';
const WHITE_SQUARE: char = '\u{2610}';

fn print_empty_board() {
    let l0 = String::from(format!("| {} | {} ", BLACK_SQUARE, WHITE_SQUARE));
    let l1 = String::from(format!("| {} | {} ", WHITE_SQUARE, BLACK_SQUARE));
    let mut line0 = String::new();
    let mut line1 = String::new();
    for _ in 0..4 {
        line0.push_str(l0.as_str());
        line1.push_str(l1.as_str());
    }
    line0.push('|');
    line1.push('|');
    for _ in 0..4 {
        println!("{}", line0);
        println!("{}", line1);
    }
}


pub struct CheckersUiText {
    // fields go here
}

impl CheckersUiText {
    pub fn new() -> CheckersUiText {
        CheckersUiText {
            // fields go here
        }
    }
}


impl BoardObserver for CheckersUiText {
    fn update(&self, bc: &BoardContent) {
        self.draw_board(bc);
    }
}


impl CheckersUi for CheckersUiText {
    fn splash_screen(&self) {
        println!("SM-Checkers v{} - {} {} {} {}",  env!("CARGO_PKG_VERSION"), WHITE_MAN, BLACK_MAN, WHITE_KNIGHT, BLACK_KNIGHT);
        print_empty_board();
   }

    fn draw_board(&self, bc: &BoardContent) {
        print_empty_board();
    }
}