use super::*;

pub use postgres::Error as PsqlError;
use postgres::{Client, NoTls};

pub type PsqlResult<T> = Result<T, PsqlError>;

/// Handles access to the postgres games database
pub struct PsqlAccess {
    client: Client,
}

impl PsqlAccess {
    /// Sets up access to the database and initializes all the neccessary tables
    pub fn new() -> PsqlResult<Self> {
        let client = Client::connect("host=localhost user=test password=test", NoTls)?;
        let db = Self { client }.init()?;
        Ok(db)
    }

    /// Initialize all the neccessary database stuff
    fn init(mut self) -> PsqlResult<Self> {
        self.client.batch_execute(
            "
            CREATE TABLE IF NOT EXISTS games (
                id          SERIAL PRIMARY KEY,
                code        VARCHAR NOT NULL
                state       INT
                )
            ",
        )?;
        Ok(self)
    }
}

#[derive(Debug)]
pub enum Error {
    Psql(PsqlError),
    GameNotFound(GameCode),
    DBCorrupt,
}

impl DBAccessor for PsqlAccess {
    type Error = Error;

    fn create_game(&mut self) -> Result<GameCode, Self::Error> {
        let code = game::game_code_generator();
        let state = GameState::Lobby;
        self.client.execute(
            "INSERT INTO games (code, state) VALUES ($1, $2)",
            &[&code, &(state as i32)],
        )?;
        Ok(code)
    }

    fn get_game(&mut self, game_code: &GameCode) -> Result<GameState, Self::Error> {
        let res = self
            .client
            .query("SELECT state FROM games WHERE code == $1", &[game_code])?;
        match &res[..] {
            [] => Err(Error::GameNotFound(game_code.to_owned())),
            [row] => {
                let state = row.get::<usize, i32>(0);
                u8::try_from(state)
                    .map_err(|_| Error::DBCorrupt)
                    .and_then(|state| GameState::try_from(state).map_err(|_| Error::DBCorrupt))
            }
            _ => panic!("Expected at most one game to match the code"), // TODO: move that check to game creation
        }
    }
}

impl From<PsqlError> for Error {
    fn from(error: PsqlError) -> Self {
        Self::Psql(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Psql(error) => error.fmt(f),
            Self::GameNotFound(code) => write!(f, "Failed to find a game with the code: {code}"),
            Self::DBCorrupt => write!(f, "Database appears to be corrupt. Oops..."),
        }
    }
}

impl std::error::Error for Error {}
