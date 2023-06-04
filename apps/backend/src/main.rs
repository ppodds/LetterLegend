use std::error::Error;

use backend::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().unwrap();
    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}
