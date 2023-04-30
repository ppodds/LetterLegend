use std::sync::Arc;

use crate::connection::Connection;
use crate::controller::control::connect::ConnectController;
use crate::frame::Frame;
use crate::router::{RequestContext, Router};
use crate::service::lobby_service::LobbyService;
use crate::service::player_service::PlayerService;
use crate::{
    controller::{
        control::{disconnect::DisconnectController, heartbeat::HeartbeatController},
        game::start::StartController,
        lobby::{
            create::CreateController, join::JoinController, list::ListController,
            quit::QuitController, ready::ReadyController,
        },
    },
    operation::Operation,
    service::game_service,
};
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{sleep, Duration};
#[derive(Debug, Clone)]
pub struct Server {
    host: String,
    port: u32,
    player_service: Arc<PlayerService>,
    // lobby_service: Arc<LobbyService>,
    // game_service: Arc<GameService>,
    router: Arc<Router>,
}

pub struct Context {
    pub opcode: u8,
    pub payload: Vec<u8>,
}

unsafe impl Send for Server {}
unsafe impl Sync for Server {}

impl Server {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        let mut next_client_id = 0;

        let server = self.clone();

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(60)).await;
                match server.player_service.kick_timeout_users() {
                    Ok(_) => println!("kick timeout players success"),
                    Err(e) => eprintln!("failed to kick timeout players, err: {}", e),
                }
            }
        });

        loop {
            let (socket, _) = listener.accept().await?;
            let (tx, mut rx): (Sender<Frame>, Receiver<Frame>) = channel(128);

            let client_id = next_client_id;
            next_client_id += 1;

            let connection_bak = Arc::new(tokio::sync::Mutex::new(Connection::new(socket)));
            // clone the map
            let connection = connection_bak.clone();
            let server = self.clone();

            tokio::spawn(async move {
                loop {
                    let frame = match connection.lock().await.try_read_frame() {
                        Ok(Some(frame)) => frame,
                        Ok(None) => {
                            continue;
                        }
                        Err(e) => {
                            eprintln!("failed to read frame; err = {:?}", e);
                            break;
                        }
                    };
                    match frame {
                        Frame::Request(req) => {
                            match server.router.route(
                                req,
                                RequestContext {
                                    client_id,
                                    #[cfg(not(test))]
                                    connection: connection.clone(),
                                },
                            ) {
                                Ok(res) => {
                                    if tx.send(Frame::Response(res)).await.is_err() {
                                        eprintln!("failed to send frame");
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("failed to handle request; err = {:?}", e);
                                }
                            };
                        }
                        _ => {
                            eprintln!("invalid frame; frame = {:?}", frame)
                        }
                    };
                }
            });

            let connection = connection_bak.clone();

            tokio::spawn(async move {
                loop {
                    while let Some(frame) = rx.recv().await {
                        println!("received frame; frame = {:?}", frame);
                        match connection.lock().await.write_frame(&frame).await {
                            Ok(_) => {
                                println!("sent frame; frame = {:?}", frame);
                                continue;
                            }
                            Err(e) => {
                                eprintln!("failed to write frame; err = {:?}", e);
                                break;
                            }
                        };
                    }
                }
            });
        }
    }

    pub fn new() -> Self {
        let player_service = Arc::new(PlayerService::new());
        let lobby_service = Arc::new(LobbyService::new());
        let game_service = Arc::new(game_service::GameService::new());
        let router = Arc::new(Router::new());
        router
            .register_controller(
                Operation::Connect,
                Box::new(ConnectController::new(player_service.clone())),
            )
            .register_controller(
                Operation::Disconnect,
                Box::new(DisconnectController::new(player_service.clone())),
            )
            .register_controller(
                Operation::Heartbeat,
                Box::new(HeartbeatController::new(player_service.clone())),
            )
            .register_controller(
                Operation::CreateLobby,
                Box::new(CreateController::new(
                    player_service.clone(),
                    lobby_service.clone(),
                )),
            )
            .register_controller(
                Operation::JoinLobby,
                Box::new(JoinController::new(
                    player_service.clone(),
                    lobby_service.clone(),
                )),
            )
            .register_controller(
                Operation::ListLobby,
                Box::new(ListController::new(lobby_service.clone())),
            )
            .register_controller(
                Operation::QuitLobby,
                Box::new(QuitController::new(
                    player_service.clone(),
                    lobby_service.clone(),
                )),
            )
            .register_controller(
                Operation::Ready,
                Box::new(ReadyController::new(player_service.clone())),
            )
            .register_controller(
                Operation::StartGame,
                Box::new(StartController::new(
                    player_service.clone(),
                    game_service.clone(),
                )),
            );
        Server {
            host: String::from("0.0.0.0"),
            port: 45678,
            player_service,
            // lobby_service,
            // game_service,
            router,
        }
    }
}
