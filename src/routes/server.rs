use hyper::{Request, Body, Method};

use crate::helpers::request::HyperClient;
// use crate::app::client::AppClient;
use crate::{helpers::response::ApiResponse};
use crate::controllers::{not_found, authorize_bot, health_check, handle_redirect};



pub async fn routes(
    req: Request<Body>, 
    client: HyperClient
) -> ApiResponse {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => health_check(),
        (&Method::GET, "/enable") => authorize_bot(&client).await,
        (&Method::GET, "/twitter/oauth") => handle_redirect(req, &client).await,
        _ => {
            not_found()
        }
    }
}