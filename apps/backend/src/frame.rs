use std::io::Cursor;

use bytes::Buf;
use prost::Message;

use crate::{
    model::control::connect::ConnectRequest, model::control::connect::ConnectResponse,
    model::control::disconnect::DisconnectResponse, model::control::heartbeat::HeartbeatResponse,
    model::lobby::create::CreateRequest, model::lobby::create::CreateResponse,
    model::lobby::join::JoinRequest, model::lobby::join::JoinResponse,
    model::lobby::list::ListResponse, model::lobby::quit::QuitResponse,
    model::lobby::ready::ReadyResponse, operation::Operation,
};

#[derive(Debug)]
pub enum Frame {
    Error(String),
    Request(Request),
    Response(Response),
}

#[derive(Debug, Clone)]
pub enum Request {
    Connect(ConnectRequest),
    Disconnect,
    Heartbeat,
    CreateLobby(CreateRequest),
    JoinLobby(JoinRequest),
    QuitLobby,
    ListLobby,
    Ready,
}

#[derive(Debug, Clone)]
pub enum Response {
    Connect(ConnectResponse),
    Disconnect(DisconnectResponse),
    Heartbeat(HeartbeatResponse),
    CreateLobby(CreateResponse),
    JoinLobby(JoinResponse),
    QuitLobby(QuitResponse),
    ListLobby(ListResponse),
    Ready(ReadyResponse),
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
        src.set_position(4);
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
        let payload_len = get_u32(src)?;
        let payload = src.take(payload_len as usize);
        match op {
            Operation::Connect => match ConnectRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request::Connect(req))),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::Disconnect => Ok(Frame::Request(Request::Disconnect)),
            Operation::Heartbeat => Ok(Frame::Request(Request::Heartbeat)),
            Operation::CreateLobby => match CreateRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request::CreateLobby(req))),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::JoinLobby => match JoinRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request::JoinLobby(req))),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::QuitLobby => Ok(Frame::Request(Request::QuitLobby)),
            Operation::ListLobby => Ok(Frame::Request(Request::ListLobby)),
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
