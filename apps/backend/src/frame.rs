use std::io::Cursor;

use bytes::Buf;
use prost::Message;

use crate::{
    model::control::connect::ConnectRequest, model::control::connect::ConnectResponse,
    model::control::disconnect::DisconnectResponse, model::control::heartbeat::HeartbeatResponse,
    model::lobby::create::CreateResponse, model::lobby::join::JoinRequest,
    model::lobby::join::JoinResponse, operation::Operation,
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
    CreateLobby,
    JoinLobby(JoinRequest),
}

#[derive(Debug, Clone)]
pub enum Response {
    Connect(ConnectResponse),
    Disconnect(DisconnectResponse),
    Heartbeat(HeartbeatResponse),
    CreateLobby(CreateResponse),
    JoinLobby(JoinResponse),
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
            Operation::CreateLobby => return Ok(()),
            Operation::JoinLobby => JoinRequest::decode(payload).err(),
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
            Operation::CreateLobby => Ok(Frame::Request(Request::CreateLobby)),
            Operation::JoinLobby => match JoinRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request::JoinLobby(req))),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
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
