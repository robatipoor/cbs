extern crate bincode;
extern crate bytes;
extern crate clipboard;
extern crate daemonize;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate hex;
extern crate lazy_static;
extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate tokio;
extern crate users;
extern crate ctrlc;

#[macro_use]
pub mod utils;
pub mod action;
pub mod peer;
pub mod response;
pub mod server;
pub mod user_group;
pub mod clip;
pub mod codec;
pub mod constants;
pub mod errors;

pub use crate::server::run_server;
pub use crate::action::Action;
