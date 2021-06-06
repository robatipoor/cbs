pub mod client;
pub mod clipboard;
pub mod codec;
pub mod config;
pub mod server;
pub mod utils;
pub mod message;
pub mod constant;

pub type Result<T = ()> = std::result::Result<T, String>;
