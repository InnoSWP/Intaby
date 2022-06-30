use rocket::{get, post, put, serde::json::Json, State};
use std::sync::Mutex;

use crate::model::*;

mod error;

use error::*;

type WebClient = Box<dyn crate::web_client::WebClient>;
type SResult<T> = Result<T, Error>;
type GamesState = Mutex<Games>;

pub async fn rocket(
    config: rocket::Config,
    web_client: WebClient,
) -> rocket::Rocket<rocket::Build> {
    rocket::custom(config)
        .manage(web_client)
        .manage(Mutex::new(Games::new()))
        .mount(
            "/",
            rocket::routes![index, create_game, join_game, get_game_state, game_answer],
        )
}

#[get("/")]
fn index() -> &'static str {
    "This server handles games\n"
}

/// Create a new game from the quiz id
#[post("/games/<quiz_id>", data = "<user_id>")]
async fn create_game(
    quiz_id: QuizId,
    user_id: Json<UserId>,
    games: &State<GamesState>,
    web_client: &State<WebClient>,
) -> SResult<GameCode> {
    let user_id = user_id.0;
    let quiz_config = web_client.get_quiz(user_id, quiz_id).await?;
    let code = games.lock().unwrap().create_game(quiz_config);
    Ok(code)
}

/// Join an existing game with the specified code
#[post("/games/<code>", data = "<player>", rank = 2)]
async fn join_game(code: GameCode, player: Json<Player>, games: &State<GamesState>) -> SResult<()> {
    match games.lock().unwrap().get_game_mut(&code) {
        None => Err(Error::GameNotFound(code)),
        Some(game) => {
            game.player_join(player.0);
            Ok(())
        }
    }
}

#[get("/games/<code>")]
async fn get_game_state(code: GameCode, games: &State<GamesState>) -> SResult<GameCode> {
    // TODO: proper game json
    let _ = get_game(&games.lock().unwrap(), &code)?;
    Ok(code)
}

#[put("/games/<code>", data = "<answer>")]
async fn game_answer(
    code: GameCode,
    answer: Json<GameAnswer>,
    games: &State<GamesState>,
) -> SResult<()> {
    let games = &mut games.lock().unwrap();
    let game = get_game_mut(games, &code)?;
    game.player_answer(answer.0);
    Ok(())
}

fn get_game<'a>(games: &'a Games, code: &GameCode) -> SResult<&'a Game> {
    match games.get_game(&code) {
        Some(game) => Ok(game),
        None => Err(Error::GameNotFound(code.to_owned())),
    }
}

fn get_game_mut<'a>(games: &'a mut Games, code: &GameCode) -> SResult<&'a mut Game> {
    match games.get_game_mut(&code) {
        Some(game) => Ok(game),
        None => Err(Error::GameNotFound(code.to_owned())),
    }
}
