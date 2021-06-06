use bytes::BytesMut;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Get,
    Set(String, Option<usize>),
    Data(String),
    Ok,
}

impl Message {}

impl TryFrom<&[u8]> for Message {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bincode::deserialize::<Self>(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

impl TryFrom<BytesMut> for Message {
    type Error = std::io::Error;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Message::try_from(&value[..])
    }
}

impl TryFrom<Message> for BytesMut {
    type Error = std::io::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let encode = bincode::serialize(&value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(BytesMut::from(encode.as_slice()))
    }
}
