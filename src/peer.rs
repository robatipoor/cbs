use futures::{Future, Poll, Stream};
use tokio::net::UnixStream;
use tokio::prelude::*;

use crate::clip::handle_action;
use crate::codec::Codec;
use crate::errors::*;
use log::*;

pub struct Peer {
    codec: Codec,
}

impl Peer {
    pub fn new(socket: UnixStream) -> Self {
        Peer {
            codec: Codec::new(socket),
        }
    }
}

impl Future for Peer {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        while let Async::Ready(data) = self.codec.poll().map_err(|e| {
            error!("{}",e);
            Error::PollCodecError
        })? {
            if let Some(d) = data {
                self.codec.write_buffer(handle_action(d).unwrap());
            } else {
                return Ok(Async::Ready(()));
            }
        }
        let _ = self.codec.poll_flush().map_err(|e| {
            error!("{}",e);
            Error::PollFlushError
        })?;
        Ok(Async::NotReady)
    }
}
