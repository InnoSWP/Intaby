use crate::model::*;
use async_trait::async_trait;

pub mod reqwest_client;

#[derive(Debug)]
pub enum Error {
    /// Web client failed to parse an incoming message
    Parse(serde_json::Error),
    /// Some other error, typically related to connection
    Other(Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Trait representing a handler for the database
#[async_trait]
pub trait WebClient: Send + Sync {
    async fn get_quiz(&self, user_id: UserId, quiz_id: QuizId) -> Result<QuizConfig>;
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(error) => error.fmt(f),
            Error::Parse(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
