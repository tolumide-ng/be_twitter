use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use std::{net::SocketAddr};
use dotenv::dotenv;

use crate::helpers::request::HyperClient;
use crate::routes::server::routes;


type GenericError = hyper::Error;

pub async fn server(client: HyperClient) {
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let service = make_service_fn(move|_| {
        let client = client.clone();

        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                routes(req, client.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
