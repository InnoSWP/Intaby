use super::*;

use sqlx::AnyPool;

/// Handles access to the postgres games database
pub struct SqlAccess {
    pool: AnyPool,
}

impl SqlAccess {
    /// Sets up access to the database and initializes all the neccessary tables
    pub async fn new(db_uri: &str) -> DBResult<Self> {
        let pool = DBPool::connect(db_uri)
            .await
            .expect("Failed to connect to the database");
        let sql = Self { pool }.init().await?;
        Ok(sql)
    }

    /// Initialize all the neccessary database stuff
    async fn init(self) -> DBResult<Self> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS games (
                id          SERIAL PRIMARY KEY,
                code        VARCHAR NOT NULL
                )
            ",
        )
        .execute(&self.pool)
        .await?;
        Ok(self)
    }
}

#[async_trait]
impl DBAccessor for SqlAccess {
    async fn create_game(&self) -> DBResult<GameCode> {
        let code = crate::game::game_code_generator();
        let query = format!(
            "INSERT INTO games (code) VALUES (\'{}\') RETURNING id",
            code
        );
        let _ = sqlx::query(&query).fetch_one(&self.pool).await?;
        Ok(code)
    }

    async fn get_game(&self, game_code: &GameCode) -> DBResult<()> {
        let query = format!("SELECT FROM games WHERE code=\'{}\'", game_code);
        let res = sqlx::query(&query).fetch_all(&self.pool).await?;
        match &res[..] {
            [] => Err(DBError::GameNotFound(game_code.to_owned())),
            [_row] => Ok(()),
            _ => Err(DBError::DBCorrupt(format!(
                "More than one game with equal codes exist"
            ))),
        }
    }
}

impl From<sqlx::Error> for DBError {
    fn from(error: sqlx::Error) -> Self {
        Self::Other(Box::new(error))
    }
}
