use hyper::{Server, Client};
use hyper::service::{make_service_fn, service_fn};
use hyper_tls::HttpsConnector;
use std::{net::SocketAddr};
use dotenv::dotenv;
use redis::{Client as RedisClient};

use crate::helpers::request::HyperClient;
use crate::routes::server::routes;

type GenericError = hyper::Error;

pub async fn server() {
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let https = HttpsConnector::new();
    let hyper_pool = Client::builder().build::<_, hyper::Body>(https);
    let hyper_client = hyper_pool.clone();
    
    let redis_client= RedisClient::open("rediss://127.0.0.1/").expect("Redis connection failed");
    // let mgr = ConnectionManager::new(redis_client).await.unwrap();

    let service = make_service_fn(move|_| {
        let redis = redis_client.clone();
        let client = hyper_client.clone();

        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                routes(req, client.to_owned(),   redis.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
