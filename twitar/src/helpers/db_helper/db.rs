#[cfg(test)]
#[path = "./db.test.rs"]
mod db_test;

#[derive(sqlx::Type)]
#[sqlx(type_name = "tweet_type", rename_all = "lowercase")]
#[derive(Clone, Debug, PartialEq, Copy, derive_more::Display)]
pub enum TweetType {
    #[display(fmt = "tweets")]
    Tweets,
    #[display(fmt = "rts")]
    Rts,
    #[display(fmt = "likes")]
    Likes,
}

impl TweetType {
    pub fn get_all_as_vec() -> Vec<TweetType> {
        vec![TweetType::Rts, TweetType::Tweets, TweetType::Likes]
    }
}

pub type TweetIds<'a> = Vec<Vec<&'a String>>;

#[derive(Debug, Clone)]
pub struct AllTweetIds<'a> {
    tweets: TweetIds<'a>,
    rts: TweetIds<'a>,
    likes: TweetIds<'a>,
}

impl<'a> AllTweetIds<'a> {
    pub fn new(tweets: TweetIds<'a>, rts: TweetIds<'a>, likes: TweetIds<'a>) -> Self {
        Self {tweets, rts, likes}
    }

    pub fn get_tweets(&self) -> &TweetIds<'a> {
        &self.tweets
    }

    pub fn get_likes(&self) -> &TweetIds<'a> {
        &self.likes
    }

    pub fn get_rts(&self) -> &TweetIds<'a>  {
        &self.rts
    }
}