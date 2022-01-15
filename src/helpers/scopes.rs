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
        write!(f, "({})", Scope::generate(self))
    }
}



impl Scope {    
    fn generate(permission: &Self) -> String{            
        return match *permission {
            Self::ReadTweet => String::from("tweet.read"),
            Self::WriteTweet => String::from("tweet.write"),
            Self::ReadUsers => String::from("users.read"),
            Self::WriteUsers => String::from("users.write"),
            Self::ReadFollows => String::from("follows.read"),
            Self::WriteFollows => String::from("follows.write"),
            Self::OfflineAccess => String::from("offline.access"),
            Self::WriteLike => String::from("like.write")
        }
    }

    pub fn get_required_scopes() -> Vec<String> {
        vec![
            Scope::ReadTweet.to_string(), 
            Scope::WriteTweet.to_string(),
            Scope::OfflineAccess.to_string(),
            Scope::WriteLike.to_string(),
        ]
    }
}
