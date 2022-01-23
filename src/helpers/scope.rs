use core::fmt;

pub enum AuthType {
    User,
    App,
}

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
            Self::WriteLike => String::from("like.write")
        };

        write!(f, "{}", scope_str)
    }
}
