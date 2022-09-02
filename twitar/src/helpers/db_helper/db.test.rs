#[cfg(test)]
mod test_db_helper {

    use crate::helpers::db_helper::{TweetType, AllTweetIds};


    #[test]
    fn tweet_types() {
        let tweet_types = TweetType::get_all_as_vec();

        assert_eq!(tweet_types.len(), 3);
        let expected_tweet_type_strs = vec![TweetType::Rts, TweetType::Tweets, TweetType::Likes];

        assert_eq!(tweet_types, expected_tweet_type_strs);
        assert_eq!(TweetType::Rts.to_string(), "rts");
        assert_eq!(TweetType::Tweets.to_string(), "tweets");
        assert_eq!(TweetType::Likes.to_string(), "likes");
    }

    #[test]
    fn all_tweet_ids_returns_content() {
        let all = vec![String::from("1"), String::from("2"), String::from("3"), 
            String::from("4"), String::from("5"), String::from("6")];
        let mut tweets = Vec::with_capacity(2);
        let mut likes = Vec::with_capacity(2);
        let mut rts = Vec::with_capacity(2);
        
        for i in 0..6 {
            if i < 2 {
                tweets.push(&all[i]);
            } else if i < 4 {
                likes.push(&all[i]);
            } else {
                rts.push(&all[i]);
            }
        }
        
        let all = AllTweetIds::new(vec![tweets.clone()], vec![rts.clone()], vec![likes.clone()]);

        let expected_likes = &all.get_likes()[0];
        let expected_rts = &all.get_rts()[0];
        let expected_tweets = &all.get_tweets()[0];

        assert_eq!(expected_likes, &likes);
        assert_eq!(expected_rts, &rts);
        assert_eq!(expected_tweets, &tweets);
    }
}