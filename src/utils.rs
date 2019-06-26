use crate::constants::*;
use crate::errors::*;
use log::*;
use nix::sys::signal::{kill, SIGTERM};
use nix::unistd::Pid;
use std::fs::File;
use std::io::{prelude::*, BufRead};
use std::path::Path;

#[macro_export]
macro_rules! fatal {
    ($msg:tt) => {{
        error!("{} in file {} line {}", $msg, file!(), line!());
        clean();
        std::process::exit(1)
    }};
}

pub fn read_from_stdin() -> Result<String> {
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map_err(|e| {
            error!("{}", e);
            Error::StdinError
        })
}

pub fn clean() {
    debug!("remove pid and socket file");
    let _ = remove_file(OUT_DIR.join(PID_FILE));
    let _ = remove_file(OUT_DIR.join(SOCKET_FILE));
}

pub fn clean_and_exit() {
    clean();
    debug!("exit ...");
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

pub fn read_file<P: AsRef<Path>>(p: P) -> Result<String> {
    File::open(p)
        .map_err(|e| {
            error!("open file error {}", e);
            Error::OpenFileError
        })
        .and_then(|mut f: File| {
            let mut buf = String::new();
            f.read_to_string(&mut buf).map_err(|e| {
                error!("read to string error {}", e);
                Error::ReadToStringError
            })?;
            Ok(buf)
        })
}

pub fn kill_server() -> Result {
    let pid = read_file(OUT_DIR.join(PID_FILE))?
        .parse::<i32>()
        .map_err(|e| {
            error!("{}", e);
            Error::ParseError
        })?;
    kill(Pid::from_raw(pid), SIGTERM).map_err(|e| {
        error!("{}", e);
        Error::KillError
    })?;
    clean();
    Ok(())
}
