use hyper::{Request, Body, Method};
use redis::{Client as RedisClient};

use crate::helpers::request::HyperClient;
use crate::helpers::response::{ApiBody};
// use crate::app::client::AppClient;
use crate::{helpers::response::TResult};
use crate::controllers::{not_found, authorize_bot, 
    health_check, handle_redirect, revoke_token, refresh_token, user_lookup, 
    get_timeline, handle_delete, request_token
};



pub async fn routes(
    req: Request<Body>, 
    client: HyperClient,
    conn: RedisClient
) -> TResult<ApiBody> {
    // migrate this to [routerify](https://docs.rs/routerify/latest/routerify/) eventually
    match (req.method(), req.uri().path(), req.uri().query()) {
        (&Method::GET, "/", _) => health_check(),
        (&Method::GET, "/enable", _) => authorize_bot(client, conn).await,
        (&Method::GET, "/oauth/callback", x) => handle_redirect(req, client, conn).await,
        (&Method::POST, "/revoke", _) => revoke_token(req, client, conn).await,
        (&Method::GET, "/refresh", _) => refresh_token(req, client, conn).await,
        (&Method::GET, "/user", x) => user_lookup(req, client, conn).await,
        (&Method::GET, "/timeline", x) => get_timeline(req, client, conn).await,
        (&Method::POST, "/remove", _) => handle_delete(req, client, conn).await,
        (&Method::GET, "/oauth1/request", _) => request_token(req, client, conn).await,
        // (&Method::GET, "/oauth1/", _) => request_token(req, client, conn).await,
        _ => {
            not_found()
        }
    }
}