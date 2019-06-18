extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::panic;
use std::process::Command;

#[test]
fn command_test() {
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
}