use bytes::Buf;
use prost::{DecodeError, Message};

include!(concat!(env!("OUT_DIR"), "/control.disconnect.rs"));

impl DisconnectRequest {
    pub fn deserialize(buf: &mut dyn Buf) -> Result<Self, DecodeError> {
        DisconnectRequest::decode(buf)
    }
}
