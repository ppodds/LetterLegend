use backend::{config::Config, server::Server, service::config_service::ConfigService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();
    let config = Config::load();
    let mut server = Server::new(ConfigService::new(config));
    server
        .bind(
            dotenvy::var("HOST").unwrap(),
            dotenvy::var("PORT").unwrap().parse::<u32>().unwrap(),
        )?
        .run()
        .await?;
    Ok(())
}
