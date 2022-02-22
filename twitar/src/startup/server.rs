use http::Request;
use hyper::{Server, Client, Body};
use hyper::service::{make_service_fn, service_fn};
use hyper_tls::HttpsConnector;
use sqlx::{PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use std::time::Duration;
use std::{net::SocketAddr};
use dotenv::dotenv;
use redis::{Client as RedisClient};

use crate::configurations::db_settings::DatabaseSettings;
use crate::helpers::commons::AppEnv;
use crate::helpers::request::HyperClient;
use crate::routes::server::routes;
use crate::configurations::variables::SettingsVars;

// use super::timeout::TimeoutLayer;

type GenericError = hyper::Error;


#[derive(Debug)]
pub struct AppState {
    pub redis: RedisClient,
    pub hyper: HyperClient,
    pub req: Request<Body>,
    pub env_vars: SettingsVars,
    pub db_pool: Pool<Postgres>,
}

#[derive(Debug, Clone)]
pub struct LocalAppState {
    pub redis: RedisClient,
    pub app_env: AppEnv,
    pub db_pool: Pool<Postgres>,
}

impl AppState {
    fn new(env_vars: SettingsVars, req: Request<Body>, hyper: HyperClient, redis: RedisClient, db_pool: Pool<Postgres>) -> Self {
        Self { redis, hyper, req, env_vars, db_pool}
    }

    pub fn to_local(&self) -> LocalAppState {
        let app_env = self.env_vars.app_env.clone();
        LocalAppState { redis: self.redis.clone(), app_env: app_env, db_pool: self.db_pool.clone() }
    }
}


pub async fn server() {
    dotenv().ok();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let https = HttpsConnector::new();
    let hyper_pool = Client::builder().build::<_, hyper::Body>(https);
    let hyper_client = hyper_pool.clone();
    let redis_client= RedisClient::open("redis://127.0.0.1/").expect("Redis connection failed");
    let env_vars = SettingsVars::new();
    let db_pool = get_pool(DatabaseSettings::new(env_vars.clone()));

    
    let service = make_service_fn(move|_| {
        let redis = redis_client.clone();
        let client = hyper_client.clone();
        let vars = env_vars.clone();
        let db_pool = db_pool.clone();

        let svc= service_fn(move |req| {
                let state = AppState::new(vars.clone(), 
                    req, client.to_owned(), redis.to_owned(), db_pool.to_owned());
                routes(state)
            });

        // let svc = ServiceBuilder::new()
        //     .layer(TimeoutLayer::new(Duration::new(60, 0)))
        //     .service(svc).inner;
        
        let svc = ServiceBuilder::new()
            .timeout(Duration::new(45, 0))
            .service(svc);

        async {
            Ok::<_, GenericError>(svc)
        }
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}


pub fn get_pool(config: DatabaseSettings) -> PgPool {
    println!("THE CONFIG WITH DB {:#?}", config.with_db());
    PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}