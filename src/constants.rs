use std::path::PathBuf;

pub const SPLITTER: &str = "\r\n";
pub const SPLITTER_LEN: usize = 2; // SPLITTER.len(); /** nightly only worked */
pub const BUFFER_SIZE: usize = 1024;
pub const STD_ERR_FILE: &str = "cbs.err";
pub const STD_OUT_FILE: &str = "cbs.out";
pub const SOCKET_FILE: &str = "cbs.socket";
pub const PID_FILE: &str = "cbs.pid";

lazy_static! {
    pub static ref OUT_DIR: PathBuf =
        PathBuf::from(std::env::var("OUT_DIR").unwrap_or_else(|_| "/tmp".to_owned()));
}
