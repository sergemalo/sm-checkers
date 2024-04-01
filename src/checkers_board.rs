#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileState {
    Empty,
    RedMan,
    RedKnight,
    BlackMan,
    BlackKnight
}

#[derive(Debug, Clone)]
pub struct CheckersBoard {
    pub tiles: [TileState; 32]
}

impl CheckersBoard {
    pub fn new() -> Self {
        let mut default_tiles: [TileState; 32] = [TileState::Empty; 32];
        for i in 0..12 {
            default_tiles[i] = TileState::BlackMan;
            default_tiles[i+20] = TileState::RedMan;
        }
        CheckersBoard {
            tiles: default_tiles
        }
    }
}

pub trait GameBoardObserver {
    fn update(&mut self, bc: &CheckersBoard);
}
