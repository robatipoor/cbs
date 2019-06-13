use cbs::Action;
use tokio::prelude::*;

fn main() {
    let client = Action::Get.send().and_then(|resp| {
        println!("{:?}", resp);
        Ok(())
    }).map_err(|err| {
        println!("{:?}", err);
    });
    tokio::run(client);
}