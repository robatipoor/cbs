use crate::constants::*;
use log::*;

#[macro_export]
macro_rules! fatal {
    ($msg:tt) => {{
        error!("{} in file {} line {}", $msg, file!(), line!());
        let _ = std::fs::remove_file(OUT_DIR.join(PID_FILE));
        let _ = std::fs::remove_file(OUT_DIR.join(SOCKET_FILE));
        std::process::exit(1)
    }};
}

pub fn clean() {
    info!("remove pid and socket file");
    let _ = std::fs::remove_file(OUT_DIR.join(PID_FILE));
    let _ = std::fs::remove_file(OUT_DIR.join(SOCKET_FILE));
}

pub fn clean_and_exit() {
    clean();
    info!("exit ...");
    std::process::exit(0);
}
