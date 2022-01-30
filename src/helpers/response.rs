use http::{Request, HeaderMap, HeaderValue};
use hyper::{Response, Body};
use serde::{Serialize, Deserialize};

use crate::errors::response::{TError, TwitterErrors};
use crate::helpers::request::HyperClient;

// pub type ApiResponse = http::Result<Response<Body>>;

// pub type ApiResponse<T> = Result<T, anyhow::Error>;
pub type TResult<T> = std::result::Result<T, TError>;
pub type THeaders = HeaderMap<HeaderValue>;
pub type ApiBody = Response<Body>;


const X_RATE_LIMIT_RESET: &str = "X-Rate-Limit-Reset";


#[derive(Serialize, Deserialize)]
pub struct ApiResponseBody<T> {
    message: String,
    body: Option<T>
}


impl<T: Serialize> ApiResponseBody<T> {
    pub fn new(message: String, body: Option<T>) -> String {
        let response= Self {
            message,
            body,
        };

        let res= serde_json::to_string(&response).unwrap();
        res
    }
}


pub async fn make_request(request: Request<Body>, client: HyperClient) -> TResult<(THeaders, Vec<u8>)> {
    let res = client.request(request).await.unwrap();
    
    println!("THE RESPONSE {:#?}", res);

    let (parts, body) = res.into_parts();
    println!(":::::::::::::::::::::::::::::::::::::::::::::");
    let body = hyper::body::to_bytes(body).await?.to_vec();

    // println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {:#?}", body);
    
    if let Ok(errors) = serde_json::from_slice::<TwitterErrors>(&body) {
        println!("there was an error!!!!!!!!!!!!!!! {:#?}", errors);

        if errors.errors.iter().any(|e| e.code == 88)
        && parts.headers.contains_key(X_RATE_LIMIT_RESET) {
            println!("CONTENT OF THE PARTS HEADER {:#?}", parts);
            return Err(TError::RateLimit())
        } else {
            return Err(TError::TwitterError(parts.headers, errors))
        }
    }

    println!("<<<<<<<<<<<<<IT SHOULD BE HERE>>>>>>>>>>>");
    if !parts.status.is_success() {
        // println!("WHAT'S IS THE THE BODY {:#?}", body);
        let s = String::from_utf8_lossy(&body);
        println!("result: {}", s);
        return Err(TError::BadStatus(parts.status))
    }

    println!("THE ACTUAL BODY>>>>>>>>>>>> {:#?}", body);
    
    Ok((parts.headers, body))
}