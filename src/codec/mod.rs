use crate::{constant::SPLITTER, message::Message};
use bytes::{Buf, BufMut, BytesMut};
use std::{convert::TryFrom, io};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub struct MessageCodec {
    next_index: usize,
    max_length: usize,
    is_discarding: bool,
}

impl MessageCodec {
    pub fn new() -> Self {
        Self {
            next_index: 0,
            max_length: usize::MAX,
            is_discarding: false,
        }
    }
}

impl Decoder for MessageCodec {
    type Item = Message;

    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            let read_to = std::cmp::min(self.max_length.saturating_add(1), src.len());
            let newline_offset = src[self.next_index..read_to]
                .iter()
                .position(|b| *b == SPLITTER);

            match (self.is_discarding, newline_offset) {
                (true, Some(offset)) => {
                    src.advance(offset + self.next_index + 1);
                    self.is_discarding = false;
                    self.next_index = 0;
                }
                (true, None) => {
                    src.advance(read_to);
                    self.next_index = 0;
                    if src.is_empty() {
                        return Ok(None);
                    }
                }
                (false, Some(offset)) => {
                    // Found a msg!
                    let new_msg_index = offset + self.next_index;
                    self.next_index = 0;
                    let msg = src.split_to(new_msg_index + 1);
                    let msg = &msg[..msg.len() - 1];
                    let msg = without_carriage_return(msg);
                    let msg = Message::try_from(msg)?;
                    return Ok(Some(msg));
                }
                (false, None) if src.len() > self.max_length => {
                    self.is_discarding = true;
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "maximum length without finding message",
                    ));
                }
                (false, None) => {
                    self.next_index = read_to;
                    return Ok(None);
                }
            }
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Message>, io::Error> {
        Ok(match self.decode(buf)? {
            Some(frame) => Some(frame),
            None => {
                // No terminating newline - return remaining data, if any
                if buf.is_empty() || buf == &b"\r"[..] {
                    None
                } else {
                    let msg = buf.split_to(buf.len());
                    let msg = without_carriage_return(&msg);
                    let msg = Message::try_from(msg)?;
                    self.next_index = 0;
                    Some(msg)
                }
            }
        })
    }
}

impl Encoder<Message> for MessageCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let src = BytesMut::try_from(item)?;
        dst.reserve(src.len() + 1);
        dst.put(src);
        dst.put_u8(SPLITTER);
        Ok(())
    }
}

fn without_carriage_return(src: &[u8]) -> &[u8] {
    if let Some(&b'\r') = src.last() {
        &src[..src.len() - 1]
    } else {
        src
    }
}
