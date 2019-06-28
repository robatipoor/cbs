use crate::constants::*;
use crate::errors::{Error, Result};
use crate::peer::Peer;
use crate::user_group::UserGroup;
use crate::utils::*;
use daemonize::Daemonize;
use futures::{Future, Stream};
use log::*;
use std::fs::File;
use std::os::unix;
use tokio::net::UnixListener;

pub fn run_daemon_server() {
    clean();
    start_daemonize().unwrap_or_else(|e| fatal!(e));
    debug!("start daemon proccess");
    ctrlc::set_handler(clean_and_exit).unwrap_or_else(|e| fatal!(e));
    debug!("unix listener bind to {:?}", OUT_DIR.join(SOCKET_FILE));
    let listener = UnixListener::bind(OUT_DIR.join(SOCKET_FILE)).unwrap_or_else(|e| fatal!(e));
    let server = listener
        .incoming()
        .for_each(|socket| {
            let peer = Peer::new(socket).map_err(|err| {
                error!("peer error => {}", err);
            });
            tokio::spawn(peer);
            Ok(())
        })
        .map_err(|e| {
            error!("error => {}", e);
        });
    debug!("start server");
    tokio::run(server);
}

fn start_daemonize() -> Result {
    let stdout = File::create(OUT_DIR.join(STD_OUT_FILE)).unwrap_or_else(|e| fatal!(e));
    let stderr = File::create(OUT_DIR.join(STD_ERR_FILE)).unwrap_or_else(|e| fatal!(e));
    let ug = UserGroup::default();
    let daemonize = Daemonize::new()
        .pid_file(OUT_DIR.join(PID_FILE))
        .chown_pid_file(true)
        .working_directory(OUT_DIR.as_path())
        .user(&*ug.get_user().unwrap_or_else(|| "non-user".to_owned()))
        .group(&*ug.get_group().unwrap_or_else(|| "non-group".to_owned()))
        .umask(0o027)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            debug!("Success, daemonized");
            Ok(())
        }
        Err(e) => {
            error!("error daemonized {}", e);
            Err(Error::DaemonError)
        }
    }
}

pub fn is_running_server() -> bool {
    unix::net::UnixStream::connect(OUT_DIR.join(SOCKET_FILE)).is_ok()
}
