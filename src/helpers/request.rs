use std::{collections::HashMap};
use hyper::Uri;
use url::Url;


pub fn get_param_from_uri(uri: &Uri) -> Option<HashMap<String, String>> {
    let mut dic = HashMap::new();
    let mut uri_string = uri.to_string();

    if !uri_string.starts_with("https:/") {
        uri_string = format!("https:/{}", uri_string);
    }

    let parsed_uri = match Url::parse(&uri_string) {
        Ok(uri) => uri,
        Err(e) => {
            panic!("There was an error parsing the uri {}", e);
        }
    };


    if let Some(all_qs) = parsed_uri.query() {
        let params: Vec<&str> = all_qs.split("&").collect();
        

        for param in params {
            let vec_param = param.split("=").collect::<Vec<_>>();
            dic.insert(vec_param[0].to_string(), vec_param[1].to_string());
        }

        return Some(dic)
    }

    None
}
