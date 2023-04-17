use backend::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().unwrap();
    let server = Server::new(
        dotenvy::var("HOST").unwrap(),
        dotenvy::var("PORT").unwrap().parse::<u32>().unwrap(),
    );
    server.run().await?;
    Ok(())
}
