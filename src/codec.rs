use crate::constants::*;
use bytes::{buf::BufMut, BytesMut};
use futures::{Poll, Stream};
use std::net::Shutdown;
use tokio::net::UnixStream;
use tokio::prelude::*;

pub struct Codec {
    socket: UnixStream,
    read_buf: BytesMut,
    write_buf: BytesMut,
}

impl Codec {
    pub fn new(socket: UnixStream) -> Self {
        Codec {
            socket,
            read_buf: BytesMut::new(),
            write_buf: BytesMut::new(),
        }
    }

    pub fn write_buffer(&mut self, data: BytesMut) {
        let hex_buf = hex::encode(&data).into_bytes();
        self.write_buf.reserve(hex_buf.len()); // TODO
        self.write_buf.put(hex_buf);
        self.write_buf.extend_from_slice(SPLITTER.as_bytes());
    }

    pub fn fill_read_buffer(&mut self) -> Poll<(), tokio::io::Error> {
        loop {
            self.read_buf.reserve(BUFFER_SIZE);
            let n = match self.socket.read_buf(&mut self.read_buf) {
                Ok(Async::Ready(n)) => n,
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) => return Err(e),
            };
            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }

    pub fn poll_flush(&mut self) -> Poll<(), tokio::io::Error> {
        while !self.write_buf.is_empty() {
            let n = match self.socket.poll_write(&self.write_buf) {
                Ok(Async::Ready(n)) => n,
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) => return Err(e),
            };
            assert!(n > 0);
            self.write_buf.split_to(n);
            if self.write_buf.is_empty() {
                let _ = self.socket.shutdown(Shutdown::Both);
            }
        }
        Ok(Async::Ready(()))
    }
}

impl Stream for Codec {
    type Item = BytesMut;
    type Error = tokio::io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let socket_closed = self.fill_read_buffer()?.is_ready();
        let position = self
            .read_buf
            .windows(SPLITTER_LEN)
            .enumerate()
            .find(|&(_, p)| p == SPLITTER.as_bytes())
            .map(|(i, _)| i);

        if let Some(pos) = position {
            // Remove the data from the read buffer and set it to `data`.
            let mut data = self.read_buf.split_to(pos + 2);
            // Drop the trailing \r\n
            data.split_off(pos);
            if let Ok(hex_decoded) = hex::decode(&data) {
                return Ok(Async::Ready(Some(BytesMut::from(hex_decoded))));
            }
        }

        if socket_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
