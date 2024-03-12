
const WHITE_MAN: char = '\u{26C0}';
const BLACK_MAN: char = '\u{26C1}';
const WHITE_KNIGHT: char = '\u{26C2}';
const BLACK_KNIGHT: char = '\u{26C3}';


fn main() {
    println!("SM-Checkers v{} - {} {} {} {}",  env!("CARGO_PKG_VERSION"), WHITE_MAN, BLACK_MAN, WHITE_KNIGHT, BLACK_KNIGHT);
}
