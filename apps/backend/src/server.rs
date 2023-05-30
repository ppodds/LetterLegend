use std::sync::Arc;

use crate::connection::Connection;
use crate::controller::control::connect::ConnectController;
use crate::controller::game::get_new_card::GetNewCardController;
use crate::controller::game::set_tile::SetTileController;
use crate::frame::{Frame, Response};
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
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Server {
    host: String,
    port: u32,
    player_service: Arc<PlayerService>,
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

        loop {
            let (socket, _) = listener.accept().await?;
            let (tx, rx): (Sender<Frame>, Receiver<Frame>) = channel(128);
            let shared_rx_bak = Arc::new(Mutex::new(rx));
            let client_id = next_client_id;
            next_client_id += 1;

            let connection_bak = Arc::new(Connection::new(socket));
            // clone the map
            let connection = connection_bak.clone();
            let server = self.clone();
            let shared_rx = shared_rx_bak.clone();

            tokio::spawn(async move {
                loop {
                    let frame = match connection.read_frame().await {
                        Ok(Some(frame)) => frame,
                        Ok(None) => {
                            continue;
                        }
                        Err(e) => {
                            eprintln!("failed to read frame; err = {:?}", e);
                            if let Some(player) = server.player_service.get_player(client_id) {
                                match server.player_service.remove_player(player) {
                                    Ok(player) => println!(
                                        "clean up player's resource success. player id: {}, player name: {}", player.id, player.name
                                    ),
                                    Err(e) => {
                                        eprintln!("failed to clean up player's resource, err: {e}")
                                    }
                                }
                            };
                            shared_rx.lock().await.close();
                            break;
                        }
                    };
                    match frame {
                        Frame::Request(req) => {
                            let state = req.get_state();
                            match server.router.route(
                                req,
                                RequestContext {
                                    client_id,
                                    #[cfg(not(test))]
                                    sender: tx.clone(),
                                },
                            ) {
                                Ok(res) => {
                                    if tx
                                        .send(Frame::Response(Response::new(state, Arc::new(res))))
                                        .await
                                        .is_err()
                                    {
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
            let shared_rx = shared_rx_bak.clone();

            tokio::spawn(async move {
                loop {
                    while let Some(frame) = shared_rx.lock().await.recv().await {
                        println!("received frame; frame = {:?}", frame);
                        match connection.write_frame(&frame).await {
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
        let lobby_service = Arc::new(LobbyService::new());
        let game_service = Arc::new(game_service::GameService::new());
        let player_service = Arc::new(PlayerService::new(
            lobby_service.clone(),
            game_service.clone(),
        ));
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
            .register_controller(Operation::Heartbeat, Box::new(HeartbeatController::new()))
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
            )
            .register_controller(
                Operation::SetTile,
                Box::new(SetTileController::new(
                    player_service.clone(),
                    game_service.clone(),
                )),
            )
            .register_controller(
                Operation::GetNewCard,
                Box::new(GetNewCardController::new(
                    player_service.clone(),
                    game_service.clone(),
                )),
            );
        Server {
            host: String::from("0.0.0.0"),
            port: 45678,
            player_service,
            router,
        }
    }
}
