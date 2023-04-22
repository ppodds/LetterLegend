use std::io::Cursor;

use crate::frame::Frame;
use bytes::{Buf, BytesMut};
use prost::Message;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub fn try_read_frame(
        &mut self,
    ) -> Result<Option<Frame>, Box<dyn std::error::Error + Send + Sync>> {
        // Attempt to parse a frame from the buffered data. If
        // enough data has been buffered, the frame is
        // returned.
        if let Some(frame) = self.parse_frame()? {
            return Ok(Some(frame));
        }

        let (reader, _) = self.stream.split();

        // There is not enough buffered data to read a frame.
        // Attempt to read more data from the socket.
        //
        // On success, the number of bytes is returned. `0`
        // indicates "end of stream".
        match reader.try_read_buf(&mut self.buffer) {
            Ok(0) => {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Err("connection safe closed".into());
                } else {
                    return Err("connection reset by peer".into());
                }
            }
            Ok(_) | Err(_) => Ok(None),
        }
    }

    pub async fn read_frame(
        &mut self,
    ) -> Result<Option<Frame>, Box<dyn std::error::Error + Send + Sync>> {
        loop {
            // Attempt to parse a frame from the buffered data. If
            // enough data has been buffered, the frame is
            // returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            let (mut reader, _) = self.stream.split();

            // There is not enough buffered data to read a frame.
            // Attempt to read more data from the socket.
            //
            // On success, the number of bytes is returned. `0`
            // indicates "end of stream".
            if 0 == reader.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>, Box<dyn std::error::Error + Send + Sync>> {
        // Create the `T: Buf` type.
        let mut buf = Cursor::new(&self.buffer[..]);

        // Check whether a full frame is available
        match Frame::check(&mut buf) {
            Ok(_) => {
                // Get the byte length of the frame
                let len = buf.position() as usize;

                // Reset the internal cursor for the
                // call to `parse`.
                buf.set_position(0);

                // Parse the frame
                let frame = match Frame::parse(&mut buf) {
                    Ok(frame) => frame,
                    Err(e) => panic!("parse error, this should not happen: {:?}", e),
                };

                // Discard the frame from the buffer
                self.buffer.advance(len);

                // Return the frame to the caller.
                Ok(Some(frame))
            }
            // Not enough data has been buffered
            Err(crate::frame::Error::Incomplete) => Ok(None),
            // client sent invalid data
            Err(crate::frame::Error::ProtobufDecodeFailed(_)) => Ok(None),
            // An error was encountered
            Err(crate::frame::Error::Other(e)) => Err(e),
        }
    }

    /// Write a frame to the connection.
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), Box<dyn std::error::Error>> {
        match frame {
            Frame::Response(res) => {
                let mut buf = match res {
                    crate::frame::Response::Connect(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::Response::Disconnect(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::Response::Heartbeat(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::Response::CreateLobby(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::Response::JoinLobby(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::Response::QuitLobby(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::Response::ListLobby(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                };
                let (_, mut writer) = self.stream.split();
                writer.write_u32_le(buf.len().try_into().unwrap()).await?;
                writer.write_buf(&mut buf).await?;
                Ok(())
            }
            _ => Err("not implemented".into()),
        }
    }
}
