use crate::errors::{Error, Result};
use crate::message::{Action, Selection};
use crate::response::Response;
use bytes::BytesMut;
#[cfg(target_os = "linux")]
use clipboard::x11_clipboard::{Primary, X11ClipboardContext};
use clipboard::{ClipboardContext, ClipboardProvider};
use log::*;
use std::convert::{TryFrom, TryInto};

#[cfg(target_os = "linux")]
pub fn handle_action(bytes: BytesMut) -> Result<BytesMut> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| {
        error!("{}", e);
        Error::ClipboardProviderError
    })?;
    let mut ctx_primary: X11ClipboardContext<Primary> = ClipboardProvider::new().unwrap();
    let content = match Action::try_from(bytes)? {
        Action::Clear(select) => {
            match select {
                Selection::Clipboard => {
                    ctx.set_contents(String::new()).map_err(|e| {
                        error!("{}", e);
                        Error::GetContentsError
                    })?;
                }
                Selection::Primary => {
                    ctx_primary.set_contents(String::new()).map_err(|e| {
                        error!("{}", e);
                        Error::GetContentsError
                    })?;
                }
                _ => unimplemented!(),
            }
            None
        }
        Action::Get(select) => match select {
            Selection::Clipboard => Some(ctx.get_contents().map_err(|e| {
                error!("{}", e);
                Error::GetContentsError
            })?),
            Selection::Primary => Some(ctx_primary.get_contents().map_err(|e| {
                error!("{}", e);
                Error::GetContentsError
            })?),
            _ => unimplemented!(),
        },
        Action::Set { content, select } => {
            match select {
                Selection::Clipboard => {
                    ctx.set_contents(content).map_err(|e| {
                        error!("{}", e);
                        Error::SetContentsError
                    })?;
                }
                Selection::Primary => {
                    ctx_primary.set_contents(content).map_err(|e| {
                        error!("{}", e);
                        Error::SetContentsError
                    })?;
                }
                _ => unimplemented!(),
            }
            None
        }
    };
    Response {
        success: true,
        content,
    }
    .try_into()
}

#[cfg(target_os = "macos")]
pub fn handle_action(bytes: BytesMut) -> Result<BytesMut> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| {
        error!("{}", e);
        Error::ClipboardProviderError
    })?;
    let content = match Action::try_from(bytes)? {
        Action::Clear(_) => {
            ctx.set_contents(String::new()).map_err(|e| {
                error!("{}", e);
                Error::GetContentsError
            })?;
            None
        }
        Action::Get(_) => Some(ctx.get_contents().map_err(|e| {
            error!("{}", e);
            Error::GetContentsError
        })?),
        Action::Set { content, .. } => {
            ctx.set_contents(content).map_err(|e| {
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
