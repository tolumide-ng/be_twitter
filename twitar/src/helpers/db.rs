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