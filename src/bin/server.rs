use cbs::server::Server;

#[tokio::main]
pub async fn main() -> cbs::Result<()> {
    env_logger::init();
    let server = Server::new("127.0.0.1:8080".to_owned());
    server.run().await
}
