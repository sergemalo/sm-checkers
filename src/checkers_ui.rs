use crate::checkers_board::CheckersBoard;

pub trait CheckersUi {
    fn splash_screen(&self);
    fn draw_board(&self, bc: &CheckersBoard);
}

