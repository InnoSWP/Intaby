mod database;
mod model;
mod server;
mod web_client;

#[cfg(test)]
mod tests;

#[rocket::launch]
async fn launch() -> _ {
    let config = rocket::Config {
        log_level: rocket::log::LogLevel::Debug,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        port: 8080,
        ..Default::default()
    };
    let database = Box::new(
        database::sql::SqlAccess::new("postgresql://test:test@postgres_db:5432/test")
            .await
            .expect("Failed to access the database"),
    );
    let web_client = Box::new(web_client::reqwest_client::ReqwestClient::new());
    server::rocket(config, database, web_client).await
}
