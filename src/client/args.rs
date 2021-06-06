use serde::{Deserialize, Serialize};
use clap::Clap;

#[derive(Debug, Clap, Default, Serialize, Deserialize)]
// #[clap(
//     version = "0.1.0",
//     author = "Mahdi Robatipoor <mahdi.robatipoor@gmail.com>"
// )]
pub struct Args {}

pub fn get_args() -> Args {
    Args::parse()
}
