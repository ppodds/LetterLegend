use std::{io::Cursor, time::Duration};

use crate::frame::Frame;
use bytes::{Buf, BytesMut};
use prost::Message;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::Mutex,
    time::timeout,
};

#[derive(Debug)]
pub struct Connection {
    reader: Mutex<OwnedReadHalf>,
    writer: Mutex<OwnedWriteHalf>,
    buffer: Mutex<BytesMut>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        Self {
            reader: Mutex::new(reader),
            writer: Mutex::new(writer),
            buffer: Mutex::new(BytesMut::with_capacity(4096)),
        }
    }

    pub async fn read_frame(
        &self,
    ) -> Result<Option<Frame>, Box<dyn std::error::Error + Send + Sync>> {
        loop {
            // Attempt to parse a frame from the buffered data. If
            // enough data has been buffered, the frame is
            // returned.
            if let Some(frame) = self.parse_frame().await? {
                return Ok(Some(frame));
            }

            match timeout(
                Duration::from_secs(30),
                self.reader
                    .lock()
                    .await
                    .read_buf(&mut *self.buffer.lock().await),
            )
            .await
            {
                // There is not enough buffered data to read a frame.
                // Attempt to read more data from the socket.
                //
                // On success, the number of bytes is returned. `0`
                // indicates "end of stream".
                Ok(result) => {
                    if 0 == result? {
                        // The remote closed the connection. For this to be
                        // a clean shutdown, there should be no data in the
                        // read buffer. If there is, this means that the
                        // peer closed the socket while sending a frame.
                        if self.buffer.lock().await.is_empty() {
                            return Ok(None);
                        } else {
                            return Err("connection reset by peer".into());
                        }
                    }
                }
                Err(_) => return Err("read timeout, maybe connection closed".into()),
            }
        }
    }

    async fn parse_frame(&self) -> Result<Option<Frame>, Box<dyn std::error::Error + Send + Sync>> {
        let mut buf_mutex_guard = self.buffer.lock().await;
        // Create the `T: Buf` type.
        let mut buf = Cursor::new(&buf_mutex_guard[..]);

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
                buf_mutex_guard.advance(len);

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
    pub async fn write_frame(&self, frame: &Frame) -> Result<(), Box<dyn std::error::Error>> {
        match frame {
            Frame::Response(res) => {
                let mut buf = match res.get_data().as_ref() {
                    crate::frame::ResponseData::Connect(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::Disconnect(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::Heartbeat(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::CreateLobby(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::JoinLobby(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::QuitLobby(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::ListLobby(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::Ready(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::StartGame(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::Error(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::LobbyBroadcast(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::SetTile(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::FinishTurn(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::GetNewCard(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                    crate::frame::ResponseData::GameBroadcast(res) => {
                        let mut buf = BytesMut::with_capacity(res.encoded_len());
                        res.encode(&mut buf)?;
                        buf
                    }
                };

                {
                    let mut writer = self.writer.lock().await;
                    writer.write_u32_le(res.get_state()).await?;
                    writer.write_u32_le(buf.len().try_into().unwrap()).await?;
                    writer.write_buf(&mut buf).await?;
                }

                Ok(())
            }
            _ => Err("not implemented".into()),
        }
    }
}
