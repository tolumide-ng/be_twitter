-- Add up migration script here

CREATE TABLE auth_one(
    -- user_id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
    id SERIAL PRIMARY KEY UNIQUE NOT NULL,
    user_id UUID UNIQUE NOT NULL references user_preference(user_id) ON DELETE CASCADE,
    twitter_user_id TEXT UNIQUE,
    oauth_token VARCHAR(128) NOT NULL,
    oauth_secret VARCHAR(128) NOT NULL,
    oauth_verifier VARCHAR(128) NOT NULL
);