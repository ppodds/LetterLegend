use std::error::Error;

use backend::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().unwrap();
    let server = Server::new();
    server.run().await?;
    Ok(())
}
