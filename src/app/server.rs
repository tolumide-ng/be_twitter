use futures::{future, Future, Stream};
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};
use std::{env, net::SocketAddr};

use crate::routes::server::routes;


pub async fn server() {
    let addr: SocketAddr = match env::var("APP_ADDRESS") {
        Ok(value) => value.parse().unwrap(),
        Err(_) => panic!("Env Variable: APP_ADDRESS is required"),
    };

    let service = make_service_fn(|_| async {Ok::<_, hyper::Error>(service_fn(routes))});

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
