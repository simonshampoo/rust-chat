use server::Server;
#[path = "./server.rs"]
pub mod server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = Server::new(8080);
    server.run().await;
}
