-- Add up migration script here

CREATE TABLE auth_two(
    id SERIAL PRIMARY KEY UNIQUE NOT NULL,
    user_id UUID UNIQUE NOT NULL references user_preference(user_id) ON DELETE CASCADE,
    twitter_user_id TEXT UNIQUE,
    pkce VARCHAR(128),
    access_token VARCHAR(128),
    refresh_token VARCHAR(128)
);
