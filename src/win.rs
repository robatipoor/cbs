use crate::action::Action;
use crate::errors::{Error, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use log::*;
use std::io::BufRead;

pub fn action_handler(action: Option<Action>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    if action.is_some() {
        match action.unwrap() {
            Action::Clear => ctx.set_contents(String::new()).unwrap(),
            Action::Get => println!("{}", ctx.get_contents().unwrap()),
            Action::Set(data) => ctx.set_contents(data).unwrap(),
        }
    } else {
        ctx.set_contents(read_from_stdin().unwrap()).unwrap()
    }
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

#[cfg(test)]
mod win_test {
    // use assert_cmd::prelude::*;
    // use std::process::Command;
    // TODO add tests

    #[test]
    fn win_cbs_test() {

    }
}
