extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::process::Command;
use std::thread;
use std::time::Duration;
#[test]
fn set_and_get_content_test() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("-sc")
        .arg("hi")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    thread::sleep(Duration::from_secs(1));
    let out = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("-p")
        .output()
        .unwrap()
        .stdout;
    thread::sleep(Duration::from_secs(1));
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("-k")
        .assert()
        .success();
    assert_eq!(std::str::from_utf8(&out).unwrap().trim(), "hi");
}
#[test]
fn clear_content_test() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("-c")
        .arg("hi")
        .assert()
        .success();
    thread::sleep(Duration::from_secs(1));
    let out = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("-C")
        .output()
        .unwrap()
        .stdout;
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("-k")
        .assert()
        .success();
    assert_eq!(std::str::from_utf8(&out).unwrap().trim(), "");
}
