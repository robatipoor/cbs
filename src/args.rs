use cbs::action::Action;
use clap::{App, Arg, ArgMatches};

#[derive(Debug, Default)]
pub struct AppArgs {
    pub action: Option<Action>,
}

impl AppArgs {
    pub fn get_app_args() -> AppArgs {
        let matches: ArgMatches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                Arg::with_name("copy")
                    .short("c")
                    .long("copy")
                    .value_name("content")
                    .help("Set a text content")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("paste")
                    .short("p")
                    .long("paste")
                    .help("Paste text content")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("clear")
                    .short("C")
                    .long("clear")
                    .help("Clear content clipboard")
                    .takes_value(false),
            )
            .get_matches();
        let mut app_args: AppArgs = AppArgs::default();
        if matches.is_present("paste") {
            app_args.action = Some(Action::Get);
        } else if let Some(p) = matches.value_of("copy") {
            app_args.action = Some(Action::Set(p.to_owned()));
        } else if matches.is_present("clear") {
            app_args.action = Some(Action::Clear);
        }
        app_args
    }
}
