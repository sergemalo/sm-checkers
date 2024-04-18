use sm_checkers_base::checkers_board::*;

pub trait CheckersUi {
    fn splash_screen(&self);
    fn draw_board(&self, bc: &CheckersBoard);
}

