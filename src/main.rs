extern crate cbs;
#[macro_use]
extern crate clap;

mod args;

use crate::args::AppArgs;
use cbs::os::action_handler;

fn main() {
    env_logger::init();
    let app = AppArgs::get_app_args();
    action_handler(app.action);
}
