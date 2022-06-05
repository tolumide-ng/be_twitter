use hyper::{Client, client::HttpConnector};
use hyper_tls::HttpsConnector;


pub type HyperClient = Client<HttpsConnector<HttpConnector>>;


pub fn req_query<'a>(query: Option<&str>, key: &'a str) -> Option<String> {

    if let Some(str_query) = query {
        let q = str_query.split("&").collect::<Vec<_>>()
            .iter().map(|q| q.to_string()).collect::<Vec<String>>()
            .iter().find(|q| {q.split("=").collect::<Vec<&str>>().contains(&key)})
            .map(|x| x.split("=").collect::<Vec<_>>()[1].to_string());

        return q;
    }
    
    None
}