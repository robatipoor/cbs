use tokio_util::codec::{Decoder, Encoder};

use crate::message::Message;

#[derive(Debug)]
pub struct ClipboardCodec;

impl ClipboardCodec {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder<Message> for ClipboardCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Message, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Decoder for ClipboardCodec {
    type Item = Message;

    type Error = std::io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
