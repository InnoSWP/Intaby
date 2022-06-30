use rocket::tokio;
use rocket::{http::Status, local::asynchronous::Client};

use crate::model::*;

macro_rules! request {
    ($request: expr) => {{
        println!("---\nProcessing: {:?}", $request);
        let response = $request.dispatch().await;
        println!("Got response:\n{response:?}\n---");
        response
    }};
}

async fn setup(web_client: Box<dyn crate::web_client::WebClient>) -> Client {
    let config = rocket::Config {
        log_level: rocket::log::LogLevel::Debug,
        address: std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
        port: 8000,
        ..Default::default()
    };
    let rocket = crate::server::rocket(config, web_client).await;
    Client::tracked(rocket).await.unwrap()
}

async fn setup_end_to_end() -> Client {
    let web_client = Box::new(crate::web_client::reqwest_client::ReqwestClient::new());
    setup(web_client).await
}

async fn setup_mock() -> Client {
    let web_client = Box::new(crate::web_client::mock::MockWebClient::new());
    setup(web_client).await
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
