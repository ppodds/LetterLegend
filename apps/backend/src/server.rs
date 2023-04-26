use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

use crate::connection::Connection;
use crate::frame::{Frame, Request, Response};
use crate::lobby::lobbies::Lobbies;
use crate::lobby::lobby::Lobby;
use crate::model::control::{
    connect::ConnectResponse, disconnect::DisconnectResponse, heartbeat::HeartbeatResponse,
};
use crate::model::lobby::list::LobbyInfos;
use crate::player::Player;
use priority_queue::PriorityQueue;
use std::sync::Mutex;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct Server {
    player_timeout_queue: Arc<Mutex<PriorityQueue<u32, Reverse<Instant>>>>,
    host: String,
    port: u32,
    online_player_map: ClientMap,
    lobbies: Arc<Lobbies>,
    game_map: GameMap,
}

pub struct Context {
    pub opcode: u8,
    pub payload: Vec<u8>,
}

type ClientMap = Arc<Mutex<HashMap<u32, Arc<Player>>>>;
type GameMap = Arc<Mutex<HashMap<u32, Arc<Mutex<Vec<u32>>>>>>;

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
                match server.kick_timeout_users() {
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
                            let result = server
                                .handle_request(
                                    client_id,
                                    tx.clone(),
                                    #[cfg(not(test))]
                                    connection.clone(),
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

    #[cfg(not(test))]
    pub fn new(host: String, port: u32) -> Self {
        Server {
            player_timeout_queue: Arc::new(Mutex::new(PriorityQueue::new())),
            host,
            port,
            online_player_map: ClientMap::new(Mutex::new(HashMap::new())),
            lobbies: Arc::new(Lobbies::new()),
            game_map: GameMap::new(Mutex::new(HashMap::new())),
        }
    }

    #[cfg(test)]
    pub fn new() -> Self {
        Server {
            player_timeout_queue: Arc::new(Mutex::new(PriorityQueue::new())),
            host: String::from("0.0.0.0"),
            port: 45678,
            online_player_map: ClientMap::new(Mutex::new(HashMap::new())),
            lobbies: Arc::new(Lobbies::new()),
            game_map: GameMap::new(Mutex::new(HashMap::new())),
        }
    }

    fn connect(
        &self,
        client_id: u32,
        name: String,
        #[cfg(not(test))] connection: Arc<tokio::sync::Mutex<Connection>>,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        if self
            .online_player_map
            .lock()
            .unwrap()
            .contains_key(&client_id)
        {
            return Err("client already connected".into());
        }

        self.online_player_map.lock().unwrap().insert(
            client_id,
            Arc::new(Player::new(
                client_id,
                name,
                #[cfg(not(test))]
                connection,
            )),
        );

        self.player_timeout_queue
            .lock()
            .unwrap()
            .push(client_id, Reverse(Instant::now()));

        Ok(())
    }

    fn disconnect(&self, client_id: u32) -> Result<(), Box<dyn Error + Sync + Send>> {
        match self.online_player_map.lock().unwrap().remove(&client_id) {
            Some(player) => {
                self.player_timeout_queue.lock().unwrap().remove(&client_id);
                if player.lobby_id.lock().unwrap().is_some() {
                    let lobby = self
                        .lobbies
                        .clone()
                        .get_lobby(player.lobby_id.lock().unwrap().unwrap())
                        .unwrap();
                    lobby.remove_player(player.id);
                }
                if player.game_id.lock().unwrap().is_some() {
                    let game = self.game_map.lock().unwrap()
                        [&player.game_id.lock().unwrap().unwrap()]
                        .clone();
                    game.lock().unwrap().retain(|&x| x != client_id);
                }
                Ok(())
            }
            None => Err("Player not found")?,
        }
    }

    fn heartbeat(&self, client_id: u32) -> Result<(), Box<dyn Error + Sync + Send>> {
        match self
            .player_timeout_queue
            .lock()
            .unwrap()
            .change_priority(&client_id, Reverse(Instant::now()))
        {
            Some(_) => Ok(()),
            None => Err("Player not found")?,
        }
    }

    fn create_lobby(
        &self,
        client_id: u32,
        max_players: u32,
    ) -> Result<Arc<Lobby>, Box<dyn Error + Sync + Send>> {
        if max_players < 4 || max_players > 8 {
            return Err("Invalid max players".into());
        }

        let lobby = self.lobbies.create_lobby(max_players);
        let players = self.online_player_map.lock().unwrap();
        let player = players.get(&client_id);
        if player.is_none() {
            return Err("Player not found".into());
        }
        match lobby.clone().add_player(player.unwrap().clone()) {
            Ok(_) => Ok(lobby),
            Err(e) => Err(e),
        }
    }

    fn join_lobby(
        &self,
        client_id: u32,
        lobby_id: u32,
    ) -> Result<Arc<Lobby>, Box<dyn Error + Sync + Send>> {
        let lobby = self.lobbies.clone().get_lobby(lobby_id);

        if lobby.is_none() {
            return Err("Lobby not found".into());
        }

        let players = self.online_player_map.lock().unwrap();
        let player = players.get(&client_id);

        if player.is_none() {
            return Err("Player not found".into());
        }

        let lobby = lobby.unwrap().clone();
        match lobby.clone().add_player(player.unwrap().clone()) {
            Ok(_) => Ok(lobby),
            Err(e) => return Err(e),
        }
    }

    fn quit_lobby(&self, client_id: u32) -> Result<(), Box<dyn Error + Sync + Send>> {
        let players = self.online_player_map.lock().unwrap();
        let player = players.get(&client_id);

        if player.is_none() {
            return Err("Player not found".into());
        }

        let lobby = {
            let lobby_id = player.unwrap().lobby_id.lock().unwrap();

            if lobby_id.is_none() {
                return Err("Player not in lobby".into());
            }
            self.lobbies.clone().get_lobby(lobby_id.unwrap())
        };

        if lobby.is_none() {
            return Err("Lobby not found".into());
        }

        lobby.unwrap().remove_player(client_id);
        Ok(())
    }

    fn list_lobby(&self) -> Result<LobbyInfos, Box<dyn Error + Sync + Send>> {
        Ok(LobbyInfos::from_lobbies(self.lobbies.clone()))
    }

    fn kick_timeout_users(&self) -> Result<u32, Box<dyn Error + Sync + Send>> {
        let queue = self.player_timeout_queue.clone();
        let mut timeout_players = Vec::new();
        {
            let mut queue = queue.lock().unwrap();
            while let Some(item) = queue.pop() {
                if item.1 .0.elapsed().as_secs() < 30 {
                    queue.push(item.0, item.1);
                    break;
                }
                timeout_players.push(item.0);
            }
        }
        for timeout_player in &timeout_players {
            self.disconnect(*timeout_player)?;
        }
        Ok(timeout_players.len() as u32)
    }

    fn ready(&self, client_id: u32) -> Result<bool, Box<dyn Error + Sync + Send>> {
        let player = match self.online_player_map.lock().unwrap().get(&client_id) {
            Some(player) => player.clone(),
            None => return Err("Player not found".into()),
        };

        let lobby_id = match *player.lobby_id.lock().unwrap() {
            Some(id) => id,
            None => return Err("Player not in lobby".into()),
        };

        let lobby = match self.lobbies.get_lobby(lobby_id) {
            Some(lobby) => lobby.clone(),
            None => return Err("Lobby not found".into()),
        };

        let player = lobby.get_player(client_id);

        match player {
            Some(player) => {
                let mut ready = player.ready.lock().unwrap();
                *ready = !*ready;
                return Ok(*ready);
            }
            None => return Err("Player not found".into()),
        }
    }

    async fn handle_request(
        &self,
        client_id: u32,
        tx: Sender<Frame>,
        #[cfg(not(test))] connection: Arc<tokio::sync::Mutex<Connection>>,
        request: Request,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        match request {
            Request::Connect(req) => match self.connect(
                client_id,
                req.name,
                #[cfg(not(test))]
                connection,
            ) {
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
            Request::Disconnect => match self.disconnect(client_id) {
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
            Request::Heartbeat => match self.heartbeat(client_id) {
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
            Request::CreateLobby(req) => match self.create_lobby(client_id, req.max_players) {
                Ok(lobby) => {
                    tx.send(Frame::Response(Response::CreateLobby(
                        crate::model::lobby::create::CreateResponse {
                            success: true,
                            lobby: Some(crate::model::lobby::lobby::Lobby::from_lobby(lobby)),
                        },
                    )))
                    .await?;
                    Ok(())
                }
                Err(e) => {
                    tx.send(Frame::Response(Response::CreateLobby(
                        crate::model::lobby::create::CreateResponse {
                            success: false,
                            lobby: None,
                        },
                    )))
                    .await?;
                    Err(e)
                }
            },
            Request::JoinLobby(req) => match self.join_lobby(client_id, req.lobby_id) {
                Ok(res) => {
                    tx.send(Frame::Response(Response::JoinLobby(
                        crate::model::lobby::join::JoinResponse {
                            success: true,
                            lobby: Some(crate::model::lobby::lobby::Lobby::from_lobby(res)),
                        },
                    )))
                    .await?;
                    Ok(())
                }
                Err(e) => {
                    tx.send(Frame::Response(Response::JoinLobby(
                        crate::model::lobby::join::JoinResponse {
                            success: false,
                            lobby: None,
                        },
                    )))
                    .await?;
                    Err(e)
                }
            },
            Request::QuitLobby => match self.quit_lobby(client_id) {
                Ok(_) => {
                    tx.send(Frame::Response(Response::QuitLobby(
                        crate::model::lobby::quit::QuitResponse { success: true },
                    )))
                    .await?;
                    Ok(())
                }
                Err(e) => {
                    tx.send(Frame::Response(Response::QuitLobby(
                        crate::model::lobby::quit::QuitResponse { success: false },
                    )))
                    .await?;
                    Err(e)
                }
            },
            Request::ListLobby => match self.list_lobby() {
                Ok(res) => {
                    tx.send(Frame::Response(Response::ListLobby(
                        crate::model::lobby::list::ListResponse {
                            success: true,
                            lobby_infos: Some(res),
                        },
                    )))
                    .await?;
                    Ok(())
                }
                Err(e) => {
                    tx.send(Frame::Response(Response::ListLobby(
                        crate::model::lobby::list::ListResponse {
                            success: false,
                            lobby_infos: None,
                        },
                    )))
                    .await?;
                    Err(e)
                }
            },
            Request::Ready => match self.ready(client_id) {
                Ok(_) => {
                    tx.send(Frame::Response(Response::Ready(
                        crate::model::lobby::ready::ReadyResponse { success: true },
                    )))
                    .await?;
                    Ok(())
                }
                Err(e) => {
                    tx.send(Frame::Response(Response::Ready(
                        crate::model::lobby::ready::ReadyResponse { success: false },
                    )))
                    .await?;
                    Err(e)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_with_test_user_online_player_map_should_include_test_user(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server.connect(0, String::from("test"))?;
        let online_player_map = server.online_player_map.lock().unwrap();
        let player = online_player_map.get(&0).unwrap();
        assert_eq!(player.id, 0);
        assert_eq!(player.name, String::from("test"));
        Ok(())
    }

    #[test]
    fn connect_with_test_user_player_timeout_queue_should_include_test_user(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server.connect(0, String::from("test"))?;
        let player_timeout_queue = server.player_timeout_queue.lock().unwrap();
        assert!(player_timeout_queue.get(&0).is_some());
        Ok(())
    }

    #[test]
    fn connect_with_test_user_who_already_connected_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server.connect(0, String::from("test"))?;
        assert!(server.connect(0, String::from("test")).is_err());
        Ok(())
    }

    #[test]
    fn disconnect_with_user_already_connected_should_be_removed(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        server
            .player_timeout_queue
            .lock()
            .unwrap()
            .push(0, Reverse(Instant::now()));
        server.disconnect(0)?;
        assert!(server.online_player_map.lock().unwrap().len() == 0);
        assert!(server.player_timeout_queue.lock().unwrap().len() == 0);
        Ok(())
    }

    #[test]
    fn disconnect_with_user_not_exist_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        assert!(server.disconnect(0).is_err());
        Ok(())
    }

    #[test]
    fn create_lobby_with_test_user_should_create_lobby() -> Result<(), Box<dyn Error + Sync + Send>>
    {
        let server = Server::new();
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        server.create_lobby(0, 4)?;
        assert!(server.lobbies.get_lobby(0).is_some());
        Ok(())
    }

    #[test]
    fn create_lobby_with_test_user_and_invaild_max_players_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        assert!(server.create_lobby(0, 3).is_err());
        assert!(server.create_lobby(0, 9).is_err());
        Ok(())
    }

    #[test]
    fn create_lobby_with_not_exist_user_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        assert!(server.create_lobby(0, 4).is_err());
        Ok(())
    }

    #[test]
    fn create_lobby_with_test_user_should_contains_test_user(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        server.create_lobby(0, 4)?;
        assert!(server.lobbies.get_lobby(0).unwrap().get_player(0).is_some());
        Ok(())
    }

    #[test]
    fn join_lobby_with_test_user_and_test_lobby_should_join_lobby(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        server.lobbies.create_lobby(4);
        server.join_lobby(0, 0)?;
        assert!(server.lobbies.get_lobby(0).unwrap().get_player(0).is_some());
        Ok(())
    }

    #[test]
    fn join_lobby_with_not_exist_user_and_test_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        server.create_lobby(0, 4)?;
        assert!(server.join_lobby(1, 0).is_err());
        Ok(())
    }

    #[test]
    fn join_lobby_with_test_user_and_not_exist_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        assert!(server.join_lobby(0, 0).is_err());
        Ok(())
    }

    #[test]
    fn join_lobby_with_not_exist_user_and_not_exist_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        assert!(server.join_lobby(0, 0).is_err());
        Ok(())
    }

    #[test]
    fn quit_lobby_with_test_user_in_test_lobby_should_quit_lobby(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, player.clone());
        let lobby = server.lobbies.create_lobby(4);
        lobby.add_player(player)?;
        server.quit_lobby(0)?;
        assert!(server.lobbies.get_lobby(0).unwrap().get_player(0).is_none());
        Ok(())
    }

    #[test]
    fn quit_lobby_with_not_exist_user_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        assert!(server.quit_lobby(0).is_err());
        Ok(())
    }

    #[test]
    fn quit_lobby_with_test_user_but_not_in_lobby_should_return_error(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, player.clone());
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, player.clone());
        assert!(server.quit_lobby(0).is_err());
        Ok(())
    }

    #[test]
    fn list_lobby_with_test_looby_should_return_test_lobby(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server.lobbies.create_lobby(4);
        let lobby_list = server.list_lobby()?;
        assert_eq!(lobby_list.lobby_infos.len(), 1);
        Ok(())
    }

    #[test]
    fn kick_timeout_users_with_a_timeout_user_timeout_users_should_be_kicked(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        server
            .player_timeout_queue
            .lock()
            .unwrap()
            .push(0, Reverse(Instant::now() - Duration::from_secs(60)));
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, Arc::new(Player::new(0, String::from("test"))));
        assert_eq!(server.kick_timeout_users()?, 1);
        Ok(())
    }

    #[test]
    fn kick_timeout_users_with_two_timeout_users_and_a_normal_user_timeout_users_should_be_kicked(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();

        for i in 0..3 {
            server.player_timeout_queue.lock().unwrap().push(
                i,
                match i == 2 {
                    true => Reverse(Instant::now()),
                    false => Reverse(Instant::now() - Duration::from_secs(60)),
                },
            );
            server
                .online_player_map
                .lock()
                .unwrap()
                .insert(i, Arc::new(Player::new(i, String::from("test"))));
        }

        assert_eq!(server.kick_timeout_users()?, 2);
        Ok(())
    }

    #[test]
    fn ready_with_test_user_in_test_lobby_should_ready() -> Result<(), Box<dyn Error + Sync + Send>>
    {
        let server = Server::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, player.clone());
        let lobby = server.lobbies.create_lobby(4);
        lobby.add_player(player.clone())?;
        server.ready(0)?;
        assert!(*lobby.get_player(0).unwrap().ready.lock().unwrap());
        Ok(())
    }

    #[test]
    fn ready_with_test_user_in_test_lobby_should_not_ready(
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let server = Server::new();
        let player = Arc::new(Player::new(0, String::from("test")));
        server
            .online_player_map
            .lock()
            .unwrap()
            .insert(0, player.clone());
        let lobby = server.lobbies.create_lobby(4);
        lobby.add_player(player.clone())?;
        *lobby.get_player(0).unwrap().ready.lock().unwrap() = true;
        server.ready(0)?;
        assert!(!*lobby.get_player(0).unwrap().ready.lock().unwrap());
        Ok(())
    }
}
