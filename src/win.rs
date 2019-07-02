use crate::message::{Action, Selection};
use clipboard::{ClipboardContext, ClipboardProvider};

pub fn action_handler(action: Action) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    match action {
        Action::Clear(select) => match select {
            Selection::Clipboard => {
                ctx.set_contents(String::new()).unwrap();
            }
            _ => not_support_msg(),
        },
        Action::Get(select) => match select {
            Selection::Clipboard => println!("{}", ctx.get_contents().unwrap()),
            _ => not_support_msg(),
        },
        Action::Set { content, select } => match select {
            Selection::Clipboard => ctx.set_contents(content).unwrap(),
            _ => not_support_msg(),
        },
    }
}

#[cfg(test)]
mod win_test {
    // use assert_cmd::prelude::*;
    // use std::process::Command;
    // TODO add tests

    #[test]
    fn win_cbs_test() {}
}

fn not_support_msg() {
    println!("windows not support !!!");
    std::process::exit(0);
}
