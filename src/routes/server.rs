use hyper::{Request, Body, Method};
use redis::{Client as RedisClient};

use crate::helpers::request::HyperClient;
use crate::helpers::response::ApiBody;
// use crate::app::client::AppClient;
use crate::{helpers::response::TResult};
use crate::controllers::{not_found, authorize_bot, health_check, handle_redirect};



pub async fn routes(
    req: Request<Body>, 
    client: HyperClient,
    conn: RedisClient
) -> TResult<ApiBody> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => health_check(),
        (&Method::GET, "/enable") => authorize_bot(client, conn).await,
        (&Method::GET, "/twitter/oauth") => handle_redirect(req, client, conn).await,
        // (&Method::GET)
        _ => {
            not_found()
        }
    }
}