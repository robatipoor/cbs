use crate::action::Action;
use crate::constants::*;
use bytes::{buf::BufMut, BytesMut};
use futures::{Poll, Stream};
use std::convert::TryInto;
use std::net::Shutdown;
use tokio::codec::Encoder;
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
        self.write_buf.reserve(hex_buf.len()); //TODO
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

impl Encoder for Codec {
    type Item = Action;
    type Error = std::io::Error;
    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let b: BytesMut = item.try_into().unwrap();
        dst.reserve(b.len() + SPLITTER_LEN);
        dst.put(b);
        dst.put_slice(SPLITTER.as_bytes());
        Ok(())
    }
}

// impl Decoder for Codec {
//     type Item = Action;
//      type Error = std::io::Error;
//     fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>{
//         let newline_offset = self.read_buf.windows(2).enumerate().find(|&(_, p)| p == b"\r\n").map(|(i, _)| i);
//     if let Some(newline_index) = newline_offset {
//         // Found a '\n' in the string.

//         // The index of the '\n' is at the sum of the start position + the offset found.
//         let newline_index = newline_offset + self.next_index;

//         // Split the buffer at the index of the '\n' + 1 to include the '\n'.
//         // `split_to` returns a new buffer with the contents up to the index.
//         // The buffer on which `split_to` is called will now start at this index.
//         let line = src.split_to(newline_index + 2);

//         // Trim the `\n` from the buffer because it's part of the protocol,
//         // not the data.
//         let line = &line[..line.len() - 2];

//         // Convert the bytes to a string and panic if the bytes are not valid utf-8.
//         let line = str::from_utf8(&line).expect("invalid utf8 data");

//         // Set the search start index back to 0.
//         self.next_index = 0;

//         // Return Ok(Some(...)) to signal that a full frame has been produced.
//         Ok(Some(line.to_string()))
//     } else {
//         // '\n' not found in the string.

//         // Tell the next call to start searching after the current length of the buffer
//         // since all of it was scanned and no '\n' was found.
//         self.next_index = buf.len();

//         // Ok(None) signifies that more data is needed to produce a full frame.
//         Ok(None)
//     }
//     }
// }

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
