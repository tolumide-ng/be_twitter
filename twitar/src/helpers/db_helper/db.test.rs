#[cfg(test)]
mod test_db_helper {
    use crate::helpers::db_helper::db::TweetType;


    #[test]
    fn tweet_types() {
        let tweet_types = TweetType::get_all_as_vec();

        assert_eq!(tweet_types.len(), 3);
        let expected_tweet_type_strs = vec!["tweets", "rts", "likes"];
    }
}