use crate::action::Action;
use crate::errors::{Error, Result};
use crate::response::Response;
use bytes::BytesMut;
use clipboard::{ClipboardContext, ClipboardProvider};
use log::*;
use std::convert::{TryFrom, TryInto};

pub fn handle_action(bytes: BytesMut) -> Result<BytesMut> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| {
        error!("{}", e);
        Error::ClipboardProviderError
    })?;
    let content = match Action::try_from(bytes)? {
        Action::Clear => {
            ctx.set_contents(String::new()).map_err(|e| {
                error!("{}", e);
                Error::GetContentsError
            })?;
            None
        }
        Action::Get => Some(ctx.get_contents().map_err(|e| {
            error!("{}", e);
            Error::GetContentsError
        })?),
        Action::Set(msg) => {
            ctx.set_contents(msg).map_err(|e| {
                error!("{}", e);
                Error::SetContentsError
            })?;
            None
        }
    };
    Response {
        success: true,
        content,
    }
    .try_into()
}
