use std::error::Error;

#[cfg(not(test))]
use backend::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().unwrap();
    #[cfg(test)]
    panic!("Application should not be run in test mode");
    #[cfg(not(test))]
    {
        let server = Server::new(
            dotenvy::var("HOST").unwrap(),
            dotenvy::var("PORT").unwrap().parse::<u32>().unwrap(),
        );
        server.run().await?;
        Ok(())
    }
}
