use bytes::Buf;
use prost::{DecodeError, Message};

include!(concat!(env!("OUT_DIR"), "/control.heartbeat.rs"));

impl HeartbeatRequest {
    pub fn deserialize(buf: &mut dyn Buf) -> Result<Self, DecodeError> {
        HeartbeatRequest::decode(buf)
    }
}
