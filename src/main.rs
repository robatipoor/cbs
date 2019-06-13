extern crate clap;
extern crate nix;

#[macro_use]
mod utils;
mod action;
mod cli;
mod clip;
mod codec;
mod constants;
mod errors;
mod peer;
mod response;
mod server;
mod user_group;

use crate::action::Action;
use crate::cli::AppArgs;
use crate::response::Response;
use crate::server::{is_running_server, run_server};
use nix::unistd::{fork, ForkResult};
use std::thread;
use std::time::Duration;
use tokio::prelude::*;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let app = AppArgs::get_app_args();
    if app.server {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                // in parent process
                while !is_running_server() {
                    thread::sleep(Duration::from_millis(500));
                }
                let _ = app.action.ok_or(()).and_then(|act| {
                    run_action(act);
                    Ok(())
                });
                return;
            }
            Ok(ForkResult::Child) => {
                // a new child process
                run_server(true);
            }
            Err(_) => println!("Fork failed"),
        }
    } else {
        let _ = app.action.ok_or(()).and_then(|act| {
            run_action(act);
            Ok(())
        });
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
            eprintln!("error {:?}", err);
        });
    tokio::run(client);
}
