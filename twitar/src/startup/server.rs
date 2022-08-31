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

use crate::base_repository::db::{AuthUser, V1User, V2User};
use crate::configurations::db_settings::DatabaseSettings;
use crate::helpers::request::HyperClient;
use crate::routes::server::Routes;
use crate::configurations::variables::SettingsVars;
use crate::settings::config;

// use super::timeout::TimeoutLayer;

type GenericError = hyper::Error;

pub enum User {
    AuthUser(AuthUser),
    V1User(),
    V2User(V2User),
    // All(AuthUser, V2User)
}

#[derive(Debug)]
pub struct CurrentUser {
    pub basic: AuthUser,
    pub v2_user: V2User,
    pub v1_user: V1User,
}

impl CurrentUser {
    pub fn new(basic: AuthUser, v1_user: V1User, v2_user: V2User) -> Self {
        Self {
            basic,
            v1_user,
            v2_user,
        }
    }
}


#[derive(Debug)]
pub struct AppState {
    pub redis: RedisClient,
    pub db_pool: Pool<Postgres>,
    pub hyper: HyperClient,
    pub req: Request<Body>,
    pub env_vars: SettingsVars,
    pub user: Option<CurrentUser>,
}

impl AppState {
    fn new(env_vars: SettingsVars, req: Request<Body>, hyper: HyperClient, redis: RedisClient, db_pool: Pool<Postgres>) -> Self {
        Self { redis, hyper, req, env_vars, db_pool, user: None}
    }

    pub fn with_user(&mut self, user: CurrentUser) {
        self.user = Some(user);
    }

    pub fn add_user(state: Self, user: CurrentUser) -> Self {
        Self { user: Some(user), ..state }
    }
}


pub async fn server() {
    dotenv().ok();
    let app_config = config::get_configuration();

    match app_config {
        Ok(config) => {
            println!("THE OBTAINED SETTINGS BECAUSE WE FOUND SOMETHING USEFUL {:#?}", config);
        }
        Err(e) => {
            println!("THERE'S A LOT OF THINGS WRONG BUT FIRST LET'S START HERE!!!!!!!!!!!!!!!!!!!!!! {:#?} ", e);
        }
    }
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
                
                Routes::wrapper(state)
            });
        
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
    PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}