use super::*;

#[derive(Debug)]
pub enum Error {
    Database(DBError),
    GameNotFound(GameCode),
    Internal(Box<dyn std::error::Error>),
    Other(Box<dyn std::error::Error>),
}

impl Error {
    pub fn status(&self) -> rocket::http::Status {
        use rocket::http::Status;
        match self {
            Self::Database(error) => error.status(),
            Self::GameNotFound(_) => Status::NotFound,
            Self::Internal(_) => Status::InternalServerError,
            Self::Other(_) => Status::InternalServerError,
        }
    }
}

impl DBError {
    pub fn status(&self) -> rocket::http::Status {
        use rocket::http::Status;
        match self {
            Self::QuizNotFound(_) => Status::NotFound,
            Self::DBCorrupt(_) => Status::InternalServerError,
            Self::Other(_) => Status::InternalServerError,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => error.fmt(f),
            Self::GameNotFound(msg) => {
                write!(f, "A game with the code \'{msg}\' could not be found")
            }
            Self::Internal(error) => error.fmt(f),
            Self::Other(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<DBError> for Error {
    fn from(error: DBError) -> Self {
        Self::Database(error)
    }
}

impl From<crate::web_client::Error> for Error {
    fn from(error: crate::web_client::Error) -> Self {
        match error {
            crate::web_client::Error::Parse(error) => Self::Internal(Box::new(error)),
            crate::web_client::Error::Other(error) => Self::Other(error),
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for Error {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        println!("Encountered an error: {self}");
        Err(self.status())
    }
}
