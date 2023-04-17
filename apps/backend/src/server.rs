use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

use crate::connection::Connection;
use crate::frame::{Frame, Request, Response};
use crate::model::control::{
    connect::ConnectResponse, disconnect::DisconnectResponse, heartbeat::HeartbeatResponse,
};
use crate::player::Player;
use priority_queue::PriorityQueue;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Server {
    player_timeout_queue: Arc<Mutex<PriorityQueue<u32, Instant>>>,
    host: String,
    port: u32,
    online_player_map: ClientMap,
    lobby_map: LobbyMap,
    game_map: GameMap,
}

pub struct Context {
    pub opcode: u8,
    pub payload: Vec<u8>,
}

type ClientMap = Arc<Mutex<HashMap<u32, Arc<Mutex<Player>>>>>;
type LobbyMap = Arc<Mutex<HashMap<u32, Arc<Mutex<Vec<u32>>>>>>;
type GameMap = Arc<Mutex<HashMap<u32, Arc<Mutex<Vec<u32>>>>>>;

unsafe impl Send for Server {}
unsafe impl Sync for Server {}

impl Server {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        let mut next_client_id = 0;

        loop {
            let (socket, _) = listener.accept().await?;
            let (tx, mut rx): (Sender<Frame>, Receiver<Frame>) = channel(128);

            let client_id = next_client_id;
            next_client_id += 1;

            // clone the map
            let connection = Arc::new(Mutex::new(Connection::new(socket)));
            let connection_receiver = connection.clone();
            let connection_sender = connection.clone();
            let server = self.clone();

            tokio::spawn(async move {
                loop {
                    let frame = match connection_receiver.lock().await.try_read_frame() {
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
                            let result = server
                                .handle_request(
                                    client_id,
                                    tx.clone(),
                                    connection_receiver.clone(),
                                    req,
                                )
                                .await;
                            if result.is_err() {
                                eprintln!("failed to handle request; err = {:?}", result);
                            }
                        }
                        Frame::Error(_) | Frame::Response(_) => {
                            eprintln!("invalid frame; frame = {:?}", frame)
                        }
                    };
                }
            });

            tokio::spawn(async move {
                loop {
                    while let Some(frame) = rx.recv().await {
                        println!("received frame; frame = {:?}", frame);
                        let mut connection = connection_sender.lock().await;
                        // println!("get connection = {:?}", connection);
                        match connection.write_frame(&frame).await {
                            Ok(_) => {
                                println!("sent frame; frame = {:?}", frame);
                                continue;
                            }
                            Err(e) => {
                                eprintln!("failed to write frame; err = {:?}", e);
                                break;
                            }
                        }
                    }
                }
            });
        }
    }

    pub fn new(host: String, port: u32) -> Self {
        Server {
            player_timeout_queue: Arc::new(Mutex::new(PriorityQueue::new())),
            host,
            port,
            online_player_map: ClientMap::new(Mutex::new(HashMap::new())),
            lobby_map: LobbyMap::new(Mutex::new(HashMap::new())),
            game_map: GameMap::new(Mutex::new(HashMap::new())),
        }
    }

    async fn connect(
        &self,
        client_id: u32,
        name: String,
        connection: Arc<Mutex<Connection>>,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        if self.online_player_map.lock().await.contains_key(&client_id) {
            return Err("client already connected".into());
        }

        self.online_player_map.lock().await.insert(
            client_id,
            Arc::new(Mutex::new(Player::new(client_id, name, connection))),
        );
        self.player_timeout_queue
            .lock()
            .await
            .push(client_id, Instant::now());

        Ok(())
    }

    async fn disconnect(&self, client_id: u32) -> Result<(), Box<dyn Error + Sync + Send>> {
        match self.online_player_map.lock().await.remove(&client_id) {
            Some(player) => {
                let player = player.lock().await;
                self.player_timeout_queue.lock().await.remove(&client_id);
                if player.lobby_id.is_some() {
                    let lobby = self.lobby_map.lock().await[&player.lobby_id.unwrap()].clone();
                    lobby.lock().await.retain(|&x| x != client_id);
                }
                if player.game_id.is_some() {
                    let game = self.game_map.lock().await[&player.game_id.unwrap()].clone();
                    game.lock().await.retain(|&x| x != client_id);
                }
                Ok(())
            }
            None => Err("Player not found")?,
        }
    }

    async fn heartbeat(&self, client_id: u32) -> Result<(), Box<dyn Error + Sync + Send>> {
        match self
            .player_timeout_queue
            .lock()
            .await
            .change_priority(&client_id, Instant::now())
        {
            Some(_) => Ok(()),
            None => Err("Player not found")?,
        }
    }

    async fn handle_request(
        &self,
        client_id: u32,
        tx: Sender<Frame>,
        connection: Arc<Mutex<Connection>>,
        request: Request,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        match request {
            Request::Connect(req) => match self.connect(client_id, req.name, connection).await {
                Ok(_) => {
                    tx.send(Frame::Response(Response::Connect(ConnectResponse {
                        success: true,
                    })))
                    .await?;
                    Ok(())
                }
                Err(e) => {
                    tx.send(Frame::Response(Response::Connect(ConnectResponse {
                        success: false,
                    })))
                    .await?;
                    Err(e)
                }
            },
            Request::Disconnect => match self.disconnect(client_id).await {
                Ok(_) => {
                    tx.send(Frame::Response(Response::Disconnect(DisconnectResponse {
                        success: true,
                    })))
                    .await?;
                    Ok(())
                }
                Err(e) => {
                    tx.send(Frame::Response(Response::Disconnect(DisconnectResponse {
                        success: false,
                    })))
                    .await?;
                    Err(e)
                }
            },
            Request::Heartbeat => match self.heartbeat(client_id).await {
                Ok(_) => {
                    tx.send(Frame::Response(Response::Heartbeat(HeartbeatResponse {
                        success: true,
                    })))
                    .await?;
                    Ok(())
                }
                Err(e) => {
                    tx.send(Frame::Response(Response::Heartbeat(HeartbeatResponse {
                        success: false,
                    })))
                    .await?;
                    Err(e)
                }
            },
        }
    }
}
