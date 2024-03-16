use crate::board_content::BoardContent;

pub trait CheckersUi {
    fn splash_screen(&self);
    fn draw_board(&self, bc: &BoardContent);
}

/*/
impl BoardObserver for dyn CheckersUi {
    fn update(&self, bc: &BoardContent) {
        self.draw_board(bc);
    }
}
*/

