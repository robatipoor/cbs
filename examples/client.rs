#[cfg(target_family = "unix")]
fn main() {
    use cbs::Action;
    use std::path::PathBuf;
    use std::process::Command;
    use tokio::prelude::*;
    dotenv::dotenv().ok();
    let root_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let path_cbs = root_dir.join("target/debug/cbs");
    let _ = Command::new(path_cbs)
        .arg("-s")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    let client = Action::Get
        .send()
        .and_then(|resp| {
            println!("reponse is {:?}", resp);
            Ok(())
        })
        .map_err(|err| {
            eprintln!("error {:?}", err);
        });
    tokio::run(client);
}
#[cfg(target_family = "windows")]
fn main() {} // TODO add example
