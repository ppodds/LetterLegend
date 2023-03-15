use crate::service::config_service::ConfigService;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct Server {
    pub config_service: ConfigService,
}

impl Server {
    async fn listen(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = &self.config_service.config;
        let listener = TcpListener::bind(format!("{}:{}", config.ip, config.port)).await?;
        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    }
}

pub fn create_server() -> Server {
    Server {
        config_service: ConfigService::new(),
    }
}
