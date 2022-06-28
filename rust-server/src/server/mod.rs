use std::sync::Mutex;

use rocket::{get, post, put, serde::json::Json, State};
use rocket::serde;
use rocket::serde::json::serde_json;

use crate::database::{DBAccessor, DBError};

pub mod games;
mod python_server;

use games::*;

type Database = Box<dyn DBAccessor>;
type SResult<T> = Result<T, Error>;
type GamesState = Mutex<Games>;
type UserId = u64;

#[derive(Debug)]
enum Error {
    Database(DBError),
    GameNotFound(GameCode),
    Reqwest(reqwest::Error),
    BadRequest,
    Internal(Box<dyn std::error::Error>),
}

pub async fn rocket(config: rocket::Config, db_uri: &str) -> rocket::Rocket<rocket::Build> {
    let database = crate::database::sql::SqlAccess::new(db_uri)
        .await
        .expect("Failed to access the database");
    rocket::custom(config)
        .manage(Box::new(database) as Database)
        .manage(Mutex::new(Games::new()))
        .mount("/", rocket::routes![index, create_or_join_game, get_game, game_answer])
}

#[get("/")]
fn index() -> &'static str {
    "This server handles games\n"
}

/// Create a new game from the quiz id
/// or join an existing game with the specified code
#[post("/games/<id>", data = "<name>")]
async fn create_or_join_game(
    id: GameCode,
    name: Option<PlayerName>,
    games: &State<GamesState>,
) -> Result<String, Error> {
    match id.parse::<u64>() {
        Ok(quiz_id) => create_game(quiz_id, games).await,
        Err(_) => match name {
            Some(name) => join_game(id, name, games).await.map(|()| format!("")),
            None => Err(Error::BadRequest),
        },
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

async fn create_game(
    quiz_id: QuizId,
    games: &State<GamesState>,
) -> SResult<GameCode> {
    let quiz_config = python_server::get_quiz(todo!(), quiz_id).await?;
    let code = games.lock().unwrap().create_game(quiz_config);
    Ok(code)
}

async fn join_game(
    game_code: GameCode,
    player_name: PlayerName,
    games: &State<GamesState>,
) -> SResult<()> {
    match games.lock().unwrap().get_game_mut(&game_code) {
        None => Err(Error::GameNotFound(game_code)),
        Some(game) => {
            game.player_join(player_name);
            Ok(())
        }
    }
}

impl Error {
    fn status(&self) -> rocket::http::Status {
        use rocket::http::Status;
        match self {
            Self::Database(error) => error.status(),
            Self::GameNotFound(_) => Status::NotFound,
            Self::BadRequest => Status::BadRequest,
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
            Self::BadRequest => write!(f, "Bad request"),
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

impl From<python_server::Error> for Error {
    fn from(error: python_server::Error) -> Self {
        match error {
            python_server::Error::Reqwest(error) => Self::Reqwest(error),
            python_server::Error::Parse(error) => Self::Internal(Box::new(error)),
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for Error {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        println!("Encountered an error: {self}");
        Err(self.status())
    }
}
