#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}


impl Move {
    pub fn new(from: usize, to: usize) -> Move {
        Move { from, to }
    }
}