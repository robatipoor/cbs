extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::panic;
use std::process::Command;

#[test]
fn command_test() {
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

fn run_test<T>(test: T)
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    std::env::set_var("RUST_LOG", "debug");
    let _ = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("-s")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    let _result = panic::catch_unwind(|| {
        test();
    });
    let _ = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("-k")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
