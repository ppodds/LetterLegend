use bytes::Buf;
use prost::{DecodeError, Message};

include!(concat!(env!("OUT_DIR"), "/control.connect.rs"));

impl ConnectRequest {
    pub fn deserialize(buf: &mut dyn Buf) -> Result<Self, DecodeError> {
        ConnectRequest::decode(buf)
    }
}
