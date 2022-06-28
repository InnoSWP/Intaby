use std::sync::Mutex;
use rocket::{get, post, put, serde::json::Json, State};

use crate::database::{DBAccessor, DBError};
use crate::model::*;

type Database = Box<dyn DBAccessor>;
type SResult<T> = Result<T, Error>;
type GamesState = Mutex<Games>;

#[derive(Debug)]
enum Error {
    Database(DBError),
    GameNotFound(GameCode),
    Reqwest(reqwest::Error),
    Internal(Box<dyn std::error::Error>),
}

pub async fn rocket(config: rocket::Config, db_uri: &str) -> rocket::Rocket<rocket::Build> {
    let database = crate::database::sql::SqlAccess::new(db_uri)
        .await
        .expect("Failed to access the database");
    rocket::custom(config)
        .manage(Box::new(database) as Database)
        .manage(Mutex::new(Games::new()))
        .mount(
            "/",
            rocket::routes![index, create_game, join_game, get_game, game_answer],
        )
}

#[get("/")]
fn index() -> &'static str {
    "This server handles games\n"
}

/// Create a new game from the quiz id
#[post("/games/<id>", data = "<user_id>")]
async fn create_game(
    id: QuizId,
    user_id: Json<UserId>,
    games: &State<GamesState>,
) -> SResult<GameCode> {
    let user_id = user_id.0;
    let quiz_config = crate::web_client::get_quiz(user_id, id).await?;
    let code = games.lock().unwrap().create_game(quiz_config);
    Ok(code)
}

/// Join an existing game with the specified code
#[post("/games/<code>", data = "<name>", rank = 2)]
async fn join_game(code: GameCode, name: PlayerName, games: &State<GamesState>) -> SResult<()> {
    match games.lock().unwrap().get_game_mut(&code) {
        None => Err(Error::GameNotFound(code)),
        Some(game) => {
            game.player_join(name);
            Ok(())
        }
    }
}

#[get("/games/<code>")]
async fn get_game(code: GameCode, games: &State<GamesState>) -> SResult<GameCode> {
    match games.lock().unwrap().get_game(&code) {
        Some(_game) => Ok(code),
        None => Err(Error::GameNotFound(code)),
    }
}

#[put("/games/<code>", data = "<answer>")]
async fn game_answer(code: GameCode, answer: Json<GameAnswer>) -> SResult<()> {
    // TODO: register answer
    Ok(())
}

impl Error {
    fn status(&self) -> rocket::http::Status {
        use rocket::http::Status;
        match self {
            Self::Database(error) => error.status(),
            Self::GameNotFound(_) => Status::NotFound,
            Self::Reqwest(_) => Status::InternalServerError,
            Self::Internal(_) => Status::InternalServerError,
        }
    }
}

impl DBError {
    fn status(&self) -> rocket::http::Status {
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
            Self::Reqwest(error) => error.fmt(f),
            Self::Internal(error) => error.fmt(f),
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
            crate::web_client::Error::Reqwest(error) => Self::Reqwest(error),
            crate::web_client::Error::Parse(error) => Self::Internal(Box::new(error)),
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for Error {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        println!("Encountered an error: {self}");
        Err(self.status())
    }
}
