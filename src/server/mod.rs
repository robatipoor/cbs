use std::net::SocketAddr;

use futures::prelude::*;

pub mod args;

use log::error;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;

use crate::{codec::MessageCodec, message::Message};
#[derive(Debug)]
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub async fn run(&self) -> crate::Result {
        let listener = TcpListener::bind(&self.addr)
            .await
            .map_err(|e| e.to_string())?;
        loop {
            let (stream, addr) = listener.accept().await.map_err(|e| e.to_string())?;
            tokio::spawn(async move {
                if let Err(e) = process(stream, addr).await {
                    error!("failed process error message : {} addr : {}", e, addr);
                }
            });
        }
    }
}

pub async fn process(stream: TcpStream, addr: SocketAddr) -> crate::Result {
    let (mut sink, mut stream) = Framed::new(stream, MessageCodec::new()).split();
    loop {
        tokio::select! {
            Some(msg) = stream.next() => {
                let msg = msg.map_err(|e|e.to_string())?;
                let msg = handler(msg).await?;
                sink.send(msg).await;
            },
        }
    }
}

pub async fn handler(msg: Message) -> crate::Result<Message> {
    match msg {
        Message::Get => Ok(Message::Data("hello".to_string())),
        Message::Set(content, ttl) => Ok(Message::Ok),
        _ => Err("invalid input data".to_string()),
    }
}
