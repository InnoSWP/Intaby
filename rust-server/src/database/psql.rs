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

impl DBAccessor for PsqlAccess {
    type Error = PsqlError;

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
        todo!()
    }

    fn start_game(&mut self, game_code: &GameCode) -> Result<(), Self::Error> {
        todo!()
    }

    fn finish_game(&mut self, game_code: &GameCode) -> Result<(), Self::Error> {
        todo!()
    }
}
