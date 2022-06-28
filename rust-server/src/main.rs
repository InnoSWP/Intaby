mod database;
mod server;
mod model;
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
    server::rocket(config, "postgresql://test:test@postgres_db:5432/test").await
}
