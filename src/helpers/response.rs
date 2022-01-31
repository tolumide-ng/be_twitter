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
    
    let (parts, body) = res.into_parts();
    let body = hyper::body::to_bytes(body).await?.to_vec();

    println!("WHAT THE ERROR IS LIKE \n\n\n {:#?} \n\n\n", String::from_utf8_lossy(&body));
    
    if let Ok(errors) = serde_json::from_slice::<TwitterErrors>(&body) {
        println!("THE LOOPED ERROR SETS");
        if errors.errors.iter().any(|e| e.code == 88)
        && parts.headers.contains_key(X_RATE_LIMIT_RESET) {
            return Err(TError::RateLimit())
        } else {
            return Err(TError::TwitterError(parts.headers, errors))
        }
    }

    if !parts.status.is_success() {
        println!("IS THIS AN ERROR!!!???");
        // put the body in the logger
        let body = String::from_utf8_lossy(&body);
        return Err(TError::BadStatus(parts.status))
    }

    println!("THIS WAS A SUCCESS");


    
    Ok((parts.headers, body))
}
