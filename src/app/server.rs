use hyper::{Server, Client};
use hyper_tls::HttpsConnector;
use routerify::RouterService;
use std::{net::SocketAddr};
use dotenv::dotenv;
use redis::{Client as RedisClient};

use crate::helpers::request::HyperClient;
use crate::routes::server::{router};
use crate::setup::variables::SettingsVars;

type GenericError = hyper::Error;


pub struct AppState {
    pub redis: RedisClient,
    pub hyper: HyperClient,
    // pub req: Request<Body>,
    pub env_vars: SettingsVars,
}

impl AppState {
    fn new(hyper: HyperClient, redis: RedisClient) -> Self {
        // is this expensive? Should this rather be done at the point of intializing the hyper_client e.t.c, and then implement
        // clone (Iterator trait) for settingsVars?
        let env_vars = SettingsVars::new();
        Self { redis, hyper, env_vars}
    }
}


pub async fn server() {
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let https = HttpsConnector::new();
    let hyper_pool = Client::builder().build::<_, hyper::Body>(https);
    // let hyper_client = hyper_pool.clone();
    
    let redis_client= RedisClient::open("redis://127.0.0.1/").expect("Redis connection failed");
    // let hyper_client = redis_client.get_async_connection().await.unwrap();
    // let mgr = ConnectionManager::new(redis_client).await.unwrap();
    
    // let service = make_service_fn(move|_| {
    //     let redis = redis_client.clone();
    //     let client = hyper_client.clone();
        
    //     async {
    //         Ok::<_, GenericError>(service_fn(move |req| {
    //             let state = AppState::new(req, client.to_owned(), redis.to_owned());
    //             routes(state)
    //         }))
    //     }
    // });

    // let service = make_service_fn(move|_| {
    //     let redis = redis_client.clone();
    //     let client = hyper_client.clone();

    //     async {
    //         Ok::<_, GenericError>(service_fn(move |req| {
    //             routes(req, client.to_owned(),   redis.to_owned())
    //         }))
    //     }
    // });

    let env_vars = SettingsVars::new();

    let state = AppState::new(hyper_pool.to_owned(), redis_client.to_owned());

    let router = router(state);

    let service = RouterService::new(router).unwrap();

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
