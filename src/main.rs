#[macro_use]
mod utils;
mod action;
mod clip;
mod codec;
mod constants;
mod errors;
mod peer;
mod response;
mod server;
mod user_group;
use crate::server::run_server;

fn main() {
    env_logger::init();
    run_server(true);
}
