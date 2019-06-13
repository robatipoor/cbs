use crate::constants::*;
use crate::errors::*;
use log::*;
use std::path::Path;

#[macro_export]
macro_rules! fatal {
    ($msg:tt) => {{
        error!("{} in file {} line {}", $msg, file!(), line!());
        clean();
        std::process::exit(1)
    }};
}

pub fn clean() {
    info!("remove pid and socket file");
    let _ = remove_file(OUT_DIR.join(PID_FILE));
    let _ = remove_file(OUT_DIR.join(SOCKET_FILE));
}

pub fn clean_and_exit() {
    clean();
    info!("exit ...");
    std::process::exit(0);
}

pub fn remove_file<P: AsRef<Path>>(p: P) -> Result {
    if p.as_ref().exists() {
        std::fs::remove_file(p).map_err(|e| {
            error!("{}", e);
            Error::RemoveFileError
        })
    } else {
        Err(Error::FileNotExistError)
    }
}
