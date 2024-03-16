#[derive(Debug, Copy, Clone)]
pub enum TileState {
    Empty,
    RedMan,
    RedKnight,
    BlackMan,
    BlackKnight
}

#[derive(Debug)]
pub struct BoardContent {
    pub tiles: [TileState; 32]
}

impl BoardContent {
    pub fn new() -> Self {
        let mut temp_tiles: [TileState; 32] = [TileState::Empty; 32];
        for i in 0..12 {
            temp_tiles[i] = TileState::BlackMan;
            temp_tiles[i+20] = TileState::RedMan;
        }
        BoardContent {
            tiles: temp_tiles
        }
    }
}

pub trait BoardObserver {
    fn update(&self, bc: &BoardContent);
}
