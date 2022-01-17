// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/creating-a-signature

use urlencoding::encode;
use std::collections::HashMap;


struct Signature {}


// to be removed later -- for practise only
struct AuthorizeRequest {
    include_entities: String,
    oauth_consumer_key: String,
    oauth_nonce: String,
    oauth_signature_method: String,
    oauth_timestamp: String,
    oauth_token: String,
    oauth_version: String,
}



impl Signature {
    fn create_signature(request: AuthorizeRequest) {
        let mut request_params: HashMap<&str, String> = HashMap::new();
        let AuthorizeRequest {include_entities, oauth_consumer_key, oauth_nonce,
            oauth_signature_method, oauth_timestamp, oauth_token, oauth_version, ..
        } = request;

        request_params.insert("include_entities", include_entities);
        request_params.insert("oauth_consumer_key", oauth_consumer_key);
        request_params.insert("oauth_nonce", oauth_nonce);
        request_params.insert("oauth_signature_method", oauth_signature_method);
        request_params.insert("oauth_timestamp", oauth_timestamp);
        request_params.insert("oauth_token", oauth_token);
        request_params.insert("oauth_version", oauth_version);


        let mut encoded_r_params = vec![];

        for (key, val) in request_params.iter() {
            encoded_r_params.push((encode(key), encode(val)))
        }

        encoded_r_params.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut parameter_string = String::from("");

        encoded_r_params.iter().enumerate().for_each(|(index,val)| {
            let mut key_val = format!("{}={}", val.0, val.1);
            
            if index == encoded_r_params.len() - 1{
                key_val.push_str("&");
            }

            parameter_string.push_str(key_val.as_str());
        });
    }
}