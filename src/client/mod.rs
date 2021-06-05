use clap::Clap;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

pub struct Client {
    pub args: Args,
}

impl Client {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub async fn connect(&self,addr:&str) -> crate::Result {
        let _socket = TcpStream::connect(addr).await;
        Ok(())
    }
}

#[derive(Debug, Clap, Default, Serialize, Deserialize)]
// #[clap(
//     version = "0.1.0",
//     author = "Mahdi Robatipoor <mahdi.robatipoor@gmail.com>"
// )]
pub struct Args {}

pub fn get_args() -> Args {
    Args::parse()
}
