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
        .attach(Cors)
        .mount(
            "/",
            rocket::routes![
                index,
                index_opts,
                create_game,
                games_code_opts,
                join_game,
                get_game_state,
                game_answer,
                change_state,
                change_state_opts,
            ],
        )
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::options("/")]
fn index_opts() {}

#[get("/")]
fn index() -> &'static str {
    "This server handles games\n"
}

#[rocket::options("/games/<code>")]
#[allow(unused_variables)]
fn games_code_opts(code: String) {}

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
    let code = games.lock().unwrap().create_game(user_id, quiz_config);
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

#[rocket::options("/games/<code>/state")]
#[allow(unused_variables)]
fn change_state_opts(code: GameCode) {}

/// Start the newly created game. Only the creator of the game can start it
#[put("/games/<code>/state", data = "<state>")]
async fn change_state(
    code: GameCode,
    state: Json<StateUpdate>,
    games: &State<GamesState>,
) -> SResult<()> {
    let games = &mut games.lock().unwrap();
    let game = get_game_mut(games, &code)?;
    game.change_state(state.0);
    Ok(())
}

#[get("/games/<code>")]
async fn get_game_state(code: GameCode, games: &State<GamesState>) -> SResult<Json<SerGame>> {
    let games = &mut games.lock().unwrap();
    let game = get_game_mut(games, &code)?;
    Ok(Json(game.to_serializable()))
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

fn get_game_mut<'a>(games: &'a mut Games, code: &GameCode) -> SResult<&'a mut Game> {
    match games.get_game_mut(&code) {
        Some(game) => Ok(game),
        None => Err(Error::GameNotFound(code.to_owned())),
    }
}
