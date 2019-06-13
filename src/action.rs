use crate::constants::OUT_DIR;
use crate::constants::*;
use crate::errors::{Error, Result};
use crate::response::Response;
use bincode::{deserialize, serialize};
use bytes::buf::BufMut;
use bytes::BytesMut;
use futures::future::Future;
use serde_derive::*;
use std::convert::{TryFrom, TryInto};
use tokio::net::UnixStream;

/// Represents variant types of content action
#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Clear,
    Get,
    Set(String),
}

#[allow(dead_code)]
impl Action {
    pub fn send(self) -> impl Future<Item = Response, Error = std::io::Error> {
        UnixStream::connect(OUT_DIR.join(SOCKET_FILE))
            .and_then(|stream| {
                let buf: BytesMut = self.try_into().unwrap();
                let hex_buf = hex::encode(&buf).into_bytes();
                let mut data = BytesMut::with_capacity(hex_buf.len() + SPLITTER_LEN);
                data.put(hex_buf);
                data.put(SPLITTER);
                tokio::io::write_all(stream, data)
            })
            .and_then(|(stream, _)| {
                let buf = Vec::new();
                tokio::io::read_to_end(stream, buf)
            })
            .and_then(|(_, buf)| {
                let resp =
                    Response::try_from(BytesMut::from(hex::decode(&buf[..buf.len() - 2]).unwrap()))
                        .unwrap();
                Ok(resp)
            })
    }
}

impl TryInto<BytesMut> for Action {
    type Error = Error;

    fn try_into(self) -> Result<BytesMut> {
        serialize(&self)
            .and_then(|v| Ok(BytesMut::from(v)))
            .map_err(|_| Error::ParseError)
    }
}

impl TryFrom<BytesMut> for Action {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self> {
        deserialize(&value).map_err(|_| Error::ParseError)
    }
}
