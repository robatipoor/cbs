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
use crate::constants::{OUT_DIR, STD_OUT_FILE};
use crate::response::Response;
use crate::server::{is_running_server, run_server};
use crate::utils::*;
use log::*;
use nix::unistd::{fork, ForkResult};
use std::io::BufRead;
use std::thread;
use std::time::Duration;
use tokio::prelude::*;

fn main() {
    env_logger::init();
    let app = AppArgs::get_app_args();
    if app.log {
        read_file(OUT_DIR.join(STD_OUT_FILE))
            .and_then(|s| {
                println!("{}", s);
                Ok(())
            })
            .unwrap_or_else(|e| fatal!(e));
        return;
    }
    if app.server {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                // in parent process
                // check server is running
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
    } else if app.action.is_some() {
        run_action(app.action.unwrap());
        return;
    }

    std::io::stdin().lock().lines().next().map(|x| match x {
        Ok(c) => run_action(Action::Set(c)),
        Err(e) => fatal!(e),
    });
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
