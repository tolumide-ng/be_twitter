use secrecy::Secret;
use twitar::{helpers::hmac_signature::{AuthorizeRequest, Signature, ApiCallMethod}, app::server::server};

// use twitar::setup::credentials::Credentials;

#[tokio::main]
async fn main() {
    server().await
}
