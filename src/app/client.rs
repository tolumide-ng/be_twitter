use hyper::{Client, client::{HttpConnector}, Method, Request, Body};
use hyper_tls::HttpsConnector;
use hyper::{Response};


use crate::{middlewares::{
    request_builder::RequestBuilder,
}, setup::variables::SettingsVars, helpers::keyval::KeyVal};
use crate::helpers::{
    scope::Scope,
    gen_pkce::Pkce,
};


#[derive(Debug)]
pub struct AppClient {
    pool: Client<HttpsConnector<HttpConnector>>,
    // signature: Signature,
}


impl AppClient {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let pool = Client::builder().build::<_, hyper::Body>(https);
        
        Self {
            pool,
            // signature: signature.clone(),
        }

    }


    pub async fn make_call(&self, request: Request<Body>) {
        let client = &self.pool.clone();
        // need to write an error converter for this
        let res = client.request(request).await.unwrap();

        println!("THE RESPONSE OR WHATEVER {:#?}", res);
        let (parts, body) = res.into_parts();
        let body: Vec<_> = hyper::body::to_bytes(body).await.unwrap().to_vec();

        let body = std::str::from_utf8(&body).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "stream did not contain valid UTF-8",
            )
        }).unwrap();

    }

    // pub async fn get_request_token<S: Clone + fmt::Display + Into<String>>(&self, callback: S, consumer: KeyPair) {

    //     let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/oauth/request_token")
    //         .with_oauth_callback(callback.clone().into())
    //         .with_query(KeyPair::new("oauth_callback", encode(&callback.clone().into()).into_owned()))
    //         // .with_query(KeyPair::new("oauth_callback", c.into()))
    //         .request_keys(consumer, None);

    //     self.make_call(request).await;
    // }


    pub async fn oauth2_authorize(&self) -> Response<Body> {

        let SettingsVars {client_id, redirect_uri, state, ..} = SettingsVars::new();
        // store this pkce value in redis for the specific user associated by email
        let pkce = Pkce::new().to_string();
        // let client_id = client_id;
        let scopes = vec![Scope::ReadTweet, Scope::ReadUsers, Scope::ReadFollows, Scope::WriteFollows, 
            Scope::OfflineAccess, Scope::WriteTweet, Scope::WriteLike];

        let query_params = KeyVal::new()
            .add_list_keyval(vec![
                ("response_type".to_string(), "code".to_string()),
                ("client_id".to_string(), client_id),
                ("redirect_uri".to_string(), redirect_uri),
                ("scope".to_string(), Scope::with_scopes(scopes)),
                ("state".to_string(), state),
                ("code_challenge".to_string(), pkce),
                ("code_challenge_method".to_string(), "plain".to_string()),
            ]);

        let request = RequestBuilder::new(Method::GET, "https://twitter.com/i/oauth2/authorize")
            .add_query_params(query_params)
            .request_no_keys();

        self.redirect_user(request).await
    }

    pub async fn redirect_user(&self, request: Request<Body>) -> Response<Body> {
           Response::builder()
            .status(302)
            .header("Location", request.uri().to_string())
            .body(Body::from(request.uri().to_string()))
            .unwrap()
    }

    // this method won't be public after we refactor the controllers to be methods in just one Struct as is currently in this module
    pub async fn access_token(&self, auth_code: String) {
        let SettingsVars{client_id, redirect_uri, ..} = SettingsVars::new();
        let req_body = KeyVal::new().add_list_keyval(vec![
            ("grant_type".to_string(), auth_code),
            ("client_id".to_string(), client_id),
            ("redirect_uri".to_string(), redirect_uri),
            ("code_verifier".to_string(), "challenge".to_string())
        ]);

        // let request = RequestBuilder::new(Method::POST, "https://twitter.com/i/oauth2/token");
    }
}
