-- Add up migration script here
CREATE TYPE tweet_type AS ENUM ('rts', 'likes', 'tweets');

CREATE TABLE play_tweets(
    id SERIAL PRIMARY KEY,
    user UUID references auth_two(user_id),
    content tweet_type,
    tweet_ids text[]
)