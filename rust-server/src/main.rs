mod database;
mod game;
mod server;

// use database::{psql::*, DBAccessor};

#[rocket::launch]
fn launch() -> _ {
    let config = rocket::Config {
        log_level: rocket::log::LogLevel::Debug,
        address: std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
        port: 8000,
        ..Default::default()
    };
    server::rocket(config)
}
