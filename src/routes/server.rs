use hyper::{Request, Body, Response, Method};

use crate::{helpers::response::ApiResponse};
use crate::api::not_found;

#[derive(Debug, serde::Deserialize)]
struct ABody {
    ab: String,
    cd: String,
}

impl From<hyper::Body> for ABody {
    fn from(_: hyper::Body) -> Self {
        Self {
            ab: "".to_string(),
            cd: "".to_string()
        }
    }
}


pub async fn routes(req: Request<Body>) -> ApiResponse {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("".to_string()))),
        _ => {
            not_found()
        }
    }
}