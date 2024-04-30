// Checkers colors are TRICKY !!!
//
// We have 3 possible colors: White, Red and Black
// So, we have 3 possible games:
// White vs Red
// White vs Black
// Red   vs Black
//
// It is commonly known that the "dark player" plays first.
// RULES:
// Black always plays first
// White always plays second
// Thus, when Red plays agains black, Red plays second
//       when Red plays agains black, Red plays first !
//
// In My implementation, I decided to use Black vs Red

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    Red
}

pub fn opposite_color(color_in: Color) -> Color {
    match color_in {
        Color::Black => Color::Red,
        Color::Red => Color::Black
    }
}