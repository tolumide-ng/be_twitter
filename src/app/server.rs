use std::{env, net::SocketAddr, convert::Infallible};

use futures::{future, Future, Stream};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};


async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World")))
}

pub async fn server() {
    let addr: SocketAddr = match env::var("APP_ADDRESS") {
        Ok(value) => value.parse().unwrap(),
        Err(_) => panic!("Env Variable: APP_ADDRESS is required"),
    };

    // A MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}