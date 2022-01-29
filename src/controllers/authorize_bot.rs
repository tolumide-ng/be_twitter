use hyper::{Method, Body, Response};
use redis::{AsyncCommands};
use redis::{Client as RedisClient};

use crate::helpers::response::ApiBody;
use crate::helpers::{
    response::{ApiResponse},
    request::{HyperClient},
    gen_pkce::Pkce,
    scope::Scope,
    keyval::KeyVal,
};
use crate::setup::{variables::SettingsVars};
use crate::middlewares::request_builder::RequestBuilder;


pub async fn authorize_bot(req: &HyperClient, client: RedisClient) -> ApiResponse<ApiBody> {
    let SettingsVars {client_id, redirect_uri, state, ..} = SettingsVars::new();
    // store this pkce value in redis for the specific user associated by email
    let mut connection = client.get_async_connection().await.unwrap();
    connection.set("test", &client_id).await?;

    let pkce = Pkce::new().to_string();
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

    // println!("THE QUERY PARAMS {:#?}", query_params);

    let request = RequestBuilder::new(Method::GET, "https://twitter.com/i/oauth2/authorize")
        .add_query_params(query_params)
        .request_no_keys();

    let response_body= Response::builder().status(302)
        .header("Location", request.uri().to_string())
        .body(Body::from(request.uri().to_string())).unwrap();

    Ok(response_body)
}