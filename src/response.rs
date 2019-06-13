use bincode::{deserialize, serialize};
use bytes::BytesMut;
use serde_derive::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use crate::errors::*;
use log::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub success: bool,
    pub content: Option<String>,
}

impl TryInto<BytesMut> for Response {
    type Error = Error;

    fn try_into(self) -> Result<BytesMut> {
        Ok(BytesMut::from(serialize(&self).map_err(|e| {
            error!("{}",e);
            Error::ParseError
        })?))
    }
}

impl TryFrom<BytesMut> for Response {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self> {
        Ok(deserialize(&value).map_err(|e| {
            error!("{}",e);
            Error::ParseError
        })?)
    }
}
