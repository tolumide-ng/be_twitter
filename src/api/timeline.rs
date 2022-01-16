struct Timeline<T> {
    user_tweets: Vec<T>,
    max_tweets: i16,
    header: HeadersOptions,
}

struct HeadersOptions {
    user_gent: String,
    authorization: String
}

impl<T> Timeline<T> {
    pub fn get_user_tweets() {

        // const params = 
    }
}