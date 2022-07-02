mod model;
mod server;
mod web_client;

#[cfg(test)]
mod tests;

#[derive(clap::Parser)]
struct Args {
    #[clap(long)]
    mock: bool,
}

#[rocket::launch]
async fn launch() -> _ {
    let args = <Args as clap::Parser>::parse();

    let config = rocket::Config {
        log_level: rocket::log::LogLevel::Debug,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        port: 8080,
        ..Default::default()
    };
    let web_client = if args.mock {
        Box::new(web_client::mock::MockWebClient::new()) as Box<dyn web_client::WebClient>
    } else {
        Box::new(web_client::reqwest_client::ReqwestClient::new())
    };
    server::rocket(config, web_client).await
}
