use rocket::{get, post, State};

use crate::database::{DBAccessor, DBError, DBResult};
use crate::game::GameCode;

type Database = Box<dyn DBAccessor>;

pub async fn rocket(config: rocket::Config, db_uri: &str) -> rocket::Rocket<rocket::Build> {
    let database = crate::database::sql::SqlAccess::new(db_uri)
        .await
        .expect("Failed to access the database");
    rocket::custom(config)
        .manage(Box::new(database) as Database)
        .mount("/", rocket::routes![index, create_game, get_game])
}

#[get("/")]
fn index() -> &'static str {
    "This server handles games\n"
}

#[post("/games")]
async fn create_game(database: &State<Database>) -> DBResult<String> {
    let game_code = database.inner().create_game().await?;
    Ok(format!("Created a new game with the code: {game_code}"))
}

#[get("/games/<code>")]
async fn get_game(code: GameCode, database: &State<Database>) -> DBResult<GameCode> {
    let () = database.inner().get_game(&code).await?;
    Ok(format!("Found the game with the code: {code}"))
}

impl DBError {
    fn status(&self) -> rocket::http::Status {
        use rocket::http::Status;
        match self {
            DBError::GameNotFound(_) => Status::NotFound,
            DBError::DBCorrupt(_) => Status::InternalServerError,
            DBError::Other(_) => Status::InternalServerError,
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for DBError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        println!("Encountered an error: {self}");
        Err(self.status())
    }
}
