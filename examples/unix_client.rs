
extern crate cbs;

use cbs::Action;

#[cfg(target_family = "unix")]
fn main() {
    use tokio::prelude::*;
    
    let client = Action::Get
        .send()
        .and_then(|resp| {
            if resp.content.is_some() {
                println!("{}", resp.content.unwrap());
            }
            Ok(())
        })
        .map_err(|err| {
            eprintln!("error {:?}", err);
        });
    tokio::run(client);
}
#[cfg(target_family = "windows")]
fn main() {}