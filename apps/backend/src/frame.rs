use std::io::Cursor;

use bytes::Buf;
use prost::Message;

use crate::{
    model::control::disconnect::DisconnectRequest, model::control::disconnect::DisconnectResponse,
    model::control::heartbeat::HeartbeatRequest, model::control::heartbeat::HeartbeatResponse,
    operation::Operation,
};

#[derive(Debug)]
pub enum Frame {
    Error(String),
    Request(Request),
    Response(Response),
}

#[derive(Debug, Clone)]
pub enum Request {
    Disconnect(DisconnectRequest),
    Heartbeat(HeartbeatRequest),
}

#[derive(Debug, Clone)]
pub enum Response {
    Disconnect(DisconnectResponse),
    Heartbeat(HeartbeatResponse),
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
            Operation::Disconnect => DisconnectRequest::decode(payload).err(),
            Operation::Heartbeat => HeartbeatRequest::decode(payload).err(),
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
            Operation::Disconnect => match DisconnectRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request::Disconnect(req))),
                Err(e) => Err(Error::ProtobufDecodeFailed(e)),
            },
            Operation::Heartbeat => match HeartbeatRequest::decode(payload) {
                Ok(req) => Ok(Frame::Request(Request::Heartbeat(req))),
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
