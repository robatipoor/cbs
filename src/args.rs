use cbs::message::{Action, Selection};
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::errors::*;
use log::*;
use std::io::prelude::*;

pub struct AppArgs {
    pub action: Action,
}

impl AppArgs {
    pub fn get_app_args() -> AppArgs {
        let matches: ArgMatches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .subcommand(
                SubCommand::with_name("copy")
                    .about("copy text content")
                    .args(&[
                        Arg::with_name("content")
                            .value_name("text-content")
                            .takes_value(true)
                            .help("set content")
                            .required(true),
                        // Arg::with_name("timer")
                        //     .short("t")
                        //     .long("timer")
                        //     .value_name("seconds")
                        //     .takes_value(true)
                        //     .help("auto clear contnet")
                        //     .required(false),
                        Arg::with_name("selection")
                            .short("s")
                            .long("select")
                            .possible_values(&["clipboard", "primary"])
                            .default_value("clipboard")
                            .help("to access clipboard or primary")
                            .required(false),
                    ]),
            )
            .subcommand(
                SubCommand::with_name("clear")
                    .about("clear text content")
                    .arg(
                        Arg::with_name("selection")
                            .short("s")
                            .long("select")
                            .possible_values(&["clipboard", "primary"])
                            .default_value("clipboard")
                            .help("to access clipboard or primary")
                            .required(false),
                    ),
            )
            .subcommand(
                SubCommand::with_name("paste")
                    .about("paste text content")
                    .arg(
                        Arg::with_name("selection")
                            .short("s")
                            .long("select")
                            .possible_values(&["clipboard", "primary"])
                            .default_value("clipboard")
                            .help("to access clipboard or primary")
                            .required(false),
                    ),
            )
            .arg(
                Arg::with_name("selection")
                    .short("s")
                    .long("select")
                    .possible_values(&["clipboard", "primary"])
                    .default_value("clipboard")
                    .help("to access clipboard or primary")
                    .required(false),
            )
            .get_matches();
        if let Some(matches) = matches.subcommand_matches("copy") {
            if let Some(c) = matches.value_of("content") {
                let mut action = Action::set(c);
                if let Some(s) = matches.value_of("selection") {
                    match s {
                        "clipboard" => {
                            action = action.select(Selection::Clipboard);
                        }
                        "primary" => {
                            action = action.select(Selection::Primary);
                        }
                        _ => {
                            unimplemented!("invlaid input");
                        }
                    }
                }
                return AppArgs { action };
            } else {
                panic!("you must set content")
            }
        } else if let Some(matches) = matches.subcommand_matches("paste") {
            let mut action = Action::get();
            if let Some(s) = matches.value_of("selection") {
                match s {
                    "clipboard" => {
                        action = action.select(Selection::Clipboard);
                    }
                    "primary" => {
                        action = action.select(Selection::Primary);
                    }
                    _ => {
                        unimplemented!("invlaid input");
                    }
                }
            }
            return AppArgs { action };
        } else if let Some(matches) = matches.subcommand_matches("clear") {
            let mut action = Action::clear();
            if let Some(s) = matches.value_of("selection") {
                match s {
                    "clipboard" => {
                        action = action.select(Selection::Clipboard);
                    }
                    "primary" => {
                        action = action.select(Selection::Primary);
                    }
                    _ => {
                        unimplemented!("invlaid input");
                    }
                }
            }
            return AppArgs { action };
        } else {
            let mut action = Action::set(read_from_stdin().unwrap());
            if let Some(s) = matches.value_of("selection") {
                match s {
                    "clipboard" => {
                        action = action.select(Selection::Clipboard);
                    }
                    "primary" => {
                        action = action.select(Selection::Primary);
                    }
                    _ => {
                        unimplemented!("invlaid input");
                    }
                }
            }
            return AppArgs { action };
        }
    }
}

pub fn read_from_stdin() -> Result<String> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    stdin_lock.read_to_string(&mut buf).map_err(|e| {
        error!("{}", e);
        Error::StdinError
    })?;
    Ok(buf)
}