pub enum PlayerAction {
    Quit,
    Move
}

pub trait Player {
    fn play_turn(&self) -> &PlayerAction;
}