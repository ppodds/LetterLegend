use std::error::Error;

use backend::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Loading environment variables");
    dotenvy::dotenv().unwrap();
    println!("Building server");
    let server = Server::new().await?;
    println!("Start server");
    server.run().await?;
    Ok(())
}
