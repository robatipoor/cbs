use crate::action::Action;
use clap::*;

#[derive(Debug, Default)]
pub struct AppArgs {
    pub action: Option<Action>,
    pub log: bool,
    pub server: bool,
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
            .arg(
                Arg::with_name("server")
                    .short("s")
                    .long("server")
                    .help("Start server clipboard as daemon")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("log")
                    .short("l")
                    .long("log")
                    .help("Show logs")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("kill")
                    .short("k")
                    .long("kill")
                    .help("kill clipboard server")
                    .takes_value(false),
            )
            .get_matches();
        let mut app_args: AppArgs = AppArgs::default();
        if matches.is_present("log") {
            app_args.log = true;
            return app_args;
        }
        if matches.is_present("paste") {
            app_args.action = Some(Action::Get);
        } else if let Some(p) = matches.value_of("copy") {
            app_args.action = Some(Action::Set(p.to_owned()));
        } else if matches.is_present("clear") {
            app_args.action = Some(Action::Clear);
        } else if matches.is_present("kill") {
            app_args.action = Some(Action::Kill);
        }
        if matches.is_present("server") {
            app_args.server = true;
        }

        app_args
    }
}
