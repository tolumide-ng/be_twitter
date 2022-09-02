mod test_scope {
    use crate::helpers::scope::Scope;

    #[test]
    fn string_format_of_each_enum() {
        assert_eq!(Scope::ReadTweet.to_string(), "tweet.read");
        assert_eq!(Scope::WriteTweet.to_string(), "tweet.write");
        assert_eq!(Scope::ReadUsers.to_string(), "users.read");
        assert_eq!(Scope::WriteFollows.to_string(), "follows.write");
        assert_eq!(Scope::ReadFollows.to_string(), "follows.read");
        assert_eq!(Scope::OfflineAccess.to_string(), "offline.access");
        assert_eq!(Scope::WriteLike.to_string(), "like.write");
        assert_eq!(Scope::ReadLike.to_string(), "like.read");
    }


    #[test]
    fn with_scopes_creates_url_containing_to_string_of_scope() {
        let scopes = Scope::with_scopes(vec![Scope::ReadFollows, Scope::OfflineAccess, Scope::ReadTweet]);

        assert!(scopes.contains("follows.read"));
        assert!(scopes.contains("tweet.read"));
        assert!(!scopes.contains("follows.write"));
    }
}