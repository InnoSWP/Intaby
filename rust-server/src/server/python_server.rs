use super::*;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Parse(serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub async fn get_quiz(user_id: UserId, quiz_id: QuizId) -> Result<QuizConfig> {
    let body = reqwest::get(format!("localhost:8888/api/user/{user_id}/quiz/{quiz_id}"))
        .await?
        .text()
        .await?;
    let config = serde::json::from_str(&body)?;
    Ok(config)
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Reqwest(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(error) => error.fmt(f),
            Error::Parse(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
