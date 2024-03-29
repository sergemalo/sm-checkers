use std::any::Any;

pub trait Movement {
    fn from(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug,PartialEq)]
pub struct Shift {
    from: usize,
    pub to: usize,
}

impl Shift {
    pub fn new(from: usize, to: usize) -> Shift {
        Shift { from, to }
    }
}

impl Movement for Shift {
    fn from(&self) -> usize {
        return self.from;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct Jump {
    from: usize,
    pub to: Vec<usize>
}

impl Jump {
    pub fn new(from: usize, to: &Vec<usize>) -> Jump {
        Jump { 
            from, 
            to: to.clone() 
        }
    }
}

impl Movement for Jump {
    fn from(&self) -> usize {
        self.from
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

}
