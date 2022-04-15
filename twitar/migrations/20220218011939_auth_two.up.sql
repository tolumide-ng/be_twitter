-- Add up migration script here

CREATE TABLE auth_two(
    -- user_id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
    user_id UUID PRIMARY KEY UNIQUE NOT NULL,
    twitter_user_id TEXT UNIQUE,
    pkce VARCHAR(128),
    access_token VARCHAR(128),
    refresh_token VARCHAR(128)
);
