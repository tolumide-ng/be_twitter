use std::borrow::Cow;

pub struct SignedHeader {
    pub params: Vec<(&'static str, Cow<'static, str>)>
}


impl SignedHeader {
    fn get_header(&self) -> String {
        let oauth_str = self.params
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join(", ");

        format!("OAuth {}", oauth_str)
    }
}