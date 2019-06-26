use crate::action::Action;
use crate::constants::OUT_DIR;
use crate::constants::*;
use crate::errors::{Error, Result};
use crate::response::Response;
use crate::server::{is_running_server, run_server};
use crate::utils::read_from_stdin;
use bincode::{deserialize, serialize};
use bytes::buf::BufMut;
use bytes::BytesMut;
use futures::future::Future;
use log::*;
use nix::unistd::{fork, ForkResult};
use std::convert::{TryFrom, TryInto};
use std::thread;
use std::time::Duration;
use tokio::net::UnixStream;

pub fn action_handler(action: Option<Action>) {
    if !is_running_server() {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                // in parent process
                // wait util server is run
                while !is_running_server() {
                    thread::sleep(Duration::from_millis(500));
                }
                let _ = action.ok_or(()).and_then(|act| {
                    run_action(act);
                    Ok(())
                });
            }
            Ok(ForkResult::Child) => {
                // a new child process
                run_server(true);
            }
            Err(_) => eprintln!("Fork failed"),
        }
    } else if action.is_some() {
        run_action(action.unwrap());
    } else {
        run_action(Action::Set(read_from_stdin().unwrap()))
    }
}

fn run_action(action: Action) {
    let client = action
        .send()
        .and_then(|resp: Response| {
            if resp.content.is_some() {
                println!("{}", resp.content.unwrap());
            }
            Ok(())
        })
        .map_err(|err| {
            error!("client side error {:?}", err);
        });
    tokio::run(client);
}

impl Action {
    pub fn send(self) -> impl Future<Item = Response, Error = std::io::Error> {
        UnixStream::connect(OUT_DIR.join(SOCKET_FILE))
            .and_then(|stream| {
                let buf: BytesMut = self.try_into().unwrap();
                let hex_buf = hex::encode(&buf).into_bytes();
                let mut data = BytesMut::with_capacity(hex_buf.len() + SPLITTER_LEN);
                data.put(hex_buf);
                data.put(SPLITTER);
                tokio::io::write_all(stream, data)
            })
            .and_then(|(stream, _)| {
                let buf = Vec::new();
                tokio::io::read_to_end(stream, buf)
            })
            .and_then(|(_, buf)| {
                let resp =
                    Response::try_from(BytesMut::from(hex::decode(&buf[..buf.len() - 2]).unwrap()))
                        .unwrap();
                Ok(resp)
            })
    }
}

impl TryInto<BytesMut> for Action {
    type Error = Error;

    fn try_into(self) -> Result<BytesMut> {
        serialize(&self)
            .and_then(|v| Ok(BytesMut::from(v)))
            .map_err(|_| Error::ParseError)
    }
}

impl TryFrom<BytesMut> for Action {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self> {
        deserialize(&value).map_err(|_| Error::ParseError)
    }
}
