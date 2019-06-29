use crate::action::Action;
use crate::constants::OUT_DIR;
use crate::constants::*;
use crate::errors::{Error, Result};
use crate::response::Response;
use crate::server::{is_running_server, run_daemon_server};
use crate::utils::{clean, read_from_stdin};
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
                if let Some(a) = action {
                    run_action(a);
                } else {
                    let content = read_from_stdin().unwrap_or_else(|e| fatal!(e));
                    run_action(Action::Set(content))
                }
            }
            Ok(ForkResult::Child) => {
                // a new child process
                run_daemon_server();
            }
            Err(e) => fatal!(e),
        }
    } else if let Some(a) = action {
        run_action(a);
    } else {
        let content = read_from_stdin().unwrap_or_else(|e| fatal!(e));
        run_action(Action::Set(content))
    }
}

fn run_action(action: Action) {
    let client = action
        .send_request()
        .and_then(|resp: Response| {
            if let Some(c) = resp.content {
                println!("{}", c);
            }
            Ok(())
        })
        .map_err(|err| {
            error!("client side error {:?}", err);
        });
    tokio::run(client);
}

impl Action {
    pub fn send_request(self) -> impl Future<Item = Response, Error = std::io::Error> {
        UnixStream::connect(OUT_DIR.join(SOCKET_FILE))
            .and_then(|stream| {
                let buf: BytesMut = self.try_into().unwrap_or_else(|e| fatal!(e));
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
                let resp = Response::try_from(BytesMut::from(
                    hex::decode(&buf[..buf.len() - SPLITTER_LEN]).unwrap_or_else(|e| fatal!(e)),
                ))
                .unwrap_or_else(|e| fatal!(e));
                Ok(resp)
            })
    }
}

impl TryInto<BytesMut> for Action {
    type Error = Error;

    fn try_into(self) -> Result<BytesMut> {
        serialize(&self)
            .and_then(|v| Ok(BytesMut::from(v)))
            .map_err(|e| {
                error!("{}", e);
                Error::ParseError
            })
    }
}

impl TryFrom<BytesMut> for Action {
    type Error = Error;

    fn try_from(value: BytesMut) -> Result<Self> {
        deserialize(&value).map_err(|e| {
            error!("{}", e);
            Error::ParseError
        })
    }
}
