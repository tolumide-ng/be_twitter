use secrecy::Secret;
use twitar::app::server::server;


// use twitar::setup::credentials::Credentials;

#[tokio::main]
async fn main() {
    server().await
}
