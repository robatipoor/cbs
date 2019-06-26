extern crate assert_cmd;

use crate::utils::kill_server;
use assert_cmd::prelude::*;
use std::panic;
use std::process::Command;

fn run_test<T>(test: T)
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    kill_server().ok();
    let _result = panic::catch_unwind(|| {
        test();
    });
    kill_server().ok();
}

#[test]
fn cbs_command_test() {
    run_test(|| {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("-c")
            .arg("hi")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        let out = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("-p")
            .output()
            .unwrap()
            .stdout;
        assert_eq!(std::str::from_utf8(&out).unwrap().trim(), "hi");
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("-C")
            .assert()
            .success();
        let out = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("-p")
            .output()
            .unwrap()
            .stdout;
        assert_eq!(std::str::from_utf8(&out).unwrap().trim(), "");
    });
}
