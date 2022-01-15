use url::Url;
use crate::helpers::gen_pkce::Pkce;

enum Scope {
    ReadTweet,
    WriteTweet,
    // ReadUsers,
    // WriteUsers,
    // WriteFollows,
    // ReadFollows,
    OfflineAccess,
    WriteLike,
}


impl Scope {
    pub fn generate(permission: Self) -> String {
        return match permission {
            Self::ReadTweet => String::from("tweet.read"),
            Self::WriteTweet => String::from("tweet.write"),
            // Self::ReadUsers => String::from("users.read"),
            // Self::WriteUsers => String::from("users.write"),
            // Self::ReadFollows => String::from("follows.read"),
            // Self::WriteFollows => String::from("follows.write"),
            Self::OfflineAccess => String::from("offline.access"),
            Self::WriteLike => String::from("like.write")
        }
    }
}


struct App {
    scopes: String,
    code_challenge: Pkce,
}


impl App {
    pub fn new() -> Self {
        Self {
            scopes: String::from("tweet.read%20users.read%20follows.read%20follows.write"),
            code_challenge: Pkce::new(),
        }
    }
}