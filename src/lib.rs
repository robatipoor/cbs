// #![feature(const_str_len)]

#[cfg(target_family = "unix")]
extern crate bincode;
#[cfg(target_family = "unix")]
extern crate bytes;
extern crate clipboard;
#[cfg(target_family = "unix")]
extern crate ctrlc;
#[cfg(target_family = "unix")]
extern crate daemonize;
extern crate env_logger;
extern crate failure;
#[cfg(target_family = "unix")]
extern crate futures;
#[cfg(target_family = "unix")]
extern crate hex;
#[macro_use]
extern crate lazy_static;
extern crate log;
#[cfg(target_family = "unix")]
extern crate nix;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(target_family = "unix")]
extern crate tokio;
#[cfg(target_family = "unix")]
extern crate users;

#[macro_use]
#[cfg(target_family = "unix")]
pub mod utils;
pub mod message;
#[cfg(target_family = "unix")]
pub mod clip;
#[cfg(target_family = "unix")]
pub mod codec;
pub mod constants;
pub mod errors;
#[cfg(target_family = "unix")]
pub mod peer;
#[cfg(target_family = "unix")]
pub mod response;
#[cfg(target_family = "unix")]
pub mod server;
#[cfg(test)]
#[cfg(target_family = "unix")]
pub mod tests;
#[cfg(target_family = "unix")]
pub mod user_group;

#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "win.rs")]
pub mod os;

pub use crate::message::Action;
