// mod test_response {
//     use std::collections::HashMap;

//     use wiremock::{MockServer, Mock, ResponseTemplate};
//     use wiremock::matchers::{method, path};
//     use http::{Request, Response};
//     use hyper::{Body, Client, };
//     use hyper_tls::HttpsConnector;
//     use serde_json;

//     use crate::helpers::response::make_request;



//     #[tokio::test]
//     async fn should_resolve_request() {
//         let mock_response = ResponseTemplate::new(200).set_body_string("Mock Body of the response");
        
//         let mock_server = MockServer::start().await;
//         Mock::given(method("GET"))
//             .and(path("/api"))
//             .respond_with(mock_response)
//             .expect(1..)
//             .mount(&mock_server).await;
 
//         let req = Request::builder()
//             .method("GET")
//             .uri("api?key=value")
//             .body(Body::empty()).unwrap();


//         let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

//         let request = make_request(req, client).await;

//         assert!(request.is_ok());
//     }

//     #[tokio::test]
//     async fn should_fail_with_rate_limit_error() {
//         let errors = serde_json::json!({
//             "errors": "This should be a normal error",
//         });

//         let mock_response = ResponseTemplate::new(400).set_body_json(errors);
        
//         let mock_server = MockServer::start().await;
//         Mock::given(method("GET"))
//             .and(path("/api"))
//             .respond_with(mock_response)
//             .expect(1..)
//             .mount(&mock_server).await;
 
//         let req = Request::builder()
//             .method("GET")
//             .uri("api?key=value")
//             .body(Body::empty()).unwrap();


//         let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

//         let request = make_request(req, client).await;

//         assert!(request.is_err());
//     }
// }