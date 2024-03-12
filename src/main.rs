

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

fn main() {
    println!("SM-Checkers v{} - {} {} {} {}",  env!("CARGO_PKG_VERSION"), WHITE_MAN, BLACK_MAN, WHITE_KNIGHT, BLACK_KNIGHT);
    print_empty_board();
}
