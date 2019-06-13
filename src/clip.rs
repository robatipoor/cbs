use crate::action::Action;
use crate::response::Response;
use bytes::BytesMut;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::convert::{TryFrom, TryInto};
use crate::errors::{Error,Result};
use log::*;

pub fn handle_action(bytes: BytesMut) -> Result<BytesMut> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e|{
        error!("{}",e);
        Error::ClipboardProviderError
    })?;
    let content = match Action::try_from(bytes)? {
        Action::Clear => {
            ctx.set_contents(String::new()).unwrap();
            None
        }
        Action::Get => Some(ctx.get_contents().unwrap()),
        Action::Set(msg) => {
            ctx.set_contents(msg).unwrap();
            None
        }
    };
    
    Response {
        success: true,
        content,
    }.try_into()
}
