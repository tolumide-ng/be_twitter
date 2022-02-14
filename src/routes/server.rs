use hyper::{Request, Body, Method};

use crate::app::server::AppState;
use crate::helpers::response::{ApiBody};
use crate::{helpers::response::TResult};
use crate::controllers::{not_found, authorize_bot, 
    health_check, handle_redirect, revoke_token, refresh_token, user_lookup, 
    get_timeline, handle_delete, request_token
};



pub async fn routes(
    state: AppState
) -> TResult<ApiBody> {
    // migrate this to [routerify](https://docs.rs/routerify/latest/routerify/) eventually
    let req = &state.req;

    match (req.method(), req.uri().path(), req.uri().query()) {
        (&Method::GET, "/", _) => health_check().await,
        (&Method::GET, "/enable", _) => authorize_bot(state).await,
        (&Method::GET, "/oauth/callback", x) => handle_redirect(state).await,
        (&Method::POST, "/revoke", _) => revoke_token(state).await,
        (&Method::GET, "/refresh", _) => refresh_token(state).await,
        (&Method::GET, "/user", x) => user_lookup(state).await,
        (&Method::GET, "/timeline", x) => get_timeline(state).await,
        (&Method::POST, "/remove", _) => handle_delete(state).await,
        (&Method::GET, "/oauth1/request", _) => request_token(state).await,
        (&Method::GET, "/oauth1/", _) => request_token(state).await,
        _ => {
            not_found(state).await
        }
    }
}