use tokio::net::TcpStream;

pub mod args;

use args::Args;

pub struct Client {
    pub args: Args,
}

impl Client {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub async fn connect(&self, addr: &str) -> crate::Result {
        let _socket = TcpStream::connect(addr).await;
        Ok(())
    }
}
