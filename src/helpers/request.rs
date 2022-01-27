use std::{collections::HashMap};
use hyper::Uri;
use url::Url;


pub fn get_param_from_uri(uri: &Uri, query: &str) -> Option<String> {
    let uri_string = uri.to_string();
    let parsed_uri = Url::parse(&uri_string).unwrap();

    if let Some(all_qs) = parsed_uri.query() {
        let params: Vec<&str> = all_qs.split("&").collect();
        let mut dic = HashMap::new();

        for param in params {
            let vec_param = param.split("=").collect::<Vec<_>>();
            dic.insert(vec_param[0], vec_param[1]);
        }

        return match &dic.get(&query) {
            Some(v) => Some(v.to_string()),
            None => None
        }
    }

    None
}