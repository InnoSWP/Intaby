/// Represents different states of a game
#[derive(Debug, Clone, Copy, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum GameState {
    /// Initial state of the game
    Lobby,
    InProgress,
    Finished,
}

pub type GameCode = String;

pub fn game_code_generator() -> GameCode {
    use rand::Rng;
    const LEN: usize = 4;

    let mut rng = rand::thread_rng();
    let mut code = String::new();
    for _ in 0..LEN {
        let symbol = rng.gen_range('A'..='Z');
        code.push(symbol);
    }
    code
}
