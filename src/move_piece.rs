#[derive(Debug,PartialEq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn new(from: usize, to: usize) -> Move {
        Move { from, to }
    }
}

#[derive(Debug, PartialEq)]
pub struct Jump {
    pub from: usize,
    pub to: Vec<usize>
}

impl Jump {
    pub fn new(from: usize, to: Vec<usize>) -> Jump {
        Jump { from, to }
    }
}