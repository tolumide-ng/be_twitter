
#[derive(Debug, Clone, derive_more::Display, PartialEq)]
pub enum Scope {
    #[display(fmt = "tweet.read")]
    ReadTweet,
    #[display(fmt = "tweet.write")]
    WriteTweet,
    #[display(fmt = "users.read")]
    ReadUsers,
    #[display(fmt = "follows.write")]
    WriteFollows,
    #[display(fmt = "follows.read")]
    ReadFollows,
    #[display(fmt = "offline.access")]
    OfflineAccess,
    #[display(fmt = "like.write")]
    WriteLike,
    #[display(fmt = "like.read")]
    ReadLike,
}


impl Scope {
    pub fn with_scopes(all_scopes: Vec<Self>) -> String {
        let space_separated_scopes = all_scopes.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>().join(" ");

        urlencoding::encode(&space_separated_scopes).to_string()
    }
}