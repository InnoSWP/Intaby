use rocket::tokio;
use rocket::{http::Status, local::asynchronous::Client};

use crate::model::*;

mod end_to_end;
mod mock;

use end_to_end::setup_end_to_end;
use mock::setup_mock;

macro_rules! request {
    ($request: expr) => {{
        println!("---\nProcessing: {:?}", $request);
        let response = $request.dispatch().await;
        println!("Got response:\n{response:?}\n---");
        response
    }};
}

async fn setup(
    database: Box<dyn crate::database::DBAccessor>,
    web_client: Box<dyn crate::web_client::WebClient>,
) -> Client {
    let config = rocket::Config {
        log_level: rocket::log::LogLevel::Debug,
        address: std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
        port: 8000,
        ..Default::default()
    };
    let rocket = crate::server::rocket(config, database, web_client).await;
    Client::tracked(rocket).await.unwrap()
}

#[tokio::test]
async fn test_mock_game_create() {
    let client = setup_mock().await;

    let user_id: UserId = 0;
    let response = request!(client.post("/games/0").json(&user_id));
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap().len(), 4);
}

#[tokio::test]
async fn test_e2e_game_create() {
    let client = setup_end_to_end().await;

    let user_id: UserId = 0;
    let response = request!(client.post("/games/0").json(&user_id));
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap().len(), 4);
}
