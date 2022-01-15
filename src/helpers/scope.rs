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
        write!(f, "{}", Scope::generate(self))
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

    fn required_user_scope() -> Vec<String> {
        vec![
            Scope::ReadTweet.to_string(), 
            Scope::WriteTweet.to_string(),
            Scope::OfflineAccess.to_string(),
            Scope::WriteLike.to_string(),
        ]
    }

    pub fn new(auth_type: AuthType) -> String {
        match auth_type {
            AuthType::User => {
                let required_scope = Self::required_user_scope();
                required_scope.join(" ")
            }
            AuthType::App => {"".to_string()}
        }
    }
}
