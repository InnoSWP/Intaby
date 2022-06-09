use async_trait::async_trait;
pub use sqlx::any::AnyPool as DBPool;

use crate::game::GameCode;

pub mod sql;

/// The type of error that may occur when interacting
/// with the database
#[derive(Debug)]
pub enum DBError {
    /// A game could not be created because another
    /// with confliting properties already exists
    GameNotFound(GameCode),
    /// The database is corrupted and could not be
    /// processed properly
    DBCorrupt(String),
    /// Some other error occured
    Other(Box<dyn std::error::Error>),
}

pub type DBResult<T> = Result<T, DBError>;

/// Trait representing a handler for a games database
#[async_trait]
pub trait DBAccessor: Send + Sync {
    /// Add a new game to the database
    async fn create_game(&self) -> DBResult<GameCode>;
    /// Get information about an existing game
    async fn get_game(&self, game_code: &GameCode) -> DBResult<()>;
}

impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GameNotFound(code) => write!(f, "Failed to find a game with the code: {code}"),
            Self::DBCorrupt(msg) => write!(f, "Database is corrupted: {msg}"),
            Self::Other(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for DBError {}
