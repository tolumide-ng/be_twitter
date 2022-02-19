use twitar::startup::server::server;


#[tokio::main]
async fn main() {
    server().await;
}
