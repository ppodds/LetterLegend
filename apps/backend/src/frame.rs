use std::{io::Cursor, sync::Arc};

use bytes::Buf;
use prost::Message;

use crate::{
    model::control::connect::ConnectRequest,
    model::control::connect::ConnectResponse,
    model::control::disconnect::DisconnectResponse,
    model::control::heartbeat::HeartbeatResponse,
    model::game::broadcast::GameBroadcast,
    model::game::exit::ExitResponse,
    model::game::finish_turn::FinishTurnResponse,
    model::game::get_new_card::GetNewCardResponse,
    model::game::set_tile::SetTileRequest,
    model::game::set_tile::SetTileResponse,
    model::game::{
        cancel::{CancelRequest, CancelResponse},
        start::StartResponse,
    },
    model::lobby::broadcast::LobbyBroadcast,
    model::lobby::create::CreateRequest,
    model::lobby::create::CreateResponse,
    model::lobby::join::JoinRequest,
    model::lobby::join::JoinResponse,
    model::lobby::list::ListResponse,
    model::lobby::quit::QuitResponse,
    model::lobby::ready::ReadyResponse,
    operation::Operation,
};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum Frame {
    Request(Request),
    Response(Response),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    state: u32,
    data: Arc<RequestData>,
}

impl Request {
    pub fn new(state: u32, data: Arc<RequestData>) -> Self {
        Self { state, data }
    }

    pub fn get_state(&self) -> u32 {
        self.state
    }

    pub fn get_data(&self) -> Arc<RequestData> {
        self.data.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestData {
    Connect(ConnectRequest),
    Disconnect,
    Heartbeat,
    CreateLobby(CreateRequest),
    JoinLobby(JoinRequest),
    QuitLobby,
    ListLobby,
    Ready,
    StartGame,
    SetTile(SetTileRequest),
    FinishTurn,
    GetNewCard,
    Cancel(CancelRequest),
    Exit,
}

impl Hash for RequestData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RequestData::Connect(_) => 0.hash(state),
            RequestData::Disconnect => 1.hash(state),
            RequestData::Heartbeat => 2.hash(state),
            RequestData::CreateLobby(_) => 3.hash(state),
            RequestData::JoinLobby(_) => 4.hash(state),
            RequestData::QuitLobby => 5.hash(state),
            RequestData::ListLobby => 6.hash(state),
            RequestData::Ready => 7.hash(state),
            RequestData::StartGame => 8.hash(state),
            RequestData::SetTile(_) => 9.hash(state),
            RequestData::FinishTurn => 10.hash(state),
            RequestData::GetNewCard => 11.hash(state),
            RequestData::Cancel(_) => 12.hash(state),
            RequestData::Exit => 13.hash(state),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    state: u32,
    data: Arc<ResponseData>,
}

impl Response {
    pub fn new(state: u32, data: Arc<ResponseData>) -> Self {
        Self { state, data }
    }

    pub fn get_state(&self) -> u32 {
        self.state
    }

    pub fn get_data(&self) -> Arc<ResponseData> {
        self.data.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResponseData {
    Error(crate::model::error::error::Error),
    Connect(ConnectResponse),
    Disconnect(DisconnectResponse),
    Cancel(CancelResponse),
    Heartbeat(HeartbeatResponse),
    CreateLobby(CreateResponse),
    JoinLobby(JoinResponse),
    QuitLobby(QuitResponse),
    ListLobby(ListResponse),
    Ready(ReadyResponse),
    StartGame(StartResponse),
    LobbyBroadcast(LobbyBroadcast),
    SetTile(SetTileResponse),
    FinishTurn(FinishTurnResponse),
    GetNewCard(GetNewCardResponse),
    GameBroadcast(GameBroadcast),
    Exit(ExitResponse),
}

#[derive(Debug)]
pub enum Error {
    Incomplete,
    ProtobufDecodeFailed(prost::DecodeError),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl Frame {
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        let op = match Operation::try_from(get_u8(src)?) {
            Ok(op) => op,
            Err(e) => {
                return Err(Error::Other(e));
            }
        };
        src.set_position(8);
        let payload_len = get_u32(src)?;
        let payload = src.take(payload_len as usize);
        let e = match op {
            Operation::Connect => ConnectRequest::decode(payload).err(),
            Operation::Disconnect => return Ok(()),
            Operation::Heartbeat => return Ok(()),
            Operation::CreateLobby => CreateRequest::decode(payload).err(),
            Operation::JoinLobby => JoinRequest::decode(payload).err(),
            Operation::QuitLobby => return Ok(()),
            Operation::ListLobby => return Ok(()),
            Operation::Ready => return Ok(()),
            Operation::StartGame => return Ok(()),
            Operation::SetTile => SetTileRequest::decode(payload).err(),
            Operation::FinishTurn => return Ok(()),
            Operation::Exit => return Ok(()),
            Operation::GetNewCard => return Ok(()),
            Operation::Cancel => CancelRequest::decode(payload).err(),
        };
        if e.is_some() {
            return Err(Error::ProtobufDecodeFailed(e.unwrap()));
        }
        Ok(())
    }

    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        let op = match Operation::try_from(get_u8(src)?) {
            Ok(op) => op,
            Err(e) => {
                return Err(Error::Other(e));
            }
        };
        src.set_position(4);
        let state = get_u32(src)?;
        let payload_len = get_u32(src)?;
        let payload = src.take(payload_len as usize);
        match op {
            Operation::Connect => match ConnectRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request {
                    state,
                    data: Arc::new(RequestData::Connect(req)),
                })),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::Cancel => match CancelRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request {
                    state,
                    data: Arc::new(RequestData::Cancel(req)),
                })),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::Disconnect => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::Disconnect),
            })),
            Operation::Heartbeat => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::Heartbeat),
            })),
            Operation::CreateLobby => match CreateRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request {
                    state,
                    data: Arc::new(RequestData::CreateLobby(req)),
                })),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::JoinLobby => match JoinRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request {
                    state,
                    data: Arc::new(RequestData::JoinLobby(req)),
                })),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::QuitLobby => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::QuitLobby),
            })),
            Operation::ListLobby => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::ListLobby),
            })),
            Operation::Ready => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::Ready),
            })),
            Operation::StartGame => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::StartGame),
            })),
            Operation::SetTile => match SetTileRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request {
                    state,
                    data: Arc::new(RequestData::SetTile(req)),
                })),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::FinishTurn => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::FinishTurn),
            })),
            Operation::GetNewCard => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::GetNewCard),
            })),
            Operation::Exit => Ok(Frame::Request(Request {
                state,
                data: Arc::new(RequestData::Exit),
            })),
        }
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.get_u8())
}

fn get_u32(src: &mut Cursor<&[u8]>) -> Result<u32, Error> {
    if src.remaining() < 4 {
        return Err(Error::Incomplete);
    }

    Ok(src.get_u32())
}
