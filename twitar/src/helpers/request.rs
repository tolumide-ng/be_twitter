use hyper::{Client, client::HttpConnector};
use hyper_tls::HttpsConnector;


pub type HyperClient = Client<HttpsConnector<HttpConnector>>;