use core::fmt;
use std::collections::HashMap;

// macro to initialize an hashmap with values
// macro_rules! hashmap {
//     ($( $key: expr => $val: expr), *) => {{
//         let mut map = ::std::collections::HashMap::new();
//         $( map.insert($key, $val); )*
//     }}
// }

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
        // let scope_dic: HashMap<Scope, String> = hashmap![
        //     Self::ReadTweet => String::from("tweet.read"),
        //     Self::WriteTweet => String::from("tweet.write"),
        //     // Self::ReadUsers => String::from("users.read"),
        //     // Self::WriteUsers => String::from("users.write"),
        //     // Self::ReadFollows => String::from("follows.read"),
        //     // Self::WriteFollows => String::from("follows.write"),
        //     Self::OfflineAccess => String::from("offline.access"),
        //     Self::WriteLike => String::from("like.write")
        //     ];
            
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
