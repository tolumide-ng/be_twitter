use core::fmt;

pub enum Scope {
    ReadTweet,
    WriteTweet,
    ReadUsers,
    WriteUsers,
    WriteFollows,
    ReadFollows,
    OfflineAccess,
    WriteLike,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let scope_str = match self {
            Self::ReadTweet => String::from("tweet.read"),
            Self::WriteTweet => String::from("tweet.write"),
            Self::ReadUsers => String::from("users.read"),
            Self::WriteUsers => String::from("users.write"),
            Self::ReadFollows => String::from("follows.read"),
            Self::WriteFollows => String::from("follows.write"),
            Self::OfflineAccess => String::from("offline.access"),
            Self::WriteLike => String::from("like.write"),
        };

        write!(f, "{}", scope_str)
    }
}


impl Scope {
    pub fn with_scopes(all_scopes: Vec<Self>) -> String {
        let space_separated_scopes = all_scopes.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>().join(" ");

        urlencoding::encode(&space_separated_scopes).to_string()
    }
}