pub mod psql;

use crate::game::{self, GameCode, GameState};

/// Trait representing a handler for a games database
pub trait DBAccessor {
    /// The type of errors that may occur
    type Error: std::error::Error;
    /// Add a new game to the database
    fn create_game(&mut self) -> Result<GameCode, Self::Error>;
    /// Get information about an existing game
    fn get_game(&mut self, game_code: &GameCode) -> Result<GameState, Self::Error>;
}
