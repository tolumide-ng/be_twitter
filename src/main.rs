use hyper::Client;
use hyper_tls::HttpsConnector;
use twitar::app::server::server;


// use twitar::setup::credentials::Credentials;

#[tokio::main]
async fn main() {
    let https = HttpsConnector::new();
    let pool = Client::builder().build::<_, hyper::Body>(https);

    server(pool).await
}
