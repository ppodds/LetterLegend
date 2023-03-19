use crate::connection::Connection;
use crate::service::config_service::ConfigService;
use tokio::net::TcpListener;

pub struct Server {
    pub config_service: ConfigService,
    conntections: Vec<Connection>,
    host: String,
    port: u32,
}

pub struct Context {
    pub opcode: u8,
    pub payload: Vec<u8>,
}

impl Server {
    pub fn bind(
        &mut self,
        host: String,
        port: u32,
    ) -> Result<&mut Server, Box<dyn std::error::Error>> {
        self.host = host;
        self.port = port;
        Ok(self)
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            self.conntections.push(Connection::new(socket));

            // let config = self.config_service.config.clone();
            // let routers = self.routers.clone();

            // tokio::spawn(async move {
            //     let mut buf = [0; 1024];

            //     // In a loop, read data from the socket and write the data back.
            //     loop {
            //         match {
            //             let n = match socket.read(&mut buf).await {
            //                 // socket closed
            //                 Ok(n) if n == 0 => return,
            //                 Ok(n) => n,
            //                 Err(e) => {
            //                     eprintln!("failed to read from socket; err = {:?}", e);
            //                     return;
            //                 }
            //             };

            //             // check if the
            //             if n < 8 {
            //                 eprintln!("invalid message length");
            //                 continue;
            //             }

            //             // first byte (opcode)
            //             let opcode = buf[0];
            //             // 2~4 bytes (reserved)
            //             // 5~8 bytes (payload length)
            //             let payload_length = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
            //             if n != (payload_length + 8) as usize {
            //                 eprintln!("invalid message length");
            //                 continue;
            //             }

            //             // find according router
            //             if !routers.contains_key(&opcode) {
            //                 eprintln!("invalid opcode");
            //                 continue;
            //             };

            //             routers.get(&opcode).unwrap()(Context {
            //                 opcode,
            //                 payload: buf[8..n].to_vec(),
            //             })
            //         } {
            //             Ok(_) => continue,
            //             Err(e) => {
            //                 eprintln!("failed to handle the request; err = {:?}", e);
            //             }
            //         }
            //     }
            // });
        }
    }
    pub fn new(config_service: ConfigService) -> Self {
        Server {
            config_service,
            conntections: Vec::new(),
            host: String::from("localhost"),
            port: 45678,
        }
    }
}
