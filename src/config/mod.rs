use std::path::PathBuf;

use crate::server::args::Args;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG_ARGS: Args =
        confy::load(&*get_config_file().unwrap()).unwrap_or_default();
}

pub fn get_config_file() -> Option<String> {
    get_home_dir_config()
        .map(|p| {
            p.join("config.toml")
                .to_str()
                .to_owned()
                .map(|p| p.to_string())
        })
        .flatten()
}

pub fn get_home_dir_config() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(".config").join("cbs"))
}
