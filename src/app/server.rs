use hyper::{Server, Client};
use hyper::service::{make_service_fn, service_fn};
use hyper_tls::HttpsConnector;
use std::{net::SocketAddr};
use dotenv::dotenv;

use crate::routes::server::routes;


type GenericError = hyper::Error;

pub async fn server() {
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let https = HttpsConnector::new();
    let pool = Client::builder().build::<_, hyper::Body>(https);
    let redis_client= redis::Client::open("redis://127.0.0.1/").expect("Redis connection failed");
    // let mut conn = redis_client.get_connection().expect("Failed to connect to redis");

    let service = make_service_fn(move|_| {
        let client = pool.clone();
        let redis = redis_client.clone();

        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                routes(req, client.to_owned(), redis.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
