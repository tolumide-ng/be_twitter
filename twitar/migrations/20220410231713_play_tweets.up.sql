-- Add up migration script here
CREATE TYPE tweet_type AS ENUM ('rts', 'likes', 'tweets');

CREATE TABLE play_tweets(
    id BIGSERIAL PRIMARY KEY,
    user_id UUID references auth_two(user_id),
    tweet_type tweet_type,
    tweet_ids text[]
)